use std::ffi::CString;

// Utility function for generating vertices and indices for a grid of triangles
pub fn generate_triangles(x: u32, y: u32) -> (Vec<f32>, Vec<u32>) {
    let mut triangles: Vec<f32> = Vec::new();

    let x_dist = 2.0 / (x as f32 + 1.0);
    let y_dist = 2.0 / (y as f32 + 1.0);

    let x_size = x_dist / 4.0;
    let y_size = y_dist / 4.0;

    for x_off in 1..=x {
        for y_off in 1..=y {
            let x_curr = -1.0 + x_off as f32 * x_dist;
            let y_curr = -1.0 + y_off as f32 * y_dist;

            triangles.extend(vec![
                x_curr,
                y_curr + y_size,
                0.0,
                x_curr - x_size,
                y_curr - y_size,
                0.0,
                x_curr + x_size,
                y_curr - y_size,
                0.0,
            ]);
        }
    }

    (triangles, (0..(x * y * 3)).collect())
}

pub unsafe fn get_gl_string(name: gl::types::GLenum) -> String {
    std::ffi::CStr::from_ptr(gl::GetString(name) as *mut i8)
        .to_string_lossy()
        .to_string()
}

// Debug callback to panic upon enountering any OpenGL error
pub extern "system" fn debug_callback(
    source: u32,
    e_type: u32,
    id: u32,
    severity: u32,
    _length: i32,
    msg: *const i8,
    _data: *mut std::ffi::c_void,
) {
    if e_type != gl::DEBUG_TYPE_ERROR {
        return;
    }
    if severity == gl::DEBUG_SEVERITY_HIGH
        || severity == gl::DEBUG_SEVERITY_MEDIUM
        || severity == gl::DEBUG_SEVERITY_LOW
    {
        let severity_string = match severity {
            gl::DEBUG_SEVERITY_HIGH => "high",
            gl::DEBUG_SEVERITY_MEDIUM => "medium",
            gl::DEBUG_SEVERITY_LOW => "low",
            _ => "unknown",
        };
        unsafe {
            let string = CString::from_raw(msg as *mut i8);
            let error_message = String::from_utf8_lossy(string.as_bytes()).to_string();
            panic!(
                "{}: Error of severity {} raised from {}: {}\n",
                id, severity_string, source, error_message
            );
        }
    }
}
