pub mod camera;
pub mod pixel;
pub mod shader;

pub use camera::Camera;
pub use pixel::PixelGrid;
pub use shader::FragData;
pub use shader::FragShader;

use std::collections::HashMap;

use crate::prelude::SceneContext;

pub(crate) struct RenderContext {
    pub pixel_grid: PixelGrid,
    pub camera: Camera,
    pub buff_width: u32,
    pub buff_height: u32,

    shaders: Vec<Box<dyn FragShader>>,
}

impl RenderContext {
    pub fn new(buff_width: u32, buff_height: u32) -> RenderContext {
        RenderContext {
            camera: Camera::new(true),
            pixel_grid: PixelGrid::new(buff_width, buff_height),
            shaders: Vec::new(),
            buff_width,
            buff_height,
        }
    }

    pub fn register_frag_shader(&mut self, shader: Box<dyn FragShader>) {
        self.shaders.push(shader);
    }

    pub fn render(&mut self, scene_context: &mut SceneContext) {
        for object in scene_context.objects.iter() {
            let mut cached_screen_points = HashMap::new();

            let mut i = 0;
            while i < object.triangles.len() {
                let first_i = object.triangles[i];
                let second_i = object.triangles[i + 1];
                let third_i = object.triangles[i + 2];

                let first_projected =
                    self.camera
                        .project_point(&mut cached_screen_points, &object, first_i);

                let second_projected =
                    self.camera
                        .project_point(&mut cached_screen_points, &object, second_i);

                let third_projected =
                    self.camera
                        .project_point(&mut cached_screen_points, &object, third_i);

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
                    let first = object.vertices[first_i];
                    let second = object.vertices[second_i];
                    let third = object.vertices[third_i];

                    let mut data = FragData {
                        // TODO: use interpolated vertex color
                        color: [(255.0 * f) as u8, (255.0 * s) as u8, (255.0 * t) as u8],
                        position: first * f + second * s + third * t,
                    };

                    for shader in self.shaders.iter() {
                        shader.frag(&mut data);
                    }

                    data.color
                });

                i += 3;
            }
        }
    }
}
