use nalgebra_glm::{Mat3, Vec3, Vec4};

use crate::{render::Uniforms, vertex::Vertex};

pub fn vertex_shader(vertex: &Vertex, uniforms: &Uniforms) -> Vertex {
    let position = Vec4::new(vertex.position.x, vertex.position.y, vertex.position.z, 1.0);
    let transformed = uniforms.viewport_matrix
        * uniforms.projection_matrix
        * uniforms.view_matrix
        * uniforms.model_matrix
        * position;
    // println!("{position:?} TURNED INTO {transformed:?}");

    let w = transformed.w;
    let transformed_position = Vec3::new(transformed.x / w, transformed.y / w, transformed.z / w);

    // Transform normal
    let model_mat3 = Mat3::new(
        uniforms.model_matrix[0],
        uniforms.model_matrix[1],
        uniforms.model_matrix[2],
        uniforms.model_matrix[4],
        uniforms.model_matrix[5],
        uniforms.model_matrix[6],
        uniforms.model_matrix[8],
        uniforms.model_matrix[9],
        uniforms.model_matrix[10],
    );
    let normal_matrix = model_mat3
        .transpose()
        .try_inverse()
        .unwrap_or(Mat3::identity());
    let transformed_normal = normal_matrix * vertex.normal;
    // println!("{normal_matrix:?} -> {transformed_normal:?}");

    Vertex {
        transformed_position,
        transformed_normal,
        position: vertex.position,
        normal: vertex.normal,
        tex_coords: vertex.tex_coords,
        color: vertex.color,
    }
}
