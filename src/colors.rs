use rand::prelude::*;
use rand::rngs::StdRng;

/// Generates random RGBA values for the given vertex count
#[allow(dead_code)]
pub fn make_colors(vertex_count: u32) -> Vec<f32> {
    let mut rng = rand::thread_rng();

    // Checking if % 4 == 3 lets us set alpha to 1.0
    (0..(vertex_count << 2))
        .map(|v| if v % 4 == 3 { 1.0 } else { rng.gen() })
        .collect()
}

#[allow(dead_code)]
pub fn make_colors_seeded(vertex_count: u32, seed: u64) -> Vec<f32> {
    let mut rng = StdRng::seed_from_u64(seed);

    // Checking if % 4 == 3 lets us set alpha to 1.0
    (0..(vertex_count << 2))
        .map(|v| if v % 4 == 3 { 1.0 } else { rng.gen() })
        .collect()
}

#[allow(dead_code)]
pub fn make_colors_transparent_triangle(triangle_count: u32) -> Vec<f32> {
    let mut rng = rand::thread_rng();

    (0..triangle_count)
        .flat_map(|_| {
            let r = rng.gen();
            let g = rng.gen();
            let b = rng.gen();
            let a = rng.gen::<f32>() * 0.1 + 0.1;

            [r, g, b, a, r, g, b, a, r, g, b, a]
        })
        .collect()
}
