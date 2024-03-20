use rndr_core::pixel::PixelGrid;

use rndr_math::prelude::*;

use lazy_static::lazy_static;

lazy_static! {
    static ref POINTS: Vec<V3> = vec![
        V3 {
            x: 5f32,
            y: 1f32,
            z: 0f32,
        },
        V3 {
            x: -5f32,
            y: 1f32,
            z: 0f32,
        },
        V3 {
            x: 5f32,
            y: 1f32,
            z: 5f32,
        },
        V3 {
            x: -5f32,
            y: 1f32,
            z: 5f32,
        },
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
        instance.input();
        update(&mut instance.pixel_grid);
        instance.render().expect("Could not render");
        instance.pixel_grid.clear()
    }
}

static mut CAM_X: f32 = 0f32;
static mut CAM_Y: f32 = 90f32;
static mut CAM_Z: f32 = 270f32;

fn update(pixel_grid: &mut PixelGrid) {
    let (cam_x, cam_y, cam_z) = unsafe {
        // CAM_Z += 0.01;
        // CAM_X += 0.01;
        // CAM_Y += 0.01;
        (CAM_X, CAM_Y, CAM_Z)
    };

    // println!("{cam_z}");

    let cos_x = f32::cos(f32::to_radians(cam_x));
    let cos_y = f32::cos(f32::to_radians(cam_y));
    let cos_z = f32::cos(f32::to_radians(cam_z));

    let sin_x = f32::sin(f32::to_radians(cam_x));
    let sin_y = f32::sin(f32::to_radians(cam_y));
    let sin_z = f32::sin(f32::to_radians(cam_z));

    /*

    let world_to_screen_matrix = M3x3::new([
        V3::new(
            -1f32 * cos_z * sin_y * sin_x + sin_z * cos_x,
            cos_z * sin_y * cos_x - sin_z * sin_x,
            cos_z * cos_y,
        ),
        V3::new(
            -1f32 * sin_z * sin_y * sin_x - cos_z * cos_x,
            sin_z * sin_y * cos_x - cos_z * sin_x,
            sin_z * cos_y,
        ),
        /*
        V3::new(
            sin_z * sin_y * sin_x + cos_z * cos_x,
            -1f32 * sin_z * sin_y * cos_x - cos_z * sin_x,
            sin_z * cos_y,
        ),
        */
        V3::new(-1f32 * cos_y * sin_x, sin_y * cos_x, -1f32 * sin_y),
    ]);
    */

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
        let px = world_to_screen_matrix * *point;

        if px.z < 0f32 {
            // continue;
        }

        let px = pixel_grid.get_pixel(
            px.x as u32 + BUFF_WIDTH / 2 - 1,
            px.y as u32 + BUFF_HEIGHT / 2 - 1,
        );

        px[0] = 255;
        px[1] = 255;
        px[2] = 255;
    }
}
