use nalgebra_glm::{Mat4, Vec2, Vec3, Vec4};

use crate::{
    fragment::triangle, framebuffer::Framebuffer, shader::vertex_shader, vertex::Vertex, Model,
};

pub struct Uniforms {
    pub model_matrix: Mat4,
}

pub fn render(framebuffer: &mut Framebuffer, data: &Model) {
    let Model { objs, uniforms, .. } = data;

    for obj in objs {
        let vertex_array = obj.get_vertex_array();

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
            // println!("Painting point: {position:?}");
            framebuffer.set_current_color(fragment.color);
            let _ = framebuffer.paint_point(fragment.position);
        }
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
