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

    pub fn line(&mut self, mut start: (i32, i32), mut end: (i32, i32), color: [u8; 3]) {
        if end.0 == start.0 {
            for y in start.1.min(end.1)..end.1.max(start.1) {
                self.set_pixel_checking_bounds(start.0 as i32, y as i32, color);
            }
            return;
        }

        if end.0 < start.0 {
            (start, end) = (end, start)
        }

        let rate = (end.1 - start.1) as f32 / (end.0 - start.0) as f32;

        let mut current_x = start.0;
        let mut current_y = start.1 as f32;
        let mut previous_y = current_y;

        while current_x <= end.0 {
            if current_y > previous_y && current_y - previous_y >= 1.0 {
                for y in previous_y as i32 + 1..current_y.round() as i32 {
                    self.set_pixel_checking_bounds(current_x, y, color);
                }
            } else if previous_y > current_y && previous_y - current_y >= 1.0 {
                for y in current_y as i32 + 1..previous_y.round() as i32 {
                    self.set_pixel_checking_bounds(current_x, y, color);
                }
            }

            self.set_pixel_checking_bounds(current_x, current_y.round() as i32, color);

            previous_y = current_y;
            current_y += rate;

            current_x += 1;
        }
    }

    pub fn triangle(
        &mut self,
        first: (f32, f32),
        second: (f32, f32),
        third: (f32, f32),
        buff_width: u32,
        buff_height: u32,
    ) {
        let total_triangle_area = Self::triangle_area(first, second, third);

        let buff_width = (buff_width / 2) as f32;
        let buff_height = (buff_height / 2) as f32;

        let x_start = first
            .0
            .min(second.0)
            .min(third.0)
            .max(-1.0 * buff_width)
            .round() as i32;

        let x_end = first.0.max(second.0).max(third.0).min(buff_width).round() as i32;

        let y_start = first
            .1
            .min(second.1)
            .min(third.1)
            .max(-1.0 * buff_height)
            .round() as i32;

        let y_end = first.1.max(second.1).max(third.1).min(buff_height).round() as i32;

        for x in x_start..x_end {
            for y in y_start..y_end {
                let pt = (x as f32, y as f32);

                let comparing = Self::triangle_area(pt, first, second)
                    + Self::triangle_area(pt, first, third)
                    + Self::triangle_area(pt, third, second);

                const EPILIPSON: f32 = 0.01;

                if (comparing - total_triangle_area).abs() <= EPILIPSON {
                    let screen_x = x + buff_width as i32;
                    let screen_y = buff_height as i32 + y;
                    let px = self.get_pixel(screen_x as u32, screen_y as u32);
                    px[0] = 255;
                    px[1] = 255;
                    px[2] = 255;
                }
            }
        }
    }

    fn set_pixel_checking_bounds(&mut self, x: i32, y: i32, color: [u8; 3]) {
        if x >= self.width as i32 || x < 0 || y >= self.height as i32 || y < 0 {
            return;
        }
        let px = self.get_pixel(x as u32, y as u32);
        px[0] = color[0];
        px[1] = color[1];
        px[2] = color[2];
    }

    fn triangle_area(p1: (f32, f32), p2: (f32, f32), p3: (f32, f32)) -> f32 {
        0.5 * ((p1.0 * (p2.1 - p3.1) + p2.0 * (p3.1 - p1.1) + p3.0 * (p1.1 - p2.1)).abs())
    }
}
