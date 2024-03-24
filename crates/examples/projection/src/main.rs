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

const NEAR_PLANE: f32 = 0.1;

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

            if first.z <= NEAR_PLANE || second.z <= NEAR_PLANE || third.z <= NEAR_PLANE {
                i += 3;
                continue;
            }

            let first = (first.x, first.y);
            let second = (second.x, second.y);
            let third = (third.x, third.y);

            draw_triangle(first, second, third, pixel_grid);

            i += 3;
        }
    }
}

fn draw_triangle(
    first: (f32, f32),
    second: (f32, f32),
    third: (f32, f32),
    pixel_grid: &mut PixelGrid,
) {
    let total_triangle_area = triangle_area(first, second, third);

    let screen_width = (BUFF_WIDTH / 2) as f32;
    let screen_height = (BUFF_HEIGHT / 2) as f32;

    let x_start = first
        .0
        .min(second.0)
        .min(third.0)
        .max(-1.0 * screen_width)
        .round() as i32;

    let x_end = first.0.max(second.0).max(third.0).min(screen_width).round() as i32;

    let y_start = first
        .1
        .min(second.1)
        .min(third.1)
        .max(-1.0 * screen_height)
        .round() as i32;

    let y_end = first
        .1
        .max(second.1)
        .max(third.1)
        .min(screen_height)
        .round() as i32;

    for x in x_start..x_end {
        for y in y_start..y_end {
            let pt = (x as f32, y as f32);

            let comparing = triangle_area(pt, first, second)
                + triangle_area(pt, first, third)
                + triangle_area(pt, third, second);

            const EPILIPSON: f32 = 0.01;

            if (comparing - total_triangle_area).abs() <= EPILIPSON {
                let screen_x = x + screen_width as i32;
                let screen_y = screen_height as i32 + y;
                let px = pixel_grid.get_pixel(screen_x as u32, screen_y as u32);
                px[0] = 255;
                px[1] = 255;
                px[2] = 255;
            }
        }
    }
}

const Z_ZERO_THRESHOLD: f32 = 0.01;
const PERSPECTIVE: bool = true;

const BOGUS_X: f32 = 0.0;
const BOGUS_Y: f32 = 0.0;
const BOGUS_Z: f32 = -15.0;

lazy_static! {
    static ref DISPLAY_SURFACE_REL_PINHOLE: V3 = V3::new(BOGUS_X, BOGUS_Y, BOGUS_Z);
}

fn project_point(
    cache: &mut HashMap<usize, V3>,
    shape: &Shape,
    index: usize,
    projection_matrix: &M3x3,
) -> V3 {
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

    cache.insert(index, px);
    px
}

fn triangle_area(p1: (f32, f32), p2: (f32, f32), p3: (f32, f32)) -> f32 {
    0.5 * ((p1.0 * (p2.1 - p3.1) + p2.0 * (p3.1 - p1.1) + p3.0 * (p1.1 - p2.1)).abs())
}
