use rndr_core::events::Event;
use rndr_core::pixel::PixelGrid;

const HEIGHT: u32 = 500;
const WIDTH: u32 = 1000;

const BUFF_HEIGHT: u32 = 50;
const BUFF_WIDTH: u32 = 100;

fn main() {
    let mut instance = rndr_core::Instance::init(WIDTH, HEIGHT, BUFF_WIDTH, BUFF_HEIGHT)
        .expect("Could not init rndr");
    loop {
        for event in instance.event_pump.poll_iter() {
            if let Event::Quit { timestamp: _ } = event {
                panic!("Exit requested");
            }
        }
        update(&mut instance.pixel_grid);
        instance.render().expect("Could not render");
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
