// File containing values and functions for specific tasks to avoid cluttering main.rs
#[path = "./shapes.rs"] mod shapes;
#[path = "./colors.rs"] mod colors;

pub fn assignment2_task1b() -> (Vec<f32>, Vec<u32>, Vec<f32>) {
    let (vertices, indices) = shapes::generate_triangles(3, 3, None);
    let colors = colors::make_colors_seeded(vertices.len() as u32, 12345);
    
    (vertices, indices, colors)
}

pub fn assignment2_task2a() -> (Vec<f32>, Vec<u32>, Vec<f32>) {
    let mut vertices = Vec::new();
    vertices.extend(shapes::generate_triangle_at(0.0, -0.1, 0.1, 0.25));
    vertices.extend(shapes::generate_triangle_at(0.1, 0.1, 0.0, 0.25));
    vertices.extend(shapes::generate_triangle_at(-0.1, 0.1, -0.1, 0.25));
    #[rustfmt::skip] 
    let colors = vec![
        1.0, 0.0, 0.0, 0.5, 
        1.0, 0.0, 0.0, 0.5,
        1.0, 0.0, 0.0, 0.5,
        0.0, 1.0, 0.0, 0.5,
        0.0, 1.0, 0.0, 0.5, 
        0.0, 1.0, 0.0, 0.5, 
        0.0, 0.0, 1.0, 0.5, 
        0.0, 0.0, 1.0, 0.5, 
        0.0, 0.0, 1.0, 0.5,
    ];

    (
        vertices,
        (0..9).collect(),
        colors,
    )
}

pub fn assignment2_task2b_i() -> (Vec<f32>, Vec<u32>, Vec<f32>) {
    let mut vertices = Vec::new();
    vertices.extend(shapes::generate_triangle_at(0.0, -0.1, 0.1, 0.25));
    vertices.extend(shapes::generate_triangle_at(0.1, 0.1, 0.0, 0.25));
    vertices.extend(shapes::generate_triangle_at(-0.1, 0.1, -0.1, 0.25));
    #[rustfmt::skip] 
    let colors = vec![
        0.0, 1.0, 0.0, 0.5,
        0.0, 1.0, 0.0, 0.5, 
        0.0, 1.0, 0.0, 0.5, 
        0.0, 0.0, 1.0, 0.5, 
        0.0, 0.0, 1.0, 0.5, 
        0.0, 0.0, 1.0, 0.5,
        1.0, 0.0, 0.0, 0.5, 
        1.0, 0.0, 0.0, 0.5,
        1.0, 0.0, 0.0, 0.5,
    ];

    (
        vertices,
        (0..9).collect(),
        colors,
    )
}

pub fn assignment2_task2b_ii() -> (Vec<f32>, Vec<u32>, Vec<f32>) {
    let mut vertices = Vec::new();
    vertices.extend(shapes::generate_triangle_at(0.0, -0.1, -0.1, 0.25));
    vertices.extend(shapes::generate_triangle_at(0.1, 0.1, 0.0, 0.25));
    vertices.extend(shapes::generate_triangle_at(-0.1, 0.1, 0.1, 0.25));
    #[rustfmt::skip] 
    let colors = vec![
        0.0, 1.0, 0.0, 0.5,
        0.0, 1.0, 0.0, 0.5, 
        0.0, 1.0, 0.0, 0.5, 
        0.0, 0.0, 1.0, 0.5, 
        0.0, 0.0, 1.0, 0.5, 
        0.0, 0.0, 1.0, 0.5,
        1.0, 0.0, 0.0, 0.5, 
        1.0, 0.0, 0.0, 0.5,
        1.0, 0.0, 0.0, 0.5,
    ];

    (
        vertices,
        (0..9).collect(),
        colors,
    )
}

pub fn assignment2_task4b() -> (Vec<f32>, Vec<u32>, Vec<f32>) {
    let vertices = shapes::generate_triangle_at(0.0, 0.0, 0.0, 0.5);
    let indices = vec![0, 1, 2];
    let color = vec![
        0.0, 1.0, 0.0, 1.0,
        0.0, 0.0, 1.0, 1.0, 
        1.0, 0.0, 0.0, 1.0, 
    ];
    
    (vertices, indices, color)
}