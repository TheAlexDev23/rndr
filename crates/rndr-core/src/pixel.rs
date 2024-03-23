pub struct PixelGrid {
    width: u32,
    height: u32,
    pixels: Vec<u8>,
}

impl PixelGrid {
    pub fn new(width: u32, height: u32) -> PixelGrid {
        PixelGrid {
            width,
            height,
            pixels: vec![0; (width * height * 3) as usize],
        }
    }

    pub fn get_pixel(&mut self, x: u32, y: u32) -> &mut [u8] {
        let base = (3 * (self.width * y + x)) as usize;
        &mut self.pixels[base..=base + 2]
    }

    pub fn clear(&mut self) {
        self.pixels = self.pixels.iter().map(|_| 0).collect()
    }

    pub fn get_pixel_data(&self) -> &[u8] {
        &self.pixels
    }

    pub fn line(&mut self, mut start: (u32, u32), mut end: (u32, u32), color: [u8; 3]) {
        if end.0 == start.0 {
            for y in start.1.min(end.1)..end.1.max(start.1) {
                if self.set_pixel_checking_bounds(start.0 as i32, y as i32, color) {
                    return;
                }
            }
            return;
        }

        if end.0 < start.0 {
            (start, end) = (end, start)
        }

        let (end, start) = (
            (end.0 as i32, end.1 as i32),
            (start.0 as i32, start.1 as i32),
        );

        let rate = (end.1 - start.1) as f32 / (end.0 - start.0) as f32;

        let mut current_x = start.0;
        let mut current_y = start.1 as f32;
        let mut previous_y = current_y;

        while current_x <= end.0 {
            if current_y > previous_y && current_y - previous_y >= 1.0 {
                for y in previous_y as i32 + 1..current_y.round() as i32 {
                    if self.set_pixel_checking_bounds(current_x, y, color) {
                        return;
                    }
                }
            } else if previous_y > current_y && previous_y - current_y >= 1.0 {
                for y in current_y as i32 + 1..previous_y.round() as i32 {
                    if self.set_pixel_checking_bounds(current_x, y, color) {
                        return;
                    }
                }
            }

            if self.set_pixel_checking_bounds(current_x, current_y.round() as i32, color) {
                return;
            }

            previous_y = current_y;
            current_y += rate;

            current_x += 1;
        }
    }

    fn set_pixel_checking_bounds(&mut self, x: i32, y: i32, color: [u8; 3]) -> bool {
        if x >= self.width as i32 || x < 0 || y >= self.height as i32 || y < 0 {
            return true;
        }
        let px = self.get_pixel(x as u32, y as u32);
        px[0] = color[0];
        px[1] = color[1];
        px[2] = color[2];
        false
    }
}
