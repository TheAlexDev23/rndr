use rndr_core::events::{Event, Keycode};
use rndr_core::prelude::{Instance, Object};

use rndr_core::scene::object::Vertex;
use rndr_math::prelude::*;

use lazy_static::lazy_static;

lazy_static! {
    static ref SQUARE: Vec<Vertex> = vec![
        Vertex::new_with_color(V3::new(
             2.5,
             0.0,
             0.0,
        ), [255, 255, 255]),
        Vertex::new_with_color(V3::new(
            2.5,
            0.0,
            5.0,
        ), [255, 0, 255]),
        Vertex::new_with_color(V3::new(
             -2.5,
             0.0,
             5.0,
        ), [255, 255, 0]),
        Vertex::new_with_color(V3::new(
            -2.5,
            0.0,
            0.0,
        ), [255, 0, 0]),
    ];

    static ref SHAPES: Vec<Object> = vec![
        /*
        Object {
            transform: Transform {
                position: V3::new(0.0, 2.5, 0.0),
                rotation: V3::new(0.0, 0.0, 0.0)
            },
            vertices: SQUARE.clone(),
            triangles: vec![0, 1, 2, 0, 3, 2]
        },
        */
        Object {
            transform: Transform {
                position: V3::new(2.5, 0.0, 0.0),
                rotation: V3::new(0.0, 0.0, 90.0),
            },
            vertices: SQUARE.clone(),
            triangles: vec![0, 1, 2, 0, 3, 2]
        },
        /*
        Object {
            transform: Transform {
                position: V3::new(0.0, -2.5, 0.0),
                rotation: V3::new(0.0, 0.0, 0.0),
            },
            vertices: SQUARE.clone(),
            triangles: vec![0, 1, 2, 0, 3, 2]
        },
        Object {
            transform: Transform {
                position: V3::new(-2.5, 0.0, 0.0),
                rotation: V3::new(0.0, 0.0, 90.0),
            },
            vertices: SQUARE.clone(),
            triangles: vec![0, 1, 2, 0, 3, 2]
        }
        */
    ];
}

const HEIGHT: u32 = 500;
const WIDTH: u32 = 1000;

const BUFF_HEIGHT: u32 = 200;
const BUFF_WIDTH: u32 = 400;

fn main() {
    let mut instance =
        Instance::init(WIDTH, HEIGHT, BUFF_WIDTH, BUFF_HEIGHT).expect("Could not init rndr");

    for object in SHAPES.iter() {
        instance.register_object(object.clone());
    }

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
    const INCREASE_ROTATION: f32 = 0.01;
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
