pub mod bmp;
pub mod color;
pub mod fragment;
pub mod framebuffer;
pub mod obj;
pub mod render;
pub mod vertex;

pub fn equal(a: f32, b: f32, eps: f32) -> bool {
    (a - b).abs() < eps
}
