use camera::Camera;
use render::Uniforms;
use vertex::Vertex;

pub mod bmp;
pub mod camera;
pub mod color;
pub mod fragment;
pub mod framebuffer;
pub mod light;
pub mod material;
pub mod obj;
pub mod render;
pub mod vertex;

pub fn equal(a: f32, b: f32, eps: f32) -> bool {
    (a - b).abs() < eps
}

pub enum Message {}
pub struct Model {
    pub vertex_array: Vec<Vertex>,
    pub uniforms: Uniforms,
    pub camera: Camera,
}
