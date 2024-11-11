use nalgebra_glm::{Mat4, Vec3};

use crate::{
    blenders::BlendMode,
    color::Color,
    obj::load_objs,
    shader::{create_model_matrix, ShaderType},
    Entity,
};

pub fn create_default_planet_model_matrix() -> Mat4 {
    create_model_matrix(Vec3::zeros(), 1.0, Vec3::zeros())
}

pub fn create_disco_planet() -> Entity {
    let planet_obj = load_objs("sphere.obj").unwrap();
    let shaders = vec![
        (
            ShaderType::MovingStripes {
                speed: 1e-3,
                stripe_width: 0.1,
            },
            vec![Color::pink(), Color::green()],
            BlendMode::Replace,
        ),
        (
            ShaderType::MovingStripes {
                speed: 1e-4,
                stripe_width: 0.1,
            },
            vec![Color::black(), Color::blue()],
            BlendMode::Normal,
        ),
        (ShaderType::Intensity, vec![], BlendMode::Replace),
    ];

    Entity {
        objs: planet_obj,
        model_matrix: create_default_planet_model_matrix(),
        shaders,
    }
}

pub fn create_ocean_planet() -> Entity {
    let planet_obj = load_objs("sphere.obj").unwrap();
    let shaders = vec![
        (
            ShaderType::MovingStripes {
                speed: 1e-4,
                stripe_width: 0.1,
            },
            vec![Color::new(0, 0, 240), Color::blue()],
            BlendMode::Replace,
        ),
        (
            ShaderType::CloudShader,
            vec![Color::black(), Color::green()],
            BlendMode::Screen,
        ),
        (ShaderType::Intensity, vec![], BlendMode::Replace),
    ];

    Entity {
        objs: planet_obj,
        model_matrix: create_default_planet_model_matrix(),
        shaders,
    }
}
