pub struct PixelGrid {
    width: u32,
    height: u32,
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

    fn set_pixel_with_bound_check(&mut self, x: i32, y: i32, pixel: (f32, [u8; 3])) {
        if x >= self.width as i32 || x < 0 || y >= self.height as i32 || y < 0 {
            return;
        }
        self.set_pixel(x as u32, y as u32, pixel);
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

    pub fn line(&mut self, mut start: (i32, i32), mut end: (i32, i32), color: [u8; 3]) {
        let pixel = (1.0, color);
        if end.0 == start.0 {
            for y in start.1.min(end.1)..end.1.max(start.1) {
                self.set_pixel_with_bound_check(start.0 as i32, y as i32, pixel);
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
                    self.set_pixel_with_bound_check(current_x, y, pixel);
                }
            } else if previous_y > current_y && previous_y - current_y >= 1.0 {
                for y in current_y as i32 + 1..previous_y.round() as i32 {
                    self.set_pixel_with_bound_check(current_x, y, pixel);
                }
            }

            self.set_pixel_with_bound_check(current_x, current_y.round() as i32, pixel);

            previous_y = current_y;
            current_y += rate;

            current_x += 1;
        }
    }

    pub fn triangle<F>(
        &mut self,
        first: (f32, f32),
        second: (f32, f32),
        third: (f32, f32),
        pixel: F,
    ) where
        F: Fn(f32, f32, f32) -> (f32, [u8; 3]),
    {
        let total_area = Self::triangle_area(first, second, third);

        let width = (self.width / 2) as f32;
        let height = (self.height / 2) as f32;

        let x_start = first.0.min(second.0).min(third.0).max(-1.0 * width).round() as i32;

        let x_end = first.0.max(second.0).max(third.0).min(width).round() as i32;

        let y_start = first
            .1
            .min(second.1)
            .min(third.1)
            .max(-1.0 * height)
            .round() as i32;

        let y_end = first.1.max(second.1).max(third.1).min(height).round() as i32;

        for x in x_start..x_end {
            for y in y_start..y_end {
                let pt = (x as f32, y as f32);

                // TODO: try to get the barycentric coordiantes first, and check if they sum up to 1.0
                // instead of summing areas and dividing areas

                let first_second = Self::triangle_area(pt, first, second);
                let first_third = Self::triangle_area(pt, first, third);
                let third_second = Self::triangle_area(pt, third, second);

                const EPILIPSON: f32 = 0.01;

                if (first_second + first_third + third_second - total_area).abs() <= EPILIPSON {
                    let screen_x = x + width as i32;
                    let screen_y = height as i32 + y;

                    let first = third_second / total_area;
                    let second = first_third / total_area;
                    let third = first_second / total_area;

                    self.set_pixel(
                        screen_x as u32,
                        screen_y as u32,
                        pixel(first, second, third),
                    );
                }
            }
        }
    }

    fn triangle_area(p1: (f32, f32), p2: (f32, f32), p3: (f32, f32)) -> f32 {
        0.5 * ((p1.0 * (p2.1 - p3.1) + p2.0 * (p3.1 - p1.1) + p3.0 * (p1.1 - p2.1)).abs())
    }
}
