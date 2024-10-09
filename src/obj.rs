use nalgebra_glm::{Vec2, Vec3};
use tobj;

use crate::vertex::Vertex;

pub struct Obj {
    vertices: Vec<Vec3>,
    normals: Vec<Vec3>,
    texcoords: Vec<Vec2>,
    indices: Vec<usize>,
}

impl Obj {
    pub fn load(filename: &str) -> Result<Self, tobj::LoadError> {
        let (models, _) = tobj::load_obj(
            filename,
            &tobj::LoadOptions {
                single_index: true,
                triangulate: true,
                ..Default::default()
            },
        )?;

        let mesh = &models[0].mesh;

        let vertices: Vec<Vec3> = mesh
            .positions
            .chunks(3)
            .map(|v| Vec3::new(v[0], v[1], v[2]))
            .collect();

        let normals: Vec<Vec3> = mesh
            .normals
            .chunks(3)
            .map(|n| Vec3::new(n[0], n[1], n[2]))
            .collect();

        let texcoords: Vec<Vec2> = mesh
            .texcoords
            .chunks(2)
            .map(|t| Vec2::new(t[0], t[1]))
            .collect();

        let indices = mesh.indices.iter().map(|idx| *idx as usize).collect();

        Ok(Obj {
            vertices,
            normals,
            texcoords,
            indices,
        })
    }

    pub fn get_vertex_array(&self) -> Vec<Vertex> {
        (self.indices.iter().map(|idx| {
            let position = *self.vertices.get(*idx).unwrap();
            let normal = *self.normals.get(*idx).unwrap();
            let tex_cords = *self.texcoords.get(*idx).unwrap();
            Vertex::new(position, normal, tex_cords)
        }))
        .collect()
    }
}
