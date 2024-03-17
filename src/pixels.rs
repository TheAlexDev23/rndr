pub struct PixelGrid {
    width: u32,
    pixels: Vec<u8>,
}

impl PixelGrid {
    pub fn new(width: u32, height: u32) -> PixelGrid {
        PixelGrid {
            width,
            pixels: vec![0; (width * height * 3) as usize],
        }
    }

    pub fn get_pixel(&mut self, x: u32, y: u32) -> &mut [u8] {
        let base = (self.width * 3 * y + x * 3) as usize;
        &mut self.pixels[base..=base + 2]
    }

    pub fn get_pixel_data(&self) -> &[u8] {
        &self.pixels
    }
}
