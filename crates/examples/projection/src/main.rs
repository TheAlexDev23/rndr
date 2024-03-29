use rndr_core::events::{Event, Keycode};
use rndr_core::prelude::{Instance, Object};

const HEIGHT: u32 = 500;
const WIDTH: u32 = 1000;

const BUFF_HEIGHT: u32 = 200;
const BUFF_WIDTH: u32 = 400;

fn main() {
    let mut instance =
        Instance::init(WIDTH, HEIGHT, BUFF_WIDTH, BUFF_HEIGHT).expect("Could not init rndr");

    instance.register_object(
        Object::from_stl("../../../Utah_teapot_(solid).stl").expect("Could not load object"),
    );

    let mut timer = std::time::Instant::now();
    let mut frames = 0;
    loop {
        handle_fps(&mut timer, &mut frames);

        let poll: Vec<_> = instance.event_pump.poll_iter().collect();

        for event in poll {
            handle_input_event(event, &mut instance);
        }
        instance.center_mouse();
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
    const INCREASE_ROTATION: f32 = 0.05;
    const INCREASE_POSITION: f32 = 0.2;
    let cam = &mut instance.get_camera();

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
            cam.transform.rotation.z += INCREASE_ROTATION * xrel as f32;
            cam.transform.rotation.y += INCREASE_ROTATION * yrel as f32;
        }
        Event::KeyDown {
            keycode: Some(keycode),
            ..
        } => {
            match keycode {
                Keycode::E => {
                    cam.transform.position += cam.transform.up() * INCREASE_POSITION;
                }
                Keycode::Q => {
                    cam.transform.position -= cam.transform.up() * INCREASE_POSITION;
                }
                Keycode::W => {
                    cam.transform.position += cam.transform.fwd() * INCREASE_POSITION;
                }
                Keycode::S => {
                    cam.transform.position -= cam.transform.fwd() * INCREASE_POSITION;
                }
                Keycode::A => {
                    cam.transform.position += cam.transform.right() * INCREASE_POSITION;
                }
                Keycode::D => {
                    cam.transform.position -= cam.transform.right() * INCREASE_POSITION;
                }
                Keycode::Left => {
                    cam.transform.rotation.z += INCREASE_ROTATION;
                }
                Keycode::Right => {
                    cam.transform.rotation.z -= INCREASE_ROTATION;
                }
                Keycode::Up => {
                    cam.display_surface_offset.as_mut().unwrap().z += 2.5;
                }
                Keycode::Down => {
                    cam.display_surface_offset.as_mut().unwrap().z -= 2.5;
                }
                _ => (),
            };
        }
        _ => (),
    };
}
