// Utility function for generating vertices and indices for a grid of triangles
pub fn generate_triangles(x: u32, y: u32) -> (Vec<f32>, Vec<u32>) {
    let mut vert: Vec<f32> = Vec::new();

    let x_dist = 2.0 / (x as f32 + 1.0);
    let y_dist = 2.0 / (y as f32 + 1.0);

    let x_size = x_dist / 4.0;
    let y_size = y_dist / 4.0;

    for x_off in 1..=x {
        for y_off in 1..=y {
            let x_curr = -1.0 + x_off as f32 * x_dist;
            let y_curr = -1.0 + y_off as f32 * y_dist;

            vert.extend(vec![
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

    (vert, (0..(x * y * 3)).collect())
}

pub fn generate_circle(r: f32, segments: u32) -> (Vec<f32>, Vec<u32>) {
    let mut vert = vec![0.0, 0.0, 0.0];
    let mut indices: Vec<u32> = Vec::new();

    for seg in 0..segments {
        let angle = 2.0 * std::f32::consts::PI * seg as f32 / segments as f32;
        vert.extend(vec![r * angle.sin(), r * angle.cos(), 0.0]);
        // Have to add segments to seg for last index because we are using unsigned ints
        indices.extend(vec![0, seg + 1, (seg + segments - 1) % segments + 1]);
    }

    (vert, indices)
}

pub fn generate_spiral(r: f32, segments: u32, circles: u32, end_width: f32) -> (Vec<f32>, Vec<u32>) {
    let mut vert: Vec<f32> = Vec::new();
    let mut indices: Vec<u32> = Vec::new();

    let total_seg = segments * circles;

    let mut width = 0.01;
    let mut curr_r = 0.01;

    let w_inc = end_width / ((total_seg - 1) as f32);
    let r_inc = (r - curr_r) / ((total_seg - 1) as f32);

    for seg in 0..total_seg {
        let angle = 2.0 * std::f32::consts::PI * (seg % segments) as f32 / segments as f32;
        
        vert.extend(vec![
            // Inner point
            (curr_r - width) * angle.sin(),
            (curr_r - width) * angle.cos(),
            0.0,
            // Outer point
            curr_r * angle.sin(),
            curr_r * angle.cos(),
            0.0
        ]);

        curr_r = curr_r + r_inc;
        width = width + w_inc;

        let index = seg * 2;

        if seg > 1 {
            indices.extend(vec![index, index + 1, index - 2]);
        }

        indices.extend(vec![index + 3, index + 1, index]);
    }

    (vert, indices)
}
