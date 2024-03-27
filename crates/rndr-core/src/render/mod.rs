pub mod camera;
pub mod pixel;
pub mod shader;

pub use camera::Camera;
pub use pixel::PixelGrid;
use rndr_math::vector::V3;
pub use shader::FragData;
pub use shader::FragShader;

use std::collections::HashMap;

use crate::prelude::SceneContext;

pub(crate) struct RenderContext {
    pub pixel_grid: PixelGrid,
    pub camera: Camera,
    pub buff_width: u32,

    shaders: Vec<Box<dyn FragShader>>,
}

impl RenderContext {
    pub fn new(buff_width: u32, buff_height: u32) -> RenderContext {
        RenderContext {
            camera: Camera::new(true),
            pixel_grid: PixelGrid::new(buff_width, buff_height),
            shaders: Vec::new(),
            buff_width,
        }
    }

    pub fn register_frag_shader(&mut self, shader: Box<dyn FragShader>) {
        self.shaders.push(shader);
    }

    pub fn render(&mut self, scene_context: &mut SceneContext) {
        let projection_matrix = self.camera.get_projection_matrix();
        for object in scene_context.objects.iter() {
            let mut cached_screen_points = HashMap::new();

            let mut i = 0;
            while i < object.triangles.len() {
                let first_i = object.triangles[i];
                let second_i = object.triangles[i + 1];
                let third_i = object.triangles[i + 2];

                let first_projected = self.camera.project_point(
                    &mut cached_screen_points,
                    projection_matrix,
                    &object,
                    first_i,
                );

                let second_projected = self.camera.project_point(
                    &mut cached_screen_points,
                    projection_matrix,
                    &object,
                    second_i,
                );

                let third_projected = self.camera.project_point(
                    &mut cached_screen_points,
                    projection_matrix,
                    &object,
                    third_i,
                );

                let near_plane = self.camera.near_plane;

                if first_projected.z <= near_plane
                    || second_projected.z <= near_plane
                    || third_projected.z <= near_plane
                {
                    i += 3;
                    continue;
                }

                let first = (first_projected.x, first_projected.y);
                let second = (second_projected.x, second_projected.y);
                let third = (third_projected.x, third_projected.y);

                self.pixel_grid.triangle(first, second, third, |f, s, t| {
                    // V means vertex
                    let first_v = object.vertices[first_i];
                    let second_v = object.vertices[second_i];
                    let third_v = object.vertices[third_i];

                    // VC means virtual color, it's the projected pixel's z value and color
                    let first_vc = (first_projected.z, first_v.color);
                    let second_vc = (second_projected.z, second_v.color);
                    let third_vc = (third_projected.z, third_v.color);

                    let interpolated_color =
                        interpolate_virtual_color((f, first_vc), (s, second_vc), (t, (third_vc)));

                    if !self.shaders.is_empty() {
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

                        for shader in self.shaders.iter() {
                            shader.frag(&mut data);
                        }

                        data.output_pixel
                    } else {
                        interpolated_color
                    }
                });

                i += 3;
            }
        }
    }
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
