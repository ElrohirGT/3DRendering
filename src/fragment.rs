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
    let steps = 1e-10;
    let direction = a.position - b.position;

    loop {
        let new_position = a.position + steps * direction;
        if new_position.x > b.position.x
            && new_position.y > b.position.y
            && new_position.z > b.position.z
        {
            break;
        }

        fragments.push(Fragment::new(new_position, Color::default()))
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
