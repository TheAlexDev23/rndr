use rndr_core::events::{Event, Keycode};
use rndr_core::prelude::Instance;

use rndr_core::render::FragShader;
use rndr_core::scene::Object;

pub struct ZShader;

impl FragShader for ZShader {
    fn frag(&self, data: &mut rndr_core::prelude::FragData) {
        let color = (255.0 * data.output_pixel().0) as u8;
        data.output_pixel_mut().1 = [color, color, color]
    }
}

const HEIGHT: u32 = 500;
const WIDTH: u32 = 1000;

const BUFF_HEIGHT: u32 = 200;
const BUFF_WIDTH: u32 = 400;

fn main() {
    let mut instance =
        Instance::init(WIDTH, HEIGHT, BUFF_WIDTH, BUFF_HEIGHT).expect("Could not init rndr");

    let mut teapot =
        Object::from_stl("../../../../Utah_teapot_(solid).stl").expect("Could not load object");

    teapot.shader = Box::from(ZShader);

    instance.register_object(teapot);

    let mut timer = std::time::Instant::now();
    let mut frames = 0;
    loop {
        handle_fps(&mut timer, &mut frames);

        let poll: Vec<_> = instance.event_pump.poll_iter().collect();

        for event in poll {
            handle_input_event(event, &mut instance);
        }
        instance.render();
        instance.apply_render().expect("Could not render");
        frames += 1;
    }
}

fn handle_fps(timer: &mut std::time::Instant, frames: &mut i32) {
    if (std::time::Instant::now() - *timer).as_secs_f32() >= 1.0 {
        println!("FPS: {frames}");
        *timer = std::time::Instant::now();
        *frames = 0;
    }
}

fn handle_input_event(event: Event, instance: &mut Instance) {
    const INCREASE_ROTATION: f32 = 0.08;
    const INCREASE_POSITION: f32 = 0.2;
    let cam_transform = &mut instance.get_camera().transform;

    match event {
        Event::Quit { timestamp: _ } => {
            panic!("Exit requested");
        }
        Event::MouseMotion {
            timestamp: _,
            window_id: _,
            which: _,
            mousestate: _,
            x: _,
            y: _,
            xrel,
            yrel,
        } => {
            cam_transform.rotation.z += INCREASE_ROTATION * xrel as f32;
            cam_transform.rotation.y += INCREASE_ROTATION * yrel as f32;
        }
        Event::KeyDown {
            keycode: Some(keycode),
            ..
        } => {
            match keycode {
                Keycode::E => {
                    cam_transform.position += cam_transform.up() * INCREASE_POSITION;
                }
                Keycode::Q => {
                    cam_transform.position -= cam_transform.up() * INCREASE_POSITION;
                }
                Keycode::W => {
                    cam_transform.position += cam_transform.fwd() * INCREASE_POSITION;
                }
                Keycode::S => {
                    cam_transform.position -= cam_transform.fwd() * INCREASE_POSITION;
                }
                Keycode::A => {
                    cam_transform.position += cam_transform.right() * INCREASE_POSITION;
                }
                Keycode::D => {
                    cam_transform.position -= cam_transform.right() * INCREASE_POSITION;
                }
                Keycode::Left => {
                    cam_transform.rotation.z += INCREASE_ROTATION;
                }
                Keycode::Right => {
                    cam_transform.rotation.z -= INCREASE_ROTATION;
                }
                Keycode::Up => {
                    cam_transform.rotation.y += INCREASE_ROTATION;
                }
                Keycode::Down => {
                    cam_transform.rotation.y -= INCREASE_ROTATION;
                }
                _ => (),
            };
        }
        _ => (),
    };
}
