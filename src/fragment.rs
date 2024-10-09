use crate::{color::Color, vertex::Vertex};
use nalgebra_glm::Vec3;

pub struct Fragment {
    pub position: Vec3,
    pub color: Color,
}

impl Fragment {
    pub fn new(position: Vec3, color: Color) -> Self {
        Fragment { position, color }
    }
}

pub fn line(a: &Vertex, b: &Vertex) -> Vec<Fragment> {
    let mut fragments = vec![];
    // let distance = nalgebra_glm::distance(&b.transformed_position, &a.transformed_position);
    // let step_size = 1.0 / (10.0 / 2.0 * distance);
    let step_size = 1.0e-3;
    let direction = b.transformed_position - a.transformed_position;

    // println!(
    //     "From {:?} to {:?}, DIR={direction:?}",
    //     b.transformed_position, a.transformed_position
    // );

    let mut accum = 0.0;
    while accum <= 1.0 {
        let new_position = a.transformed_position + accum * direction;
        // println!("POINT: {new_position:?} t={accum}");
        fragments.push(Fragment::new(new_position, Color::pink()));
        accum += step_size;
    }

    fragments
}

pub fn triangle(v1: &Vertex, v2: &Vertex, v3: &Vertex) -> Vec<Fragment> {
    line(v1, v2)
        .into_iter()
        .chain(line(v2, v3))
        .chain(line(v3, v1))
        .collect()
}
