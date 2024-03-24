use std::collections::HashMap;

use crate::instance::SceneContext;
use crate::prelude::PixelGrid;

pub fn render_scene(
    pixel_grid: &mut PixelGrid,
    scene_context: &mut SceneContext,
    buff_width: u32,
    buff_height: u32,
) {
    for object in scene_context.objects.iter() {
        let mut cached_screen_points = HashMap::new();

        let mut i = 0;
        while i < object.triangles.len() {
            let first = object.triangles[i];
            let second = object.triangles[i + 1];
            let third = object.triangles[i + 2];

            let first =
                scene_context
                    .camera
                    .project_point(&mut cached_screen_points, &object, first);

            let (second, third) = unsafe {
                let second = scene_context.camera.project_point_unsafe(
                    &mut cached_screen_points,
                    &object,
                    second,
                );

                let third = scene_context.camera.project_point_unsafe(
                    &mut cached_screen_points,
                    &object,
                    third,
                );

                (second, third)
            };

            let near_plane = scene_context.camera.near_plane;

            if first.z <= near_plane || second.z <= near_plane || third.z <= near_plane {
                i += 3;
                continue;
            }

            let first = (first.x, first.y);
            let second = (second.x, second.y);
            let third = (third.x, third.y);

            pixel_grid.triangle(first, second, third, buff_width, buff_height);

            i += 3;
        }
    }
}
