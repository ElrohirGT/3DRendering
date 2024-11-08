use camera::Camera;
use nalgebra_glm::Vec3;
use obj::Obj;
use shader::Uniforms;
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
pub mod shader;
pub mod vertex;

pub fn equal(a: f32, b: f32, eps: f32) -> bool {
    (a - b).abs() < eps
}

pub enum Message {
    RotateCamera(f32, f32),
    ZoomCamera(f32),
}
pub struct Model {
    pub objs: Vec<Obj>,
    pub uniforms: Uniforms,
    pub rotation: Vec3,
    pub translation: Vec3,
    pub scale: f32,
    pub camera: Camera,
}
