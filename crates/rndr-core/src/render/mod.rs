pub mod camera;
pub mod pixel;
pub mod shader;

pub use camera::Camera;
pub use pixel::PixelGrid;
use rayon::iter::IntoParallelRefIterator;
use rayon::iter::ParallelIterator;
use rndr_math::matrix::M3x3;
use rndr_math::vector::V3;
pub use shader::FragData;
pub use shader::FragShader;

use crate::prelude::Object;
use crate::prelude::SceneContext;

pub(crate) struct RenderContext {
    pub pixel_grid: PixelGrid,
    pub camera: Camera,
    pub buff_width: u32,
    pub buff_height: u32,
}

impl RenderContext {
    pub fn new(buff_width: u32, buff_height: u32) -> RenderContext {
        RenderContext {
            camera: Camera::new(true),
            pixel_grid: PixelGrid::new(buff_width, buff_height),
            buff_width,
            buff_height,
        }
    }

    pub fn render(&mut self, scene_context: &mut SceneContext) {
        let projection_matrix = self.camera.get_projection_matrix();
        let camera = self.camera.clone();
        for object in scene_context.objects.iter() {
            let pixel_changes: Vec<_> = object
                .triangles
                .par_iter()
                .map(|triangle| self.render_triangle(object, &camera, &projection_matrix, triangle))
                .collect();

            for pixel_change in pixel_changes {
                for pixel in pixel_change {
                    self.pixel_grid.set_pixel(pixel.0, pixel.1, pixel.2);
                }
            }
        }
    }

    fn render_triangle(
        &self,
        object: &Object,
        camera: &Camera,
        projection_matrix: &M3x3,
        triangle: &[usize; 3],
    ) -> Vec<(u32, u32, (f32, [u8; 3]))> {
        let first_projected = camera.project_point(*projection_matrix, &object, triangle[0]);

        let second_projected = camera.project_point(*projection_matrix, &object, triangle[1]);

        let third_projected = camera.project_point(*projection_matrix, &object, triangle[2]);

        let near_plane = camera.near_plane;

        if first_projected.z <= near_plane
            || second_projected.z <= near_plane
            || third_projected.z <= near_plane
        {
            return Vec::new();
        }

        // Values for the raster_triangle function to take. As it takes touples and
        // the projected values would still be used in the future
        let first = (first_projected.x, first_projected.y);
        let second = (second_projected.x, second_projected.y);
        let third = (third_projected.x, third_projected.y);

        self.raster_triangle(first, second, third, |f, s, t| {
            // V means vertex
            let first_v = object.vertices[triangle[0]];
            let second_v = object.vertices[triangle[1]];
            let third_v = object.vertices[triangle[2]];

            // VC means virtual color, it's the projected pixel's z value and color
            let first_vc = (first_projected.z, first_v.color);
            let second_vc = (second_projected.z, second_v.color);
            let third_vc = (third_projected.z, third_v.color);

            let interpolated_color =
                interpolate_virtual_color((f, first_vc), (s, second_vc), (t, (third_vc)));

            // TODO: use getters and setters to obtain this values on demand in the future
            // By doing this we could get rid of pointless calculations each pixel

            let relative_position = V3::interpolate3(
                (first_v.position, f),
                (second_v.position, s),
                (third_v.position, t),
            );

            let space_position = V3::interpolate3(
                (first_v.position + object.transform.position, f),
                (second_v.position + object.transform.position, s),
                (third_v.position + object.transform.position, t),
            );

            let mut data = FragData {
                relative_position,
                space_position,
                output_pixel: interpolated_color,
            };

            object.shader.frag(&mut data);

            data.output_pixel
        })
    }

    fn raster_triangle<F>(
        &self,
        first: (f32, f32),
        second: (f32, f32),
        third: (f32, f32),
        pixel: F,
    ) -> Vec<(u32, u32, (f32, [u8; 3]))>
    where
        F: Fn(f32, f32, f32) -> (f32, [u8; 3]),
    {
        let total_area = triangle_area(first, second, third);

        let width = (self.buff_width / 2) as f32;
        let height = (self.buff_height / 2) as f32;

        let x_start = first.0.min(second.0).min(third.0).max(-1.0 * width).round() as i32;

        let x_end = first.0.max(second.0).max(third.0).min(width).round() as i32;

        let y_start = first
            .1
            .min(second.1)
            .min(third.1)
            .max(-1.0 * height)
            .round() as i32;

        let y_end = first.1.max(second.1).max(third.1).min(height).round() as i32;

        let mut ret = Vec::with_capacity(total_area as usize);

        for x in x_start..x_end {
            for y in y_start..y_end {
                let pt = (x as f32, y as f32);

                let first_second = triangle_area(pt, first, second);
                let first_third = triangle_area(pt, first, third);
                let third_second = triangle_area(pt, third, second);

                const EPILIPSON: f32 = 0.01;

                if (first_second + first_third + third_second - total_area).abs() <= EPILIPSON {
                    let screen_x = x + width as i32;
                    let screen_y = height as i32 + y;

                    let first = third_second / total_area;
                    let second = first_third / total_area;
                    let third = first_second / total_area;

                    ret.push((
                        screen_x as u32,
                        screen_y as u32,
                        pixel(first, second, third),
                    ));
                }
            }
        }

        ret
    }
}

fn triangle_area(p1: (f32, f32), p2: (f32, f32), p3: (f32, f32)) -> f32 {
    0.5 * ((p1.0 * (p2.1 - p3.1) + p2.0 * (p3.1 - p1.1) + p3.0 * (p1.1 - p2.1)).abs())
}

fn interpolate_virtual_color(
    v1: (f32, (f32, [u8; 3])),
    v2: (f32, (f32, [u8; 3])),
    v3: (f32, (f32, [u8; 3])),
) -> (f32, [u8; 3]) {
    let (v1_influence, v2_influence, v3_influence) = (v1.0, v2.0, v3.0);

    let (v1_color, v2_color, v3_color) = (v1.1 .1, v2.1 .1, v3.1 .1);
    let (v1_z, v2_z, v3_z) = (v1.1 .0, v2.1 .0, v3.1 .0);

    (
        v1_z * v1_influence + v2_z * v2_influence + v3_z * v3_influence,
        [
            (v1_color[0] as f32 * v1_influence
                + v2_color[0] as f32 * v2_influence
                + v3_color[0] as f32 * v3_influence) as u8,
            (v1_color[1] as f32 * v1_influence
                + v2_color[1] as f32 * v2_influence
                + v3_color[1] as f32 * v3_influence) as u8,
            (v1_color[2] as f32 * v1_influence
                + v2_color[2] as f32 * v2_influence
                + v3_color[2] as f32 * v3_influence) as u8,
        ],
    )
}
