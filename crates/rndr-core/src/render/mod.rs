pub mod camera;
pub mod pixel;

pub use camera::Camera;
pub use pixel::PixelGrid;

use std::collections::HashMap;

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
        for object in scene_context.objects.iter() {
            let mut cached_screen_points = HashMap::new();

            let mut i = 0;
            while i < object.triangles.len() {
                let first = object.triangles[i];
                let second = object.triangles[i + 1];
                let third = object.triangles[i + 2];

                let first = self
                    .camera
                    .project_point(&mut cached_screen_points, &object, first);

                let second = self
                    .camera
                    .project_point(&mut cached_screen_points, &object, second);

                let third = self
                    .camera
                    .project_point(&mut cached_screen_points, &object, third);

                let near_plane = self.camera.near_plane;

                if first.z <= near_plane || second.z <= near_plane || third.z <= near_plane {
                    i += 3;
                    continue;
                }

                let first = (first.x, first.y);
                let second = (second.x, second.y);
                let third = (third.x, third.y);

                self.pixel_grid
                    .triangle(first, second, third, self.buff_width, self.buff_height);

                i += 3;
            }
        }
    }
}
