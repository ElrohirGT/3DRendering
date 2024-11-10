use core::f32;

use nalgebra_glm::{vec3, vec4, Mat3, Mat4, Vec3, Vec4};

use crate::vertex::Vertex;

pub struct Uniforms {
    pub model_matrix: Mat4,
    pub view_matrix: Mat4,
    pub projection_matrix: Mat4,
    pub viewport_matrix: Mat4,
}

pub fn vertex_shader(vertex: &Vertex, uniforms: &Uniforms) -> Vertex {
    let position = vec4(vertex.position.x, vertex.position.y, vertex.position.z, 1.0);
    let transformed = uniforms.viewport_matrix
        * uniforms.projection_matrix
        * uniforms.view_matrix
        * uniforms.model_matrix
        * position;
    println!("{position:?} TURNED INTO {transformed:?}");

    let w = transformed.w;
    let transformed_position = vec3(transformed.x / w, transformed.y / w, transformed.z / w);

    // Transform normal
    // let model_mat3 = Mat3::new(
    //     uniforms.model_matrix[0],
    //     uniforms.model_matrix[1],
    //     uniforms.model_matrix[2],
    //     uniforms.model_matrix[4],
    //     uniforms.model_matrix[5],
    //     uniforms.model_matrix[6],
    //     uniforms.model_matrix[8],
    //     uniforms.model_matrix[9],
    //     uniforms.model_matrix[10],
    // );
    // let normal_matrix = model_mat3
    //     .try_inverse()
    //     .unwrap_or(Mat3::identity())
    //     .transpose();
    let vertex_normal = vec4(vertex.normal.x, vertex.normal.y, vertex.normal.z, 1.0);
    let normal_matrix = uniforms
        .model_matrix
        .try_inverse()
        .unwrap_or(Mat4::identity())
        .transpose();
    let transformed_normal = normal_matrix * vertex_normal;
    // let transformed_normal = vertex.normal;
    // println!("{normal_matrix:?} -> {transformed_normal:?}");

    let w = transformed_normal.w;
    Vertex {
        position: transformed_position,
        normal: vec3(
            transformed_normal.x / w,
            transformed_normal.y / w,
            transformed_normal.z / w,
        ),
        tex_coords: vertex.tex_coords,
        color: vertex.color,
    }
}

pub fn create_model_matrix(translation: Vec3, scale: f32, rotation: Vec3) -> Mat4 {
    let (sinx, cosx) = rotation.x.sin_cos();
    let (siny, cosy) = rotation.y.sin_cos();
    let (sinz, cosz) = rotation.z.sin_cos();

    let rotation_x = Mat4::new(
        1.0, 0.0, 0.0, 0.0, 0.0, cosx, -sinx, 0.0, 0.0, sinx, cosx, 0.0, 0.0, 0.0, 0.0, 1.0,
    );

    let rotation_y = Mat4::new(
        cosy, 0.0, siny, 0.0, 0.0, 1.0, 0.0, 0.0, -siny, 0.0, cosy, 0.0, 0.0, 0.0, 0.0, 1.0,
    );

    let rotation_z = Mat4::new(
        cosz, -sinz, 0.0, 0.0, sinz, cosz, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
    );

    let rotation_matrix = rotation_z * rotation_y * rotation_x;

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
    ) * rotation_matrix
}

pub fn create_view_matrix(eye: Vec3, center: Vec3, up: Vec3) -> Mat4 {
    nalgebra_glm::look_at(&eye, &center, &up)
}

pub fn create_projection_matrix(window_width: f32, window_height: f32) -> Mat4 {
    let fov = 45.0 * f32::consts::PI / 180.0;
    let aspect_ratio = window_width / window_height;
    let near = 0.1;
    let far = 1000.0;

    nalgebra_glm::perspective(fov, aspect_ratio, near, far)
}

pub fn create_viewport_matrix(framebuffer_width: f32, framebuffer_height: f32) -> Mat4 {
    Mat4::new(
        framebuffer_width / 2.0,
        0.0,
        0.0,
        framebuffer_width / 2.0,
        0.0,
        -framebuffer_height / 2.0,
        0.0,
        framebuffer_height / 2.0,
        0.0,
        0.0,
        1.0,
        0.0,
        0.0,
        0.0,
        0.0,
        1.0,
    )
}
