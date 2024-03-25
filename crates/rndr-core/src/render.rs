use sdl2::render::UpdateTextureError;
use std::collections::HashMap;

use thiserror::Error;

use crate::Instance;

#[derive(Error, Debug)]
pub enum RenderError {
    #[error("Could not update buffer texture: {0}")]
    SdlUpdateTexture(#[from] UpdateTextureError),
    #[error("Could not copy buffer texture to canvas: {0}")]
    SdlCanvasCopy(String),
}

impl Instance {
    pub fn render(&mut self) {
        for object in self.scene_context.objects.iter() {
            let mut cached_screen_points = HashMap::new();

            let mut i = 0;
            while i < object.triangles.len() {
                let first = object.triangles[i];
                let second = object.triangles[i + 1];
                let third = object.triangles[i + 2];

                let first = self.scene_context.camera.project_point(
                    &mut cached_screen_points,
                    &object,
                    first,
                );

                let second = self.scene_context.camera.project_point(
                    &mut cached_screen_points,
                    &object,
                    second,
                );

                let third = self.scene_context.camera.project_point(
                    &mut cached_screen_points,
                    &object,
                    third,
                );

                let near_plane = self.scene_context.camera.near_plane;

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

    pub fn apply_render(&mut self) -> Result<(), RenderError> {
        self.sdl_instance.buff_texture.update(
            None,
            self.pixel_grid.get_pixel_data(),
            (self.buff_width * 3) as usize,
        )?;

        self.sdl_instance
            .canvas
            .copy(&self.sdl_instance.buff_texture, None, None)
            .map_err(RenderError::SdlCanvasCopy)?;
        self.sdl_instance.canvas.present();

        self.pixel_grid.clear();

        Ok(())
    }
}
