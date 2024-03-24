use std::collections::HashMap;

use rndr_core::events::{Event, EventPump, Keycode};
use rndr_core::pixel::PixelGrid;

use rndr_math::prelude::*;

use lazy_static::lazy_static;

lazy_static! {
    static ref SQUARE: Vec<V3> = vec![
        V3 {
            x: 2.5,
            y: 0.0,
            z: 0.0,
        },
        V3 {
            x: 2.5,
            y: 0.0,
            z: 5.0,
        },
        V3 {
            x: -2.5,
            y: 0.0,
            z: 5.0,
        },
        V3 {
            x: -2.5,
            y: 0.0,
            z: 0.0,
        }
    ];
    static ref SHAPES: Vec<Shape> = vec![
        Shape {
            transform: Transform {
                position: V3::new(0.0, 2.5, 0.0),
                rotation: V3::new(0.0, 0.0, 0.0)
            },
            vertices: SQUARE.clone(),
            triangles: vec![0, 1, 2, 0, 3, 2]
        },
        /*
        Shape {
            transform: Transform {
                position: V3::new(2.5, 0.0, 0.0),
                rotation: V3::new(0.0, 0.0, 90.0),
            },
            vertices: SQUARE.clone(),
            triangles: vec![0, 1, 2, 0, 3, 2]
        },
        Shape {
            transform: Transform {
                position: V3::new(0.0, -2.5, 0.0),
                rotation: V3::new(0.0, 0.0, 0.0),
            },
            vertices: SQUARE.clone(),
            triangles: vec![0, 1, 2, 0, 3, 2]
        },
        Shape {
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
    let mut instance = rndr_core::Instance::init(WIDTH, HEIGHT, BUFF_WIDTH, BUFF_HEIGHT)
        .expect("Could not init rndr");

    let mut timer = std::time::Instant::now();
    let mut frames = 0;
    loop {
        handle_fps(&mut timer, &mut frames);
        input(&mut instance.event_pump);
        instance.center_mouse();
        update(&mut instance.pixel_grid);
        instance.render().expect("Could not render");
        instance.pixel_grid.clear();
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

fn input(event_pump: &mut EventPump) {
    const INCREASE_ROTATION: f32 = 0.1;
    const INCREASE_POSITION: f32 = 0.2;
    for event in event_pump.poll_iter() {
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
            } => unsafe {
                CAM_TRANSFORM.rotation.z += INCREASE_ROTATION * xrel as f32;
                CAM_TRANSFORM.rotation.y += INCREASE_ROTATION * yrel as f32;
            },
            Event::KeyDown {
                keycode: Some(keycode),
                ..
            } => match keycode {
                Keycode::E => unsafe {
                    CAM_TRANSFORM.position += CAM_TRANSFORM.up() * INCREASE_POSITION;
                },
                Keycode::Q => unsafe {
                    CAM_TRANSFORM.position -= CAM_TRANSFORM.up() * INCREASE_POSITION;
                },
                Keycode::W => unsafe {
                    CAM_TRANSFORM.position += CAM_TRANSFORM.fwd() * INCREASE_POSITION;
                },
                Keycode::S => unsafe {
                    CAM_TRANSFORM.position -= CAM_TRANSFORM.fwd() * INCREASE_POSITION;
                },
                Keycode::A => unsafe {
                    CAM_TRANSFORM.position += -1.0 * CAM_TRANSFORM.right() * INCREASE_POSITION;
                },
                Keycode::D => unsafe {
                    CAM_TRANSFORM.position += CAM_TRANSFORM.right() * INCREASE_POSITION;
                },
                Keycode::Left => unsafe {
                    CAM_TRANSFORM.rotation.z += INCREASE_ROTATION;
                },
                Keycode::Right => unsafe {
                    CAM_TRANSFORM.rotation.z -= INCREASE_ROTATION;
                },
                Keycode::Up => unsafe {
                    CAM_TRANSFORM.rotation.y += INCREASE_ROTATION;
                },
                Keycode::Down => unsafe {
                    CAM_TRANSFORM.rotation.y -= INCREASE_ROTATION;
                },
                _ => (),
            },
            _ => (),
        }
    }
}

static mut CAM_TRANSFORM: Transform = Transform {
    rotation: V3 {
        y: 0.0,
        z: 270.0,
        x: 0.0,
    },
    position: V3 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    },
};

const NEAR_PLANE: f32 = 0.0;

fn update(pixel_grid: &mut PixelGrid) {
    let cam_transform = unsafe { &CAM_TRANSFORM };
    let cam_fwd = cam_transform.fwd();
    let cam_right = cam_transform.right();
    let cam_up = cam_transform.up();

    let projection_matrix = M3x3::new([
        V3::new(cam_right.x, cam_up.x, cam_fwd.x),
        V3::new(cam_right.y, cam_up.y, cam_fwd.y),
        V3::new(cam_right.z, cam_up.z, cam_fwd.z),
    ]);

    for shape in SHAPES.iter() {
        let mut cached_screen_points = HashMap::new();

        let mut i = 0;
        while i < shape.triangles.len() {
            let first = shape.triangles[i];
            let second = shape.triangles[i + 1];
            let third = shape.triangles[i + 2];

            let first = project_point(&mut cached_screen_points, &shape, first, &projection_matrix);
            let second = project_point(
                &mut cached_screen_points,
                &shape,
                second,
                &projection_matrix,
            );
            let third = project_point(&mut cached_screen_points, &shape, third, &projection_matrix);
            if first.2 >= NEAR_PLANE && second.2 >= NEAR_PLANE {
                pixel_grid.line((first.0, first.1), (second.0, second.1), [255, 255, 255]);
            }
            if second.2 >= NEAR_PLANE && third.2 >= NEAR_PLANE {
                pixel_grid.line((second.0, second.1), (third.0, third.1), [255, 255, 255]);
            }
            if third.2 >= NEAR_PLANE && first.2 >= NEAR_PLANE {
                pixel_grid.line((third.0, third.1), (first.0, first.1), [255, 255, 255]);
            }
            i += 3;
        }
    }
}

const Z_ZERO_THRESHOLD: f32 = 0.01;
const PERSPECTIVE: bool = true;

const BOGUS_X: f32 = 0.0;
const BOGUS_Y: f32 = 0.0;
// const BOGUS_Z: f32 = -15.0;
const BOGUS_Z: f32 = -15.0;

lazy_static! {
    static ref DISPLAY_SURFACE_REL_PINHOLE: V3 = V3::new(BOGUS_X, BOGUS_Y, BOGUS_Z);
}

fn project_point(
    cache: &mut HashMap<usize, (i32, i32, f32)>,
    shape: &Shape,
    index: usize,
    projection_matrix: &M3x3,
) -> (i32, i32, f32) {
    if cache.contains_key(&index) {
        return cache[&index];
    }

    let mut point = shape.vertices[index];
    point.rotate(shape.transform.rotation);
    point += shape.transform.position;

    let point = point.relative_to(unsafe { &CAM_TRANSFORM.position });

    let mut px = *projection_matrix * point;

    if PERSPECTIVE && px.z > Z_ZERO_THRESHOLD {
        px.x = DISPLAY_SURFACE_REL_PINHOLE.z / px.z * px.x + DISPLAY_SURFACE_REL_PINHOLE.x;
        px.y = DISPLAY_SURFACE_REL_PINHOLE.z / px.z * px.y + DISPLAY_SURFACE_REL_PINHOLE.y;
    }

    let screen_x = px.x.round() as i32 + (BUFF_WIDTH / 2) as i32;
    let screen_y = (BUFF_HEIGHT / 2) as i32 + px.y.round() as i32;

    let ret = (screen_x, screen_y, px.z);

    cache.insert(index, ret);
    ret
}
