use crate::{blenders::BlendMode, color::Color, obj::load_objs, shader::ShaderType, Entity};

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
        shaders,
    }
}

pub fn create_star_planet() -> Entity {
    let planet_obj = load_objs("sphere.obj").unwrap();
    let shaders = vec![
        (
            ShaderType::MovingStripes {
                speed: 1e-3,
                stripe_width: 0.1,
            },
            vec![Color::new(170, 170, 170), Color::blue()],
            BlendMode::Replace,
        ),
        (
            ShaderType::MovingStripes {
                speed: 1e-4,
                stripe_width: 0.1,
            },
            vec![Color::black(), Color::green()],
            BlendMode::Normal,
        ),
        (ShaderType::Intensity, vec![], BlendMode::Replace),
    ];

    Entity {
        objs: planet_obj,
        shaders,
    }
}
