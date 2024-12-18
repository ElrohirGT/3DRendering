use fastnoise_lite::FastNoiseLite;
use minifb::{Key, KeyRepeat, Window, WindowOptions};
use mouse_rs::Mouse;
use nalgebra_glm::{vec3, Vec3};
use std::collections::VecDeque;
use std::f32::consts::PI;
use std::time::{Duration, Instant};
use three_d_rendering::blenders::BlendMode;
use three_d_rendering::camera::Camera;
use three_d_rendering::color::Color;
use three_d_rendering::obj::load_objs;
use three_d_rendering::planets::{
    create_disco_planet, create_face_planet, create_gas_giant, create_green_planet,
    create_ocean_planet, create_snow_planet, create_sun,
};
use three_d_rendering::render::render;
use three_d_rendering::shader::{
    create_model_matrix, create_noise, create_projection_matrix, create_view_matrix,
    create_viewport_matrix, ShaderType, Uniforms,
};
use three_d_rendering::{framebuffer, Entity};
use three_d_rendering::{Message, Model};

const ZOOM_SPEED: f32 = 1.0;
const ROTATION_SPEED: f32 = PI / 20.0;

fn main() {
    let window_width = 1080;
    let window_height = 720;

    let framebuffer_width = 1000;
    let framebuffer_height =
        (window_height as f32 / window_width as f32) * framebuffer_width as f32;
    let framebuffer_height = framebuffer_height as usize;

    println!("Framebuffer: ({framebuffer_width}, {framebuffer_height})");

    let mut framebuffer = framebuffer::Framebuffer::new(framebuffer_width, framebuffer_height);

    let window_options = WindowOptions {
        // resize: true,
        // scale: minifb::Scale::FitScreen,
        ..WindowOptions::default()
    };

    let title_prefix = "3D Rendering";
    let mut window =
        Window::new(title_prefix, window_width, window_height, window_options).unwrap();
    window.set_key_repeat_delay(0.01);
    window.set_cursor_visibility(true);
    let mouse = Mouse::new();

    let target_framerate = 60;
    let frame_delay = Duration::from_millis(1000 / target_framerate);

    let mut data = init(
        (window_width, window_height),
        (framebuffer_width, framebuffer_height),
    );
    let mut noise = FastNoiseLite::with_seed(1506);
    noise.set_frequency(Some(0.004));
    render(&mut framebuffer, &data, &mut noise);

    let mut splash_timer = 0;
    let splash_delay = 300;

    let mode_cooldown = 5;
    let mut mode_cooldown_timer = 0;

    let last_recorded_frames_max_count = 60;
    let mut last_recorded_frames = VecDeque::with_capacity(last_recorded_frames_max_count);
    let mut time = 0.0;
    while window.is_open() {
        let mut should_update = false;
        let start = Instant::now();
        mode_cooldown_timer = (mode_cooldown_timer - 1).max(0);
        splash_timer = (splash_timer + 1).min(splash_delay + 1);

        // listen to inputs
        if window.is_key_down(Key::Escape) {
            break;
        }

        let mut messages: Vec<Message> = window
            .get_keys_pressed(KeyRepeat::Yes)
            .into_iter()
            .filter_map(|key| match key {
                Key::Left => Some(Message::RotateCamera(ROTATION_SPEED, 0.0)),
                Key::Right => Some(Message::RotateCamera(-ROTATION_SPEED, 0.0)),
                Key::Up => Some(Message::RotateCamera(0.0, -ROTATION_SPEED)),
                Key::Down => Some(Message::RotateCamera(0.0, ROTATION_SPEED)),

                Key::W => Some(Message::ZoomCamera(ZOOM_SPEED)),
                Key::S => Some(Message::ZoomCamera(-ZOOM_SPEED)),

                Key::Key1 => Some(Message::ChangePlanet(create_disco_planet())),
                Key::Key2 => Some(Message::ChangePlanet(create_ocean_planet())),
                Key::Key3 => Some(Message::ChangePlanet(create_gas_giant())),
                Key::Key4 => Some(Message::ChangePlanet(create_face_planet())),
                Key::Key5 => Some(Message::ChangePlanet(create_snow_planet())),
                Key::Key6 => Some(Message::ChangePlanet(create_sun())),
                Key::Key7 => Some(Message::ChangePlanet(create_green_planet())),

                // Key::Tab => {
                //     should_update = true;
                //     Some(match data.daytime {
                //         three_d_rendering::TimeOfDay::Day => Message::TimeToNight,
                //         three_d_rendering::TimeOfDay::Night => Message::TimeToDay,
                //     })
                // }

                // Key::Space => match (mode_cooldown_timer, &data.status) {
                //     (0, GameStatus::MainMenu) => {
                //         mode_cooldown_timer = mode_cooldown;
                //         Some(Message::StartGame)
                //     }
                //     _ => None,
                // },
                // Key::R => match (mode_cooldown_timer, &data.status) {
                //     (0, GameStatus::YouLost) | (0, GameStatus::YouWon) => {
                //         mode_cooldown_timer = mode_cooldown;
                //         Some(Message::RestartGame)
                //     }
                //     _ => None,
                // },
                _ => None,
            })
            .collect();
        should_update = true;
        messages.push(Message::UpdateTime(time));

        for msg in messages {
            data = update(data, msg);
        }

        if data.camera.has_changed() || should_update {
            framebuffer.clear();
            render(&mut framebuffer, &data, &mut noise);
        }
        data.camera.reset_change();

        // Update the window with the framebuffer contents
        window
            .update_with_buffer(&framebuffer.buffer, framebuffer_width, framebuffer_height)
            .expect("Couldn't update the framebuffer!");
        let end = Instant::now();
        if last_recorded_frames.len() == last_recorded_frames_max_count {
            last_recorded_frames.pop_front();
        }
        let render_millis = (end - start).as_millis();
        last_recorded_frames.push_back(render_millis);
        time += render_millis as f32;

        let avg_millis: f32 = last_recorded_frames.iter().map(|&u| u as f32).sum::<f32>()
            / last_recorded_frames_max_count as f32;
        let avg_frames = 1000.0 / avg_millis;
        window.set_title(format!("{} - {:.2} fps", title_prefix, avg_frames).as_ref());
        std::thread::sleep(frame_delay);
    }
}

/// Init the default state
fn init(window_dimensions: (usize, usize), framebuffer_dimensions: (usize, usize)) -> Model {
    let (framebuffer_height, framebuffer_width) = framebuffer_dimensions;
    let (window_width, window_height) = window_dimensions;

    let start_planet = create_green_planet();

    let render_entities = vec![start_planet];
    // let render_entities = vec![];
    let entities = vec![];

    let camera = Camera::new(
        Vec3::new(0.0, 0.0, 10.0),
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
    );

    let scale = 1.0;
    let rotation = Vec3::zeros();
    // let translation = Vec3::zeros();
    let translation = vec3(0.0, 0.0, 0.0);

    Model {
        entities,
        render_entities,
        uniforms: Uniforms {
            view_matrix: create_view_matrix(camera.eye, camera.center, camera.up),
            projection_matrix: create_projection_matrix(window_width as f32, window_height as f32),
            viewport_matrix: create_viewport_matrix(
                framebuffer_width as f32,
                framebuffer_height as f32,
            ),
            time: 0.0,
        },
        rotation,
        translation,
        scale,
        camera,
    }
}

fn update(data: Model, msg: Message) -> Model {
    match msg {
        Message::RotateCamera(delta_yaw, delta_pitch) => {
            let Model {
                rotation,
                mut camera,
                uniforms,
                ..
            } = data;

            camera.orbit(delta_yaw, delta_pitch);
            let uniforms = Uniforms {
                view_matrix: create_view_matrix(camera.eye, camera.center, camera.up),
                ..uniforms
            };

            Model {
                rotation,
                uniforms,
                camera,
                ..data
            }
        }
        Message::ZoomCamera(delta_zoom) => {
            let Model {
                scale,
                mut camera,
                uniforms,
                ..
            } = data;

            camera.zoom(delta_zoom);
            let uniforms = Uniforms {
                view_matrix: create_view_matrix(camera.eye, camera.center, camera.up),
                ..uniforms
            };

            Model {
                uniforms,
                scale,
                camera,
                ..data
            }
        }
        Message::UpdateTime(time) => {
            let Model { uniforms, .. } = data;

            let uniforms = Uniforms { time, ..uniforms };

            Model { uniforms, ..data }
        }
        Message::ChangePlanet(entity) => {
            let render_entities = vec![entity];

            Model {
                render_entities,
                ..data
            }
        }
    }
}
