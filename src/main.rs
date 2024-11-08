use minifb::{Key, KeyRepeat, Window, WindowOptions};
use mouse_rs::Mouse;
use nalgebra_glm::Vec3;
use std::collections::VecDeque;
use std::f32::consts::PI;
use std::time::{Duration, Instant};
use three_d_rendering::camera::Camera;
use three_d_rendering::framebuffer;
use three_d_rendering::obj::load_objs;
use three_d_rendering::render::render;
use three_d_rendering::shader::{
    create_model_matrix, create_projection_matrix, create_view_matrix, create_viewport_matrix,
    Uniforms,
};
use three_d_rendering::{Message, Model};

const ZOOM_SPEED: f32 = 5.0;
const ROTATION_SPEED: f32 = PI / 20.0;

fn main() {
    let window_width = 1080;
    let window_height = 720;

    let framebuffer_width = 1080;
    let framebuffer_height = 720;

    let mut framebuffer = framebuffer::Framebuffer::new(framebuffer_width, framebuffer_height);

    let window_options = WindowOptions {
        resize: true,
        scale: minifb::Scale::FitScreen,
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
    render(&mut framebuffer, &data);

    let mut splash_timer = 0;
    let splash_delay = 300;

    let mode_cooldown = 5;
    let mut mode_cooldown_timer = 0;

    let last_recorded_frames_max_count = 60;
    let mut last_recorded_frames = VecDeque::with_capacity(last_recorded_frames_max_count);
    while window.is_open() {
        let mut should_update = false;
        let start = Instant::now();
        mode_cooldown_timer = (mode_cooldown_timer - 1).max(0);
        splash_timer = (splash_timer + 1).min(splash_delay + 1);

        // listen to inputs
        if window.is_key_down(Key::Escape) {
            break;
        }

        let messages: Vec<Message> = window
            .get_keys_pressed(KeyRepeat::Yes)
            .into_iter()
            .filter_map(|key| match key {
                Key::Left => Some(Message::RotateCamera(ROTATION_SPEED, 0.0)),
                Key::Right => Some(Message::RotateCamera(-ROTATION_SPEED, 0.0)),
                Key::Up => Some(Message::RotateCamera(0.0, -ROTATION_SPEED)),
                Key::Down => Some(Message::RotateCamera(0.0, ROTATION_SPEED)),

                Key::W => Some(Message::ZoomCamera(ZOOM_SPEED)),
                Key::S => Some(Message::ZoomCamera(-ZOOM_SPEED)),

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

        for msg in messages {
            data = update(data, msg);
        }

        if data.camera.has_changed() || should_update {
            framebuffer.clear();
            render(&mut framebuffer, &data);
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
        last_recorded_frames.push_back((end - start).as_millis());

        let avg_millis: f32 = last_recorded_frames.iter().map(|&u| u as f32).sum::<f32>()
            / last_recorded_frames_max_count as f32;
        let avg_frames = 1000.0 / avg_millis;
        window.set_title(format!("{} - {:.2} fps", title_prefix, avg_frames).as_ref());
        std::thread::sleep(frame_delay);
    }
}

/// Init the default state
fn init(window_dimensions: (usize, usize), framebuffer_dimensions: (usize, usize)) -> Model {
    // let mut args = env::args();
    // args.next();
    //
    // let asset_dir = args.next().expect("No asset directory received!");
    // println!("Reading assets from: {asset_dir}");

    let (framebuffer_height, framebuffer_width) = framebuffer_dimensions;
    let (window_width, window_height) = window_dimensions;
    let obj = load_objs("Ship.obj").unwrap();

    let camera = Camera::new(
        Vec3::new(0.0, 0.0, 10.0),
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
    );

    let scale = 1.0;
    let rotation = Vec3::zeros();
    let translation = Vec3::new(0.0, 0.0, 0.0);

    Model {
        objs: obj,
        uniforms: Uniforms {
            model_matrix: create_model_matrix(translation, scale, rotation),
            view_matrix: create_view_matrix(camera.eye, camera.center, camera.up),
            projection_matrix: create_projection_matrix(window_width as f32, window_height as f32),
            viewport_matrix: create_viewport_matrix(
                framebuffer_width as f32,
                framebuffer_height as f32,
            ),
        },
        scale,
        rotation,
        translation,
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

            camera.rotate_cam(delta_yaw, delta_pitch);
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
    }
}
