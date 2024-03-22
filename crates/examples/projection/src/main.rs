use rndr_core::events::{Event, EventPump, Keycode};
use rndr_core::pixel::PixelGrid;

use rndr_math::prelude::*;

use lazy_static::lazy_static;

lazy_static! {
    /*
    // Square on +y
    static ref POINTS: Vec<(V3, (u8, u8, u8))> = vec![
        (
            V3 {
                x: 2.5,
                y: 1.0,
                z: 0.0,
            },
            (255, 255, 255)
        ),
        (
            V3 {
                x: -2.5,
                y: 1.0,
                z: 0.0,
            },
            (255, 255, 0)
        ),
        (
            V3 {
                x: 2.5,
                y: 1.0,
                z: 5.0,
            },
            (255, 0, 255)
        ),
        (
            V3 {
                x: -2.5,
                y: 1.0,
                z: 5.0,
            },
            (0, 255, 255)
        ),
    ];
    */

    static ref POINTS: Vec<(V3, (u8, u8, u8))> = vec![
        (
            V3 {
                x: 25.0,
                y: 0.0,
                z: 0.0,
            },
            (255, 255, 255)
        ),
        (
            V3 {
                x: 0.0,
                y: 25.0,
                z: 0.0,
            },
            (255, 255, 0)
        ),
        (
            V3 {
                x: -25.0,
                y: 0.0,
                z: 0.0,
            },
            (255, 0, 255)
        ),
        (
            V3 {
                x: 0.0,
                y: -25.0,
                z: 0.0,
            },
            (0, 255, 255)
        ),
    ];
}

const HEIGHT: u32 = 500;
const WIDTH: u32 = 1000;

const BUFF_HEIGHT: u32 = 50;
const BUFF_WIDTH: u32 = 100;

fn main() {
    let mut instance = rndr_core::Instance::init(WIDTH, HEIGHT, BUFF_WIDTH, BUFF_HEIGHT)
        .expect("Could not init rndr");
    loop {
        update(&mut instance.pixel_grid);
        input(&mut instance.event_pump);
        instance.render().expect("Could not render");
        instance.pixel_grid.clear()
    }
}

fn input(event_pump: &mut EventPump) {
    const INCREASE: f32 = 2.0;
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
                    CAM_TRANSFORM.position += CAM_TRANSFORM.up() * INCREASE;
                },
                Keycode::Q => unsafe {
                    CAM_TRANSFORM.position -= CAM_TRANSFORM.up() * INCREASE;
                },
                Keycode::W => unsafe {
                    CAM_TRANSFORM.position += CAM_TRANSFORM.fwd() * INCREASE;
                },
                Keycode::S => unsafe {
                    CAM_TRANSFORM.position -= CAM_TRANSFORM.fwd() * INCREASE;
                },
                Keycode::A => unsafe {
                    CAM_TRANSFORM.position += CAM_TRANSFORM.right() * -1.0 * INCREASE;
                },
                Keycode::D => unsafe {
                    CAM_TRANSFORM.position += CAM_TRANSFORM.right() * INCREASE;
                },
                Keycode::Left => unsafe {
                    CAM_TRANSFORM.rotation.z += INCREASE;
                },
                Keycode::Right => unsafe {
                    CAM_TRANSFORM.rotation.z -= INCREASE;
                },
                Keycode::Up => unsafe {
                    CAM_TRANSFORM.rotation.y += INCREASE;
                },
                Keycode::Down => unsafe {
                    CAM_TRANSFORM.rotation.y -= INCREASE;
                },
                Keycode::H => unsafe {
                    HIDE_Z = !HIDE_Z;
                },
                _ => (),
            },
            _ => (),
        }
    }
}

static mut CAM_TRANSFORM: Transform = Transform {
    rotation: V3 {
        y: 90.0,
        z: 270.0,
        x: 0.0,
    },
    position: V3 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    },
};
static mut HIDE_Z: bool = false;

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

    for point in POINTS.iter() {
        let point = (
            point.0.relative_to(unsafe { &CAM_TRANSFORM.position }),
            point.1,
        );
        let px = world_to_screen_matrix * point.0;

        if unsafe { HIDE_Z } && px.z < 0f32 {
            continue;
        }

        // println!("Point {} => {px}", point.0);
        let screen_x = px.x.round() as i32 + (BUFF_WIDTH / 2) as i32;
        let screen_y = (BUFF_HEIGHT / 2) as i32 + px.y.round() as i32;

        if screen_x < 0 || screen_x >= BUFF_WIDTH as i32 {
            continue;
        }
        if screen_y < 0 || screen_y >= BUFF_HEIGHT as i32 {
            continue;
        }

        let (screen_x, screen_y) = (screen_x as u32, screen_y as u32);

        let px = pixel_grid.get_pixel(screen_x, screen_y);

        px[0] = point.1 .0;
        px[1] = point.1 .1;
        px[2] = point.1 .2;
    }
}
