use rndr_core::default_components::render::MeshRenderable;
use rndr_core::default_components::Transform;
use rndr_core::default_objects;
use rndr_core::events::{Event, Keycode};
use rndr_core::prelude::{Instance, Object};

use rndr_math::prelude::*;

use rndr_phys::components::MeshCollider;

const HEIGHT: u32 = 500;
const WIDTH: u32 = 1000;

const BUFF_HEIGHT: u32 = 200;
const BUFF_WIDTH: u32 = 400;

fn main() {
    let mut instance =
        Instance::init(WIDTH, HEIGHT, BUFF_WIDTH, BUFF_HEIGHT).expect("Could not init rndr");

    instance.configure_mesh_rendering_system();

    unsafe { CAMERA_ID = instance.register_object(default_objects::camera(true)) };

    let mut teapot =
        default_objects::mesh_from_file("../Utah_teapot_(solid).stl").expect("Could not load mesh");
    teapot.add_component(MeshCollider::default().into());
    instance.register_object(teapot);

    let mut timer = std::time::Instant::now();
    let mut frames = 0;
    loop {
        handle_fps(&mut timer, &mut frames);

        let poll: Vec<_> = instance.event_pump.poll_iter().collect();

        for event in poll {
            handle_input_event(event, &mut instance);
        }
        instance.render().expect("Could not render");
        instance.apply_render().expect("Could not apply render");
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

static mut CAMERA_ID: u64 = 0;

fn handle_input_event(event: Event, instance: &mut Instance) {
    const INCREASE_ROTATION: f32 = 0.08;
    const INCREASE_ROTATION_KEY: f32 = 10.0;
    const INCREASE_POSITION: f32 = 0.2;

    let cam_obj = instance.get_object_mut(unsafe { CAMERA_ID });

    let cam_transform = cam_obj.component_mut::<Transform>();

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
                Keycode::Backspace => {
                    let ray = rndr_phys::ray::Ray {
                        start: cam_transform.position,
                        dir: cam_transform.fwd(),
                        max_distance: None,
                        objects: &instance.object_manager,
                    };

                    let out = ray.cast();

                    if let Some(hit) = out {
                        println!("{hit:?}");
                        let transform = Transform {
                            position: hit.position,
                            rotation: V3::default(),
                        };

                        let mesh = MeshRenderable::plane();
                        let mut obj = Object::new();
                        obj.add_component(Box::new(transform));
                        obj.add_component(Box::new(mesh));

                        instance.register_object(obj);
                    } else {
                        println!("No hit");
                    }
                }
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
                    cam_transform.rotation.z -= INCREASE_ROTATION_KEY;
                }
                Keycode::Right => {
                    cam_transform.rotation.z += INCREASE_ROTATION_KEY;
                }
                Keycode::Up => {
                    cam_transform.rotation.y -= INCREASE_ROTATION_KEY;
                }
                Keycode::Down => {
                    cam_transform.rotation.y += INCREASE_ROTATION_KEY;
                }
                _ => (),
            };
        }
        _ => (),
    };
}
