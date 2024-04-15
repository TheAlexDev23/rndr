pub struct PixelGrid {
    pub width: u32,
    pub height: u32,
    pixel_colors: Vec<u8>,
    pixel_zs: Vec<f32>,
}

// Arbitrary large number, so that pretty much any ordinary pixel would beat it and not get z occluded
const DEFAULT_Z: f32 = 100_000_000.0;

impl PixelGrid {
    pub fn new(width: u32, height: u32) -> PixelGrid {
        PixelGrid {
            width,
            height,
            pixel_colors: vec![0; (width * height * 3) as usize],
            pixel_zs: vec![DEFAULT_Z; (width * height) as usize],
        }
    }

    pub fn get_pixel(&mut self, x: u32, y: u32) -> (&mut f32, &mut [u8]) {
        let base = 3 * (self.width * y + x) as usize;
        (
            &mut self.pixel_zs[(self.width * y + x) as usize],
            &mut self.pixel_colors[base..=base + 2],
        )
    }

    pub fn set_pixel(&mut self, x: u32, y: u32, pixel: (f32, [u8; 3])) {
        let current_pixel = self.get_pixel(x, y);
        if pixel.0 > *current_pixel.0 {
            return;
        }

        *current_pixel.0 = pixel.0;

        current_pixel.1[0] = pixel.1[0];
        current_pixel.1[1] = pixel.1[1];
        current_pixel.1[2] = pixel.1[2];
    }

    pub fn clear(&mut self) {
        self.pixel_colors
            .iter_mut()
            .for_each(|x| *x = unsafe { std::mem::zeroed() });
        self.pixel_zs.iter_mut().for_each(|x| *x = DEFAULT_Z);
    }

    pub fn get_pixel_data(&self) -> &[u8] {
        &self.pixel_colors
    }
}
