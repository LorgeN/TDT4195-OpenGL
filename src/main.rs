extern crate nalgebra_glm as glm;

use std::ptr;
use std::sync::{Arc, Mutex, RwLock};
use std::thread;

use self::camera::Camera;
use self::scene_graph::SceneNode;

mod camera;
mod colors;
mod mesh;
mod scene_graph;
mod shader;
mod shapes;
mod tasks;
mod toolbox;
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
const HELICOPTER_COUNT: u32 = 5;

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
        util::byte_size_of_array(values),
        util::pointer_to_array(values),
        gl::STATIC_DRAW,
    );

    buffer_id
}

/// Makes a new VAO, feeds the vertices to a new VBO for said VAO and makes an index buffer
/// with the given indices
unsafe fn make_vao(
    vertices: &Vec<f32>,
    indices: &Vec<u32>,
    colors: &Vec<f32>,
    normals: &Vec<f32>,
) -> u32 {
    let mut id = 0u32;
    // Make and bind VAO
    gl::GenVertexArrays(1, &mut id as *mut u32);
    gl::BindVertexArray(id);

    // Make and fill buffer. This function leaves the created VBO bound
    make_buffer(gl::ARRAY_BUFFER, vertices);
    gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 0, ptr::null());

    make_buffer(gl::ARRAY_BUFFER, colors);
    gl::VertexAttribPointer(1, 4, gl::FLOAT, gl::FALSE, 0, ptr::null());

    make_buffer(gl::ARRAY_BUFFER, normals);
    gl::VertexAttribPointer(2, 3, gl::FLOAT, gl::FALSE, 0, ptr::null());

    gl::EnableVertexAttribArray(0);
    gl::EnableVertexAttribArray(1);
    gl::EnableVertexAttribArray(2);

    make_buffer(gl::ELEMENT_ARRAY_BUFFER, indices);

    id
}

unsafe fn make_mesh_vao(mesh: &mesh::Mesh) -> u32 {
    make_vao(&mesh.vertices, &mesh.indices, &mesh.colors, &mesh.normals)
}

unsafe fn draw_mesh_vao(
    vao_id: &u32,
    index_count: &i32,
    total_transform: &glm::Mat4,
    model_transform: &glm::Mat4,
    shader_id: &u32,
) {
    // Make sure the VAO is selected before we call draw
    gl::BindVertexArray(*vao_id);

    // Activate shader to draw elements and update uniforms
    gl::UseProgram(*shader_id);

    gl::UniformMatrix4fv(3, 1, gl::FALSE, total_transform.as_ptr());
    gl::UniformMatrix4fv(4, 1, gl::FALSE, model_transform.as_ptr());
    // Draw scene
    gl::DrawElements(gl::TRIANGLES, *index_count, gl::UNSIGNED_INT, ptr::null());
}

/// Draws the given node and all children using the given view transform
unsafe fn draw_scene(node: &SceneNode, view_transform: &glm::Mat4) {
    if node.vao_id > 0 {
        draw_mesh_vao(
            &node.vao_id,
            &node.index_count,
            &(view_transform * node.current_transformation_matrix),
            &node.current_transformation_matrix,
            &node.shader_id,
        );
    }

    for &child in &node.children {
        draw_scene(&*child, view_transform);
    }
}

/// Updates all node local transformations
unsafe fn update_node_transformations(node: &mut SceneNode, initial_transform: &glm::Mat4) {
    // Construct transformation matrix
    let mut transform = glm::Mat4::identity();
    // First translate by (subtract) reference point, then rotate, then translate back
    transform = glm::translation(&node.reference_point)
        * glm::rotation(node.rotation.x, &glm::vec3(1.0, 0.0, 0.0))
        * glm::rotation(node.rotation.y, &glm::vec3(0.0, 1.0, 0.0))
        * glm::rotation(node.rotation.z, &glm::vec3(0.0, 0.0, 1.0))
        * glm::translation(&-node.reference_point)
        * transform;
    transform = glm::translation(&node.position) * transform;
    transform = glm::scaling(&node.scale) * transform;

    node.current_transformation_matrix = initial_transform * transform;

    for &child in &node.children {
        update_node_transformations(&mut *child, &node.current_transformation_matrix);
    }
}

/// Utility struct for keeping track of a helicopter model instance
struct Helicopter {
    id: u32,
    body: scene_graph::Node,
    main_rotor: scene_graph::Node,
    tail_rotor: scene_graph::Node,
    door: scene_graph::Node,
}

impl Helicopter {
    /// The time for this specific helicopter, offset to compensate
    /// for collisions between multiple helicopters in the scene
    fn get_time_with_offset(&mut self, time: f32, offset: f32) -> f32 {
        return time + (offset * self.id as f32);
    }

    /// Updates rotor values to have spinning animation
    fn update_rotors(&mut self, time: f32, offset: f32) {
        let elapsed = self.get_time_with_offset(time, offset);

        self.main_rotor.rotation.y = elapsed * 10.0;
        self.tail_rotor.rotation.x = elapsed * 15.0;
    }

    /// Updates heading for this helicopter's animation
    fn update_heading(&mut self, time: f32, offset: f32) {
        let heading = toolbox::simple_heading_animation(self.get_time_with_offset(time, offset));
        self.body.position.x = heading.x;
        self.body.position.z = heading.z;
        self.body.rotation.x = heading.pitch;
        self.body.rotation.y = heading.yaw;
        self.body.rotation.z = heading.roll;
    }
}

/// Makes VAOs for helicopter model and instanties a given amount of scene
/// nodes using them
fn make_helicopters(shader_id: u32, amount: u32) -> Vec<Helicopter> {
    // Load meshes
    let model = mesh::Helicopter::load("resources/helicopter.obj");

    // Helicopter VAOs and shader
    let body_vao_id = unsafe { make_mesh_vao(&model.body) };
    let door_vao_id = unsafe { make_mesh_vao(&model.door) };
    let main_rot_vao_id = unsafe { make_mesh_vao(&model.main_rotor) };
    let tail_rot_vao_id = unsafe { make_mesh_vao(&model.tail_rotor) };

    (0..amount)
        .map(|idx| {
            let mut root = SceneNode::from_vao(body_vao_id, shader_id, model.body.index_count);

            let mut door = SceneNode::from_vao(door_vao_id, shader_id, model.door.index_count);
            // Guessing the reference point for this one (Little bit up and to the left)
            door.reference_point = glm::vec3(1.0, 1.5, 0.0);

            let mut main_rot =
                SceneNode::from_vao(main_rot_vao_id, shader_id, model.main_rotor.index_count);
            // This doesn't actually need a reference point for the rotation we wish to perform,
            // but say we want to implement some kind of "tilt" to animate how a helicopter
            // changes direction we could do that with a reference point at the center of the rotor
            // which this is pretty close to being
            main_rot.reference_point = glm::Vec3::new(0.0, 2.3, 0.0);
            let mut tail_rot =
                SceneNode::from_vao(tail_rot_vao_id, shader_id, model.tail_rotor.index_count);
            tail_rot.reference_point = glm::Vec3::new(0.35, 2.3, 10.4);

            // Add children to root node
            root.add_child(&door);
            root.add_child(&main_rot);
            root.add_child(&tail_rot);

            Helicopter {
                id: idx,
                body: root,
                door: door,
                main_rotor: main_rot,
                tail_rotor: tail_rot,
            }
        })
        .collect()
}

/// Makes a scene graph with the lunar terrain and the given amount of helicopters
fn make_scene_graph(helicopters: u32) -> (scene_graph::Node, Vec<Helicopter>) {
    let terrain = mesh::Terrain::load("resources/lunarsurface.obj");

    // Set up terrain VAO and load shader
    let terrain_vao_id = unsafe { make_mesh_vao(&terrain) };
    let shader = unsafe {
        shader::ShaderBuilder::new()
            .attach_file("shaders/simple.vert")
            .attach_file("shaders/simple.frag")
            .link()
    };

    let mut terrain_node =
        SceneNode::from_vao(terrain_vao_id, shader.program_id, terrain.index_count);

    let helicopters = make_helicopters(shader.program_id, helicopters);

    // Add helicopters to the terrain
    for helicopter in &helicopters {
        terrain_node.add_child(&helicopter.body);
    }

    let mut root = SceneNode::new();
    root.add_child(&terrain_node);
    (root, helicopters)
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

    // Instantiate camera
    let mut camera = Camera::new();

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

        let first_frame_time = std::time::Instant::now();
        let mut last_frame_time = first_frame_time;
        let (mut root_node, mut helicopters) = make_scene_graph(HELICOPTER_COUNT);

        // This will not change, so no need to recalculate for each frame
        let fovy = (SCREEN_H as f32) / (SCREEN_W as f32);
        let offset = 15.0 / HELICOPTER_COUNT as f32;

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

            for helicopter in helicopters.iter_mut() {
                helicopter.update_heading(elapsed, offset);
                helicopter.update_rotors(elapsed, offset);
            }

            unsafe {
                gl::ClearColor(0.76862745, 0.71372549, 0.94901961, 1.0); // moon raker, full opacity
                gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

                update_node_transformations(&mut root_node, &glm::Mat4::identity());
                draw_scene(&root_node, &camera.make_view_transform(fovy));
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
