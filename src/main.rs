extern crate nalgebra_glm as glm;

use std::sync::{Arc, Mutex, RwLock};
use std::thread;
use std::{mem, os::raw::c_void, ptr};

pub use self::camera::Camera;

mod camera;
mod colors;
mod shader;
mod shapes;
mod tasks;
mod util;

use glutin::event::{
    DeviceEvent,
    ElementState::{Pressed, Released},
    Event, KeyboardInput,
    VirtualKeyCode::{self, *},
    WindowEvent,
};

use glutin::event_loop::ControlFlow;

const SCREEN_W: u32 = 800;
const SCREEN_H: u32 = 600;

// == // Helper functions to make interacting with OpenGL a little bit prettier. You *WILL* need these! // == //
// The names should be pretty self explanatory

fn byte_size_of_array<T>(val: &[T]) -> isize {
    std::mem::size_of_val(&val[..]) as isize
}

/// Get the OpenGL-compatible pointer to an arbitrary array of numbers
fn pointer_to_array<T>(val: &[T]) -> *const c_void {
    &val[0] as *const T as *const c_void
}

/// Get the size of the given type in bytes
fn size_of<T>() -> i32 {
    mem::size_of::<T>() as i32
}

/// Get an offset in bytes for n units of type T
fn offset<T>(n: u32) -> *const c_void {
    (n * mem::size_of::<T>() as u32) as *const T as *const c_void
}

// Get a null pointer (equivalent to an offset of 0)
// ptr::null()

/// Makes a new buffer and fills it with the given data values. Leaves the created
/// buffer bound
unsafe fn make_buffer<T>(target: gl::types::GLenum, values: &Vec<T>) -> u32 {
    let mut buffer_id = 0u32;
    // Make buffer
    gl::GenBuffers(1, &mut buffer_id as *mut u32);

    // Fill buffer with provided data
    gl::BindBuffer(target, buffer_id);
    gl::BufferData(
        target,
        byte_size_of_array(values),
        pointer_to_array(values),
        gl::STATIC_DRAW,
    );

    buffer_id
}

/// Makes a new VAO, feeds the vertices to a new VBO for said VAO and makes an index buffer
/// with the given indices
unsafe fn make_vao(vertices: &Vec<f32>, indices: &Vec<u32>, colors: &Vec<f32>) -> u32 {
    let mut id = 0u32;
    // Make and bind VAO
    gl::GenVertexArrays(1, &mut id as *mut u32);
    gl::BindVertexArray(id);

    // Make and fill buffer. This function leaves the created VBO bound
    make_buffer(gl::ARRAY_BUFFER, vertices);
    gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 0, ptr::null());

    make_buffer(gl::ARRAY_BUFFER, colors);
    gl::VertexAttribPointer(1, 4, gl::FLOAT, gl::FALSE, 0, ptr::null());

    gl::EnableVertexAttribArray(0);
    gl::EnableVertexAttribArray(1);

    make_buffer(gl::ELEMENT_ARRAY_BUFFER, indices);

    id
}

fn main() {
    // Set up the necessary objects to deal with windows and event handling
    let el = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new()
        .with_title("Gloom-rs")
        .with_resizable(false)
        .with_inner_size(glutin::dpi::LogicalSize::new(SCREEN_W, SCREEN_H));
    let cb = glutin::ContextBuilder::new().with_vsync(true);
    let windowed_context = cb.build_windowed(wb, &el).unwrap();

    windowed_context
        .window()
        .set_cursor_grab(true)
        .expect("failed to grab cursor");
    windowed_context.window().set_cursor_visible(false);

    // Set up a shared vector for keeping track of currently pressed keys
    let arc_pressed_keys = Arc::new(Mutex::new(Vec::<VirtualKeyCode>::with_capacity(10)));
    // Make a reference of this vector to send to the render thread
    let pressed_keys = Arc::clone(&arc_pressed_keys);

    // Set up shared tuple for tracking mouse movement between frames
    let arc_mouse_delta = Arc::new(Mutex::new((0f32, 0f32)));
    // Make a reference of this tuple to send to the render thread
    let mouse_delta = Arc::clone(&arc_mouse_delta);

    let mut camera = Camera {
        x: 0.0,
        y: -2.0,
        z: -1.0,
        yaw: 0.0,
        pitch: -1.2,
    };

    // This will not change in our case, so no need to recalculate every frame
    let fovy = (SCREEN_H as f32) / (SCREEN_W as f32);

    // Spawn a separate thread for rendering, so event handling doesn't block rendering
    let render_thread = thread::spawn(move || {
        // Acquire the OpenGL Context and load the function pointers. This has to be done inside of the rendering thread, because
        // an active OpenGL context cannot safely traverse a thread boundary
        let context = unsafe {
            let c = windowed_context.make_current().unwrap();
            gl::load_with(|symbol| c.get_proc_address(symbol) as *const _);
            c
        };

        // Set up openGL
        unsafe {
            gl::Enable(gl::DEPTH_TEST);
            gl::DepthFunc(gl::LESS);
            gl::Enable(gl::CULL_FACE);
            gl::Disable(gl::MULTISAMPLE);
            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
            gl::Enable(gl::DEBUG_OUTPUT_SYNCHRONOUS);
            gl::DebugMessageCallback(Some(util::debug_callback), ptr::null());

            // Print some diagnostics
            println!(
                "{}: {}",
                util::get_gl_string(gl::VENDOR),
                util::get_gl_string(gl::RENDERER)
            );
            println!("OpenGL\t: {}", util::get_gl_string(gl::VERSION));
            println!(
                "GLSL\t: {}",
                util::get_gl_string(gl::SHADING_LANGUAGE_VERSION)
            );
        }

        let (vertices, indices, colors) = tasks::assignment2_task4b();

        // Set up VAO
        let vao_id = unsafe { make_vao(&vertices, &indices, &colors) };

        // Load shaders
        let shader = unsafe {
            shader::ShaderBuilder::new()
                .attach_file("shaders/simple.vert")
                .attach_file("shaders/simple.frag")
                .link()
        };

        let first_frame_time = std::time::Instant::now();
        let mut last_frame_time = first_frame_time;

        // The main rendering loop
        loop {
            let now = std::time::Instant::now();
            let elapsed = now.duration_since(first_frame_time).as_secs_f32();
            let delta_time = now.duration_since(last_frame_time).as_secs_f32();
            last_frame_time = now;

            // Handle keyboard input
            if let Ok(keys) = pressed_keys.lock() {
                for key in keys.iter() {
                    match key {
                        VirtualKeyCode::A => {
                            camera.move_left(delta_time);
                        }
                        VirtualKeyCode::D => {
                            camera.move_right(delta_time);
                        }
                        VirtualKeyCode::W => {
                            camera.move_forward(delta_time);
                        }
                        VirtualKeyCode::S => {
                            camera.move_backward(delta_time);
                        }
                        VirtualKeyCode::LShift => {
                            camera.move_down(delta_time);
                        }
                        VirtualKeyCode::Space => {
                            camera.move_up(delta_time);
                        }
                        _ => {}
                    }
                }
            }

            // Handle mouse movement. delta contains the x and y movement of the mouse since last frame in pixels
            if let Ok(mut delta) = mouse_delta.lock() {
                camera.move_mouse(delta.0, delta.1);
                *delta = (0.0, 0.0);
            }

            unsafe {
                gl::ClearColor(0.76862745, 0.71372549, 0.94901961, 1.0); // moon raker, full opacity
                gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

                // Make sure the VAO is selected before we call draw
                gl::BindVertexArray(vao_id);

                // Activate shader to draw elements and update uniforms
                shader.activate();

                let mut transformation: glm::Mat4 = glm::Mat4::identity();
                transformation =
                    glm::translation(&glm::vec3(-camera.x, -camera.y, -camera.z - 2.0))
                        * transformation;
                transformation =
                    glm::rotation(camera.yaw, &glm::vec3(0.0, 1.0, 0.0)) * transformation;
                transformation =
                    glm::rotation(camera.pitch, &glm::vec3(1.0, 0.0, 0.0)) * transformation;
                transformation = glm::perspective(fovy, 45f32, 1.0, 100.0) * transformation;

                gl::UniformMatrix4fv(2, 1, gl::FALSE, transformation.as_ptr());
                // Draw scene
                gl::DrawElements(
                    gl::TRIANGLES,
                    indices.len() as i32,
                    gl::UNSIGNED_INT,
                    ptr::null(),
                );
            }

            context.swap_buffers().unwrap();
        }
    });

    // Keep track of the health of the rendering thread
    let render_thread_healthy = Arc::new(RwLock::new(true));
    let render_thread_watchdog = Arc::clone(&render_thread_healthy);
    thread::spawn(move || {
        if !render_thread.join().is_ok() {
            if let Ok(mut health) = render_thread_watchdog.write() {
                println!("Render thread panicked!");
                *health = false;
            }
        }
    });

    // Start the event loop -- This is where window events get handled
    el.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        // Terminate program if render thread panics
        if let Ok(health) = render_thread_healthy.read() {
            if *health == false {
                *control_flow = ControlFlow::Exit;
            }
        }

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                *control_flow = ControlFlow::Exit;
            }
            // Keep track of currently pressed keys to send to the rendering thread
            Event::WindowEvent {
                event:
                    WindowEvent::KeyboardInput {
                        input:
                            KeyboardInput {
                                state: key_state,
                                virtual_keycode: Some(keycode),
                                ..
                            },
                        ..
                    },
                ..
            } => {
                if let Ok(mut keys) = arc_pressed_keys.lock() {
                    match key_state {
                        Released => {
                            if keys.contains(&keycode) {
                                let i = keys.iter().position(|&k| k == keycode).unwrap();
                                keys.remove(i);
                            }
                        }
                        Pressed => {
                            if !keys.contains(&keycode) {
                                keys.push(keycode);
                            }
                        }
                    }
                }

                // Handle escape separately
                match keycode {
                    Escape => {
                        *control_flow = ControlFlow::Exit;
                    }
                    Q => {
                        *control_flow = ControlFlow::Exit;
                    }
                    _ => {}
                }
            }
            Event::DeviceEvent {
                event: DeviceEvent::MouseMotion { delta },
                ..
            } => {
                // Accumulate mouse movement
                if let Ok(mut position) = arc_mouse_delta.lock() {
                    *position = (position.0 + delta.0 as f32, position.1 + delta.1 as f32);
                }
            }
            _ => {}
        }
    });
}
