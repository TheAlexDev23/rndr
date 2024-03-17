mod pixels;

use sdl2::pixels::PixelFormatEnum;

use pixels::PixelGrid;

const HEIGHT: u32 = 500;
const WIDTH: u32 = 1000;

const BUFF_HEIGHT: u32 = 50;
const BUFF_WIDTH: u32 = 100;

fn main() {
    let sdl_ctx = sdl2::init().expect("Could not init SDL");
    let video = sdl_ctx.video().expect("Could not init SDL video subsystem");
    let win = video
        .window("Test", WIDTH, HEIGHT)
        .position_centered()
        .build()
        .expect("Could not create SDL window");
    let mut canvas = win
        .into_canvas()
        .build()
        .expect("Could not create SDL canvas");

    let texture_creator = canvas.texture_creator();
    let mut texture = texture_creator
        .create_texture_streaming(PixelFormatEnum::RGB24, BUFF_WIDTH, BUFF_HEIGHT)
        .unwrap();

    let mut event_pump = sdl_ctx.event_pump().unwrap();

    let mut pixel_grid = PixelGrid::new(BUFF_WIDTH, BUFF_HEIGHT);

    'app_loop: loop {
        for event in event_pump.poll_iter() {
            if let sdl2::event::Event::Quit { timestamp: _ } = event {
                break 'app_loop;
            }
        }

        update(&mut pixel_grid);

        texture
            .update(None, pixel_grid.get_pixel_data(), (BUFF_WIDTH * 3) as usize)
            .expect("Could not update texture");

        canvas.clear();
        canvas.copy(&texture, None, None).unwrap();
        canvas.present();
    }
}

static mut COUNTER_X: u32 = 0;
static mut COUNTER_Y: u32 = 0;

fn update(pixel_grid: &mut PixelGrid) {
    if unsafe { COUNTER_X } == BUFF_WIDTH {
        unsafe {
            COUNTER_X = 0;
            COUNTER_Y += 1;
        }
    }

    if unsafe { COUNTER_Y } == BUFF_HEIGHT {
        unsafe {
            COUNTER_Y = 0;
        }
    }

    let px = unsafe { pixel_grid.get_pixel(COUNTER_X, COUNTER_Y) };
    px[0] += 15;
    px[1] += 15;
    px[2] += 15;

    unsafe {
        COUNTER_X += 1;
    }
}
