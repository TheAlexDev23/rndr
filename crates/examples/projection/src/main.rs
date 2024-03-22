use rndr_core::events::{Event, EventPump, Keycode};
use rndr_core::pixel::PixelGrid;

use rndr_math::prelude::*;

use lazy_static::lazy_static;

lazy_static! {
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

    /*
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
    */
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
                    CAM_POS.z += INCREASE;
                },
                Keycode::Q => unsafe {
                    CAM_POS.z -= INCREASE;
                },
                Keycode::W => unsafe {
                    CAM_POS.x += INCREASE;
                },
                Keycode::S => unsafe {
                    CAM_POS.x -= INCREASE;
                },
                Keycode::A => unsafe {
                    CAM_POS.y += INCREASE;
                },
                Keycode::D => unsafe {
                    CAM_POS.y -= INCREASE;
                },
                Keycode::Left => unsafe {
                    CAM_ROT.z += INCREASE;
                },
                Keycode::Right => unsafe {
                    CAM_ROT.z -= INCREASE;
                },
                Keycode::Up => unsafe {
                    CAM_ROT.y += INCREASE;
                },
                Keycode::Down => unsafe {
                    CAM_ROT.y -= INCREASE;
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

static mut CAM_ROT: V3 = V3 {
    y: 90.0,
    z: 270.0,
    x: 0.0,
};
static mut CAM_POS: V3 = V3 {
    x: 0.0,
    y: 0.0,
    z: 0.0,
};

static mut HIDE_Z: bool = false;

fn update(pixel_grid: &mut PixelGrid) {
    let (cam_x, cam_y, cam_z) = unsafe { (CAM_ROT.x, CAM_ROT.y, CAM_ROT.z) };

    let cos_x = cam_x.to_radians().cos();
    let cos_y = cam_y.to_radians().cos();
    let cos_z = cam_z.to_radians().cos();

    let sin_x = cam_x.to_radians().sin();
    let sin_y = cam_y.to_radians().sin();
    let sin_z = cam_z.to_radians().sin();

    let world_to_screen_matrix = M3x3::new([
        V3::new(
            -1f32 * cos_z * sin_y * sin_x + sin_z * cos_x, // right
            cos_z * cos_y,                                 // up
            cos_z * sin_y * cos_x + sin_z * sin_x,         // fwd
        ),
        V3::new(
            -1f32 * sin_z * sin_y * sin_x - cos_z * cos_x, // right
            -1f32 * sin_z * cos_y,                         // up
            sin_z * sin_y * cos_y - cos_z * sin_x,         // fwd
        ),
        V3::new(
            -1f32 * cos_y * sin_x, // right
            sin_y,                 // up
            -1f32 * cos_y * cos_x, // fwd
        ),
    ]);

    for point in POINTS.iter() {
        let point = (point.0.relative_to(unsafe { &CAM_POS }), point.1);
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
