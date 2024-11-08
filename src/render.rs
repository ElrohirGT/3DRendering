use core::f32;

use nalgebra_glm::{Mat4, Vec2, Vec3, Vec4};

use crate::{
    fragment::triangle, framebuffer::Framebuffer, shader::vertex_shader, vertex::Vertex, Model,
};

pub struct Uniforms {
    pub model_matrix: Mat4,
    pub view_matrix: Mat4,
    pub projection_matrix: Mat4,
    pub viewport_matrix: Mat4,
}

pub fn render(framebuffer: &mut Framebuffer, data: &Model) {
    let Model { objs, uniforms, .. } = data;

    for obj in objs {
        let vertex_array = obj.get_vertex_array();

        // Vertex Shader
        println!("Applying shaders...");
        let new_vertices: Vec<Vertex> = vertex_array
            .iter()
            .map(|v| vertex_shader(v, uniforms))
            .collect();
        println!("Vertex shader applied!");

        // Primitive assembly
        let triangles: Vec<&[Vertex]> = new_vertices.chunks(3).collect();

        // Rasterization
        println!("Applying rasterization...");
        let mut fragments = vec![];
        for tri in triangles {
            fragments.extend(triangle(&tri[0], &tri[1], &tri[2]));
        }
        println!("Rasterization applied!");

        // Fragment Processing
        println!("Painting fragments...");
        for fragment in fragments {
            // let color = fragment.color.to_hex();
            // println!("Painting point: {position:?}");
            framebuffer.set_current_color(fragment.color);
            let _ = framebuffer.paint_point(fragment.position);
        }
        println!("Fragments painted!");
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
        100.0,
        0.0,
        0.0,
        0.0,
        0.0,
        1.0,
    )
}
