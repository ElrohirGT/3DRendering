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
    pub fn load(filename: &str) -> Result<Vec<Self>, tobj::LoadError> {
        let (models, _) = tobj::load_obj(
            filename,
            &tobj::LoadOptions {
                single_index: true,
                triangulate: true,
                ..Default::default()
            },
        )?;

        // let mesh = &models[4].mesh;
        // let vertices: Vec<Vec3> = mesh
        //     .positions
        //     .chunks(3)
        //     .map(|v| Vec3::new(v[0], v[1], v[2]))
        //     .collect();
        //
        // let normals: Vec<Vec3> = mesh
        //     .normals
        //     .chunks(3)
        //     .map(|n| Vec3::new(n[0], n[1], n[2]))
        //     .collect();
        //
        // let texcoords: Vec<Vec2> = mesh
        //     .texcoords
        //     .chunks(2)
        //     .map(|t| Vec2::new(t[0], t[1]))
        //     .collect();
        //
        // let indices: Vec<usize> = mesh.indices.iter().map(|idx| *idx as usize).collect();

        let objs = models
            .into_iter()
            .map(|m| m.mesh)
            .map(|mesh| {
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

                let indices: Vec<usize> = mesh.indices.iter().map(|idx| *idx as usize).collect();

                Obj {
                    vertices,
                    normals,
                    texcoords,
                    indices,
                }
            })
            .collect();

        Ok(objs)
    }

    pub fn get_vertex_array(&self) -> Vec<Vertex> {
        (self.indices.iter().map(|idx| {
            let position = *self.vertices.get(*idx).unwrap();
            // let normal = *self.normals.get(*idx).unwrap();
            // let tex_cords = *self.texcoords.get(*idx).unwrap();
            Vertex::new(position, Vec3::new(0.0, 0.0, 1.0), Vec2::new(0.0, 0.0))
        }))
        .collect()
    }
}
