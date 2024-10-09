use nalgebra_glm::{Mat4, Vec2, Vec3, Vec4};

use crate::{fragment::triangle, framebuffer::Framebuffer, vertex::Vertex};

pub struct Uniforms {
    model_matrix: Mat4,
}

pub fn render(framebuffer: &mut Framebuffer, uniforms: &Uniforms, vertex_array: &[Vertex]) {
    // Vertex Shader
    let new_vertices: Vec<Vertex> = vertex_array
        .iter()
        .map(|v| vertex_shader(v, uniforms))
        .collect();

    // Primitive assembly
    let triangles: Vec<&[Vertex]> = new_vertices.chunks(3).collect();

    // Rasterization
    let mut fragments = vec![];
    for tri in triangles {
        fragments.extend(triangle(&tri[0], &tri[1], &tri[2]));
    }

    // Fragment Processing
    for fragment in fragments {
        // let color = fragment.color.to_hex();
        let position = Vec2::new(fragment.position.x, fragment.position.y);
        framebuffer.set_current_color(fragment.color);
        let _ = framebuffer.paint_point(position);
    }
}

fn vertex_shader(vertex: &Vertex, uniforms: &Uniforms) -> Vertex {
    let position = Vec4::new(vertex.position.x, vertex.position.y, vertex.position.z, 1.0);
    let transformed = uniforms.model_matrix * position;

    let w = transformed.w;
    let transformed_position = Vec3::new(transformed.x / w, transformed.y / w, transformed.z / w);

    // Transform normal

    Vertex {
        transformed_position,
        transformed_normal: vertex.normal,
        ..vertex.clone()
    }
}

pub fn create_model_matrix(translation: Vec3, scale: f32, rotation: Vec3) -> Mat4 {
    Mat4::new(
        scale,
        0.0,
        0.0,
        translation.x,
        0.0,
        scale,
        0.0,
        translation.y,
        0.0,
        0.0,
        scale,
        translation.z,
        0.0,
        0.0,
        0.0,
        1.0,
    )
}
