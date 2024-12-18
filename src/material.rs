use crate::color::Color;

#[derive(Debug, Clone)]
pub struct Material {
    pub diffuse: Color,
    pub specular: f32,
    /// La cantidad de luz que un material absorbe, en porcentaje (0,1).
    /// y también
    /// La cantidad de luz que un material refleja de la fuente, en porcentaje (0,1).
    pub albedo: (f32, f32),
    /// La cantidad de luz que depende del entorno en porcentaje (0,1).
    pub reflectivity: f32,
    /// La cantidad de luz que depende de lo que se encuentra detrás del objeto en porcentaje (0,1).
    pub transparency: f32,
    /// El índice refractivo del material.
    pub refractive_index: f32,
}

impl Material {
    pub const fn default() -> Self {
        Material {
            diffuse: Color::default(),
            specular: 0.0,
            albedo: (0.0, 0.0),
            reflectivity: 0.0,
            transparency: 0.0,
            refractive_index: 0.0,
        }
    }
}

pub const DIRT: Material = Material {
    diffuse: Color::pink(),
    specular: 1.0,
    albedo: (0.95, 0.05),
    reflectivity: 0.0,
    transparency: 0.0,
    refractive_index: 1.42,
};

pub const STONE: Material = Material {
    diffuse: Color::pink(),
    specular: 1.0,
    albedo: (0.95, 0.05),
    reflectivity: 0.0,
    transparency: 0.0,
    refractive_index: 1.42,
};

pub const WATER: Material = Material {
    diffuse: Color::pink(),
    specular: 100.0,
    albedo: (0.8, 0.1),
    reflectivity: 0.1,
    transparency: 0.9,
    refractive_index: 1.25,
};

pub const OBSIDIAN: Material = Material {
    diffuse: Color::pink(),
    specular: 100.0,
    albedo: (0.95, 0.05),
    reflectivity: 0.0,
    transparency: 0.0,
    refractive_index: 1.25,
};

pub const PORTAL: Material = Material {
    diffuse: Color::pink(),
    specular: 150.0,
    albedo: (0.95, 0.05),
    reflectivity: 0.0,
    transparency: 0.4,
    refractive_index: 1.2,
};

pub const NETHERRACK: Material = Material {
    diffuse: Color::pink(),
    specular: 100.0,
    albedo: (0.95, 0.05),
    reflectivity: 0.0,
    transparency: 0.0,
    refractive_index: 1.2,
};

pub const MAGMA: Material = Material {
    diffuse: Color::pink(),
    specular: 1.0,
    albedo: (0.95, 0.05),
    reflectivity: 0.0,
    transparency: 0.0,
    refractive_index: 1.2,
};

pub const GOLD: Material = Material {
    diffuse: Color::pink(),
    specular: 2.0,
    albedo: (0.5, 0.5),
    reflectivity: 0.1,
    transparency: 0.0,
    refractive_index: 1.2,
};

pub const RUBBER: Material = Material {
    diffuse: Color::pink(),
    specular: 1.0,
    albedo: (0.9, 0.1),
    reflectivity: 0.0,
    transparency: 0.0,
    refractive_index: 1.51,
};
