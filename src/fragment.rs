use crate::{color::Color, vertex::Vertex};
use nalgebra_glm::{Vec2, Vec3};

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
    let mut fragments = vec![];
    let (a, b, c) = (
        v1.transformed_position,
        v2.transformed_position,
        v3.transformed_position,
    );

    let (min, max) = calculate_bounding_box(&a, &b, &c);

    let step_size = 1e-3;
    let mut currentx = min.x;
    let mut currenty = min.y;

    while currenty < max.y {
        while currentx < max.x {
            let point = Vec3::new(currentx, currenty, 0.0);

            let (u, v, w) = barycentric_coordinates(&point, &a, &b, &c);

            if (0.0..=1.0).contains(&u) && (0.0..=1.0).contains(&v) && (0.0..=1.0).contains(&w) {
                let base_color = Color::new(100, 100, 100);
                fragments.push(Fragment::new(point, base_color));
            }

            currentx += step_size;
        }
        currenty += step_size;
    }

    fragments
}

fn calculate_bounding_box(v1: &Vec3, v2: &Vec3, v3: &Vec3) -> (Vec2, Vec2) {
    let minx = v1.x.min(v2.x).min(v3.x);
    let miny = v1.y.min(v2.y).min(v3.y);

    let maxx = v1.x.max(v2.x).max(v3.x);
    let maxy = v1.y.max(v2.y).max(v3.y);

    (Vec2::new(minx, miny), Vec2::new(maxx, maxy))
}

fn barycentric_coordinates(p: &Vec3, a: &Vec3, b: &Vec3, c: &Vec3) -> (f32, f32, f32) {
    let pa = a - p;
    let ab = b - a;
    let ac = c - a;

    let v = (pa.y * ab.x - pa.x * ab.y) / (ac.x * ab.y - ac.y * ab.x);
    let u = (v * ac.y + pa.y) / ab.y;
    let w = 1.0 - u - v;

    (u, v, w)
}
