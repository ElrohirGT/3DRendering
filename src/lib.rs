use blenders::BlendMode;
use camera::Camera;
use color::Color;
use nalgebra_glm::Vec3;
use obj::Obj;
use shader::{ShaderType, Uniforms};

pub mod blenders;
pub mod bmp;
pub mod camera;
pub mod color;
pub mod fragment;
pub mod framebuffer;
pub mod light;
pub mod material;
pub mod obj;
pub mod planets;
pub mod render;
pub mod shader;
pub mod vertex;

pub fn equal(a: f32, b: f32, eps: f32) -> bool {
    (a - b).abs() < eps
}

pub enum Message {
    RotateCamera(f32, f32),
    ZoomCamera(f32),
    UpdateTime(f32),
    ChangePlanet(Entity),
}

pub type EntityShader = (ShaderType, Vec<Color>, BlendMode);

pub struct Entity {
    pub objs: Vec<Obj>,
    pub shaders: Vec<EntityShader>,
}

pub struct Model {
    pub entities: Vec<Entity>,
    pub render_entities: Vec<Entity>,
    pub uniforms: Uniforms,
    pub rotation: Vec3,
    pub translation: Vec3,
    pub scale: f32,
    pub camera: Camera,
}
