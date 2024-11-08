use nalgebra_glm::Vec3;
use rayon::prelude::*;

use crate::{
    fragment::{triangle, Fragment},
    framebuffer::Framebuffer,
    shader::{vertex_shader, Uniforms},
    vertex::Vertex,
    Model,
};

pub fn render(framebuffer: &mut Framebuffer, data: &Model) {
    let Model {
        objs,
        uniforms,
        camera,
        ..
    } = data;

    for obj in objs {
        let vertex_array = obj.get_vertex_array();

        // Vertex Shader
        println!("Applying shaders...");
        let new_vertices = apply_shaders(&vertex_array, uniforms);
        println!("Vertex shader applied!");

        // Primitive assembly
        println!("Assembly...");
        let triangles = assembly(&new_vertices);
        println!("Assembly done!");

        // Rasterization
        println!("Applying rasterization...");
        let fragments = rasterize(triangles, &camera.direction());
        println!("Rasterization applied!");

        // Fragment Processing
        println!("Painting fragments...");
        paint_fragments(fragments, framebuffer);
        println!("Fragments painted!");
    }
}

fn apply_shaders(vertices: &[Vertex], uniforms: &Uniforms) -> Vec<Vertex> {
    vertices
        .par_iter()
        .map(|v| vertex_shader(v, uniforms))
        .collect()
}

fn assembly(vertices: &[Vertex]) -> Vec<&[Vertex]> {
    vertices.chunks(3).collect()
}

fn rasterize(triangles: Vec<&[Vertex]>, camera_direction: &Vec3) -> Vec<Fragment> {
    triangles
        .par_iter()
        .flat_map(|tri| triangle(&tri[0], &tri[1], &tri[2], camera_direction))
        .collect()
}

fn paint_fragments(fragments: Vec<Fragment>, framebuffer: &mut Framebuffer) {
    for fragment in fragments {
        framebuffer.set_current_color(fragment.color);
        let _ = framebuffer.paint_point(fragment.position);
    }
}
