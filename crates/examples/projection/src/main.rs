use std::collections::HashMap;

use rndr_core::events::{Event, EventPump, Keycode};
use rndr_core::pixel::PixelGrid;

use rndr_math::prelude::*;

use lazy_static::lazy_static;

lazy_static! {
    static ref SHAPES: Vec<Shape> = vec![Shape {
        transform: Transform {
            position: V3 {
                x: 0.0,
                y: 1.0,
                z: 0.0
            },
            rotation: V3 {
                x: 0.0,
                y: 0.0,
                z: 0.0
            }
        },
        vertices: vec![
            V3 {
                x: 2.5,
                y: 1.0,
                z: 0.0,
            },
            V3 {
                x: 2.5,
                y: 1.0,
                z: 5.0,
            },
            V3 {
                x: -2.5,
                y: 1.0,
                z: 5.0,
            },
            V3 {
                x: -2.5,
                y: 1.0,
                z: 0.0,
            },
        ],
        triangles: vec![0, 1, 2, 0, 3, 2]
    }];
}
const HEIGHT: u32 = 500;
const WIDTH: u32 = 1000;

const BUFF_HEIGHT: u32 = 100;
const BUFF_WIDTH: u32 = 200;

fn main() {
    let mut instance = rndr_core::Instance::init(WIDTH, HEIGHT, BUFF_WIDTH, BUFF_HEIGHT)
        .expect("Could not init rndr");
    loop {
        update(&mut instance.pixel_grid);
        //1: (95, 40), 0: (95, 50) 2: (105, 40), 3: (105, 50
        // instance.pixel_grid.line((95, 40), (95, 50), [255, 255, 255]);
        // instance.pixel_grid.line((105, 40), (105, 50), [255, 255, 255]);
        // instance .pixel_grid .line((25, 15), (26, 50), [255, 255, 255]); instance .pixel_grid .line((25, 35), (15, 49), [255, 255, 255]); input(&mut instance.event_pump);
        // instance.pixel_grid.line((25, 0), (15, 15), [255, 255, 255]);
        // instance.pixel_grid.line((35, 0), (35, 15), [255, 255, 255]);
        // instance.pixel_grid.line((0, 0), (15, 25), [255, 255, 255]);
        input(&mut instance.event_pump);
        instance.render().expect("Could not render");
        instance.pixel_grid.clear()
    }
}

fn input(event_pump: &mut EventPump) {
    const INCREASE_ROTATION: f32 = 0.5;
    const INCREASE_POSITION: f32 = 0.2;
    for event in event_pump.poll_iter() {
        match event {
            Event::Quit { timestamp: _ } => {
                panic!("Exit requested");
            }
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

fn update(pixel_grid: &mut PixelGrid) {
    let cam_transform = unsafe { &CAM_TRANSFORM };
    let cam_fwd = cam_transform.fwd();
    let cam_right = cam_transform.right();
    let cam_up = cam_transform.up();

    let world_to_screen_matrix = M3x3::new([
        V3::new(cam_right.x, cam_up.x, cam_fwd.x),
        V3::new(cam_right.y, cam_up.y, cam_fwd.y),
        V3::new(cam_right.z, cam_up.z, cam_fwd.z),
    ]);

    for shape in SHAPES.iter() {
        let mut screen_points = HashMap::new();

        for i in 0..shape.vertices.len() {
            let point = shape.vertices[i].relative_to(unsafe { &CAM_TRANSFORM.position });
            let mut px = world_to_screen_matrix * point;
            px.x /= px.z / 2.0;
            px.y /= px.z / 2.0;

            let screen_x = px.x.round() as i32 + (BUFF_WIDTH / 2) as i32;
            let screen_y = (BUFF_HEIGHT / 2) as i32 + px.y.round() as i32;

            let (psx, psy) = (screen_x, screen_y);

            let (screen_x, screen_y) = (screen_x as u32, screen_y as u32);
            screen_points.insert(i, (screen_x, screen_y));

            /*
            if screen_x == 4294967282 || screen_y == 4294967282 {
                println!("{point} -> ({}, {}, {}) -> ({psx}, {psy})-> ({screen_x}, {screen_y})\n{world_to_screen_matrix:.5?}\n", px.x, px.y, px.z);
            }
            */
        }

        let mut i = 0;
        while i < shape.triangles.len() {
            let first = screen_points[&shape.triangles[i]];
            let second = screen_points[&shape.triangles[i + 1]];
            let third = screen_points[&shape.triangles[i + 2]];

            pixel_grid.line(first, second, [255, 255, 255]);
            pixel_grid.line(second, third, [255, 255, 255]);
            pixel_grid.line(third, first, [255, 255, 255]);
            i += 3;
        }

        // println!("{screen_points:?}\n{world_to_screen_matrix:?}\n");
    }
}
