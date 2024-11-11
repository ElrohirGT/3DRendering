use std::f32::consts::PI;

use nalgebra_glm::{vec3, vec4, Mat4, Vec3};

use crate::{color::Color, fragment::Fragment, vertex::Vertex};

pub struct Uniforms {
    pub model_matrix: Mat4,
    pub view_matrix: Mat4,
    pub projection_matrix: Mat4,
    pub viewport_matrix: Mat4,
    pub time: f32,
}

pub fn vertex_shader(vertex: &Vertex, uniforms: &Uniforms) -> Vertex {
    let Uniforms {
        model_matrix,
        view_matrix,
        projection_matrix,
        viewport_matrix,
        ..
    } = uniforms;

    let position = vec4(vertex.position.x, vertex.position.y, vertex.position.z, 1.0);
    let transformed = viewport_matrix * projection_matrix * view_matrix * model_matrix * position;
    // println!("{position:?} TURNED INTO {transformed:?}");

    let w = transformed.w;
    let transformed_position = vec3(transformed.x / w, transformed.y / w, transformed.z / w);

    // Transform normal
    let vertex_normal = vec4(vertex.normal.x, vertex.normal.y, vertex.normal.z, 1.0);
    let normal_matrix = uniforms
        .model_matrix
        .try_inverse()
        .unwrap_or(Mat4::identity())
        .transpose();
    let transformed_normal = normal_matrix * vertex_normal;
    let w = transformed_normal.w;
    let transformed_normal = vec3(
        transformed_normal.x / w,
        transformed_normal.y / w,
        transformed_normal.z / w,
    );
    // let transformed_normal = vertex.normal;
    // println!("{normal_matrix:?} -> {transformed_normal:?}");

    Vertex {
        position: transformed_position,
        normal: transformed_normal,
        tex_coords: vertex.tex_coords,
        color: vertex.color,
    }
}

pub fn fragment_shader(fragment: Fragment, uniforms: &Uniforms) -> Fragment {
    fragment
        // .apply(uniforms, stripes_shader)
        .apply(uniforms, moving_stripes)
    // .apply(uniforms, intensity_shader)
}

fn intensity_shader(fragment: Fragment, uniforms: &Uniforms) -> Fragment {
    let Fragment {
        color, intensity, ..
    } = fragment;

    let color = color * intensity;

    Fragment { color, ..fragment }
}

fn stripes_shader(fragment: Fragment, uniforms: &Uniforms) -> Fragment {
    let y = fragment.vertex_position.y;
    // let y = fragment.position.y as usize;

    let colors = [
        Color::new(255, 0, 0),
        Color::new(0, 255, 0),
        Color::new(0, 0, 255),
        Color::new(255, 255, 0),
    ];

    let stripe_width = 0.1;

    let stripe_idx = (y / stripe_width).abs() as usize % colors.len();
    let color = colors[stripe_idx];

    Fragment { color, ..fragment }
}

fn interesting_shader(fragment: Fragment, uniforms: &Uniforms) -> Fragment {
    let color1 = Color::red();
    let color2 = Color::green();
    let color3 = Color::blue();

    let x = fragment.vertex_position.x;
    let y = fragment.vertex_position.y;
    let frequency = 10.0;

    let wave1 = (x * 7.0 * frequency + y * 5.0 * frequency).sin() * 0.5 + 0.5;
    let wave2 = (x * 5.0 * frequency - y * 8.0 * frequency + PI / 3.0).sin() * 0.5 + 0.5;
    let wave3 = (y * 6.0 * frequency + x * 4.0 * frequency + 2.0 * PI / 3.0).sin() * 0.5 + 0.5;

    // TODO: Keep implementing...

    let color = color1
        .lerp(&color2, wave1)
        .lerp(&color3, wave2)
        .lerp(&color1, wave3);

    Fragment { color, ..fragment }
}

fn moving_stripes(fragment: Fragment, uniforms: &Uniforms) -> Fragment {
    let color1 = Color::new(255, 0, 0);
    let color2 = Color::new(0, 0, 255);

    let stripe_width = 0.2;
    let speed = 1e-4;

    let moving_y = fragment.vertex_position.y + uniforms.time * speed;

    let stripe_factor = ((moving_y / stripe_width) * PI).sin() * 0.5 + 0.5;
    let color = color1.lerp(&color2, stripe_factor);

    Fragment { color, ..fragment }
}

pub fn create_model_matrix(translation: Vec3, scale: f32, rotation: Vec3) -> Mat4 {
    let (sinx, cosx) = rotation.x.sin_cos();
    let (siny, cosy) = rotation.y.sin_cos();
    let (sinz, cosz) = rotation.z.sin_cos();

    #[rustfmt::skip]
    let rotation_x = Mat4::new(
        1.0,    0.0,    0.0,    0.0,
        0.0,    cosx,   -sinx,  0.0,
        0.0,    sinx,   cosx,   0.0,
        0.0,    0.0,    0.0,    1.0,
    );

    #[rustfmt::skip]
    let rotation_y = Mat4::new(
        cosy,   0.0,    siny,   0.0,
        0.0,    1.0,    0.0,    0.0,
        -siny,  0.0,    cosy,   0.0,
        0.0,    0.0,    0.0,    1.0,
    );

    #[rustfmt::skip]
    let rotation_z = Mat4::new(
        cosz,   -sinz,  0.0,    0.0,
        sinz,   cosz,   0.0,    0.0,
        0.0,    0.0,    1.0,    0.0,
        0.0,    0.0,    0.0,    1.0,
    );

    let rotation_matrix = rotation_z * rotation_y * rotation_x;

    #[rustfmt::skip]
    let matrix = Mat4::new(
        scale,  0.0,    0.0,    translation.x,
        0.0,    scale,  0.0,    translation.y,
        0.0,    0.0,    scale,  translation.z,
        0.0,    0.0,    0.0,    1.0,
    ) * rotation_matrix;

    matrix
}

pub fn create_view_matrix(eye: Vec3, center: Vec3, up: Vec3) -> Mat4 {
    nalgebra_glm::look_at(&eye, &center, &up)
}

pub fn create_projection_matrix(window_width: f32, window_height: f32) -> Mat4 {
    let fov = 45.0 * PI / 180.0;
    let aspect_ratio = window_width / window_height;
    let near = 0.1;
    let far = 1000.0;

    nalgebra_glm::perspective(fov, aspect_ratio, near, far)
}

pub fn create_viewport_matrix(framebuffer_width: f32, framebuffer_height: f32) -> Mat4 {
    #[rustfmt::skip]
    let matrix = Mat4::new(
        framebuffer_width / 2.0,    0.0,                        0.0,    framebuffer_width / 2.0,
        0.0,                        -framebuffer_height / 2.0,  0.0,    framebuffer_height / 2.0,
        0.0,                        0.0,                        1.0,    0.0,
        0.0,                        0.0,                        0.0,    1.0);

    matrix
}
