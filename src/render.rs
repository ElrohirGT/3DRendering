use core::f32;

use nalgebra_glm::{Mat4, Vec2, Vec3, Vec4};
use rayon::prelude::*;

use crate::{
    fragment::{triangle, Fragment},
    framebuffer::Framebuffer,
    shader::vertex_shader,
    vertex::Vertex,
    Model,
};

pub fn render(framebuffer: &mut Framebuffer, data: &Model) {
    let Model { objs, uniforms, .. } = data;

    for obj in objs {
        let vertex_array = obj.get_vertex_array();

        // Vertex Shader
        println!("Applying shaders...");
        let new_vertices: Vec<Vertex> = vertex_array
            .par_iter()
            .map(|v| vertex_shader(v, uniforms))
            .collect();
        println!("Vertex shader applied!");

        // Primitive assembly
        let triangles: Vec<&[Vertex]> = new_vertices.chunks(3).collect();

        // Rasterization
        println!("Applying rasterization...");
        let fragments: Vec<Fragment> = triangles
            .par_iter()
            .flat_map(|tri| triangle(&tri[0], &tri[1], &tri[2]))
            .collect();
        println!("Rasterization applied!");

        // Fragment Processing
        println!("Painting fragments...");
        for fragment in fragments {
            framebuffer.set_current_color(fragment.color);
            let _ = framebuffer.paint_point(fragment.position);
        }
        println!("Fragments painted!");
    }
}
