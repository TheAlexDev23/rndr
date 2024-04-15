pub mod pixel;
pub mod shader;

pub use pixel::PixelGrid;
pub use shader::FragData;
pub use shader::FragShader;

use thiserror::Error;

use crate::default_components::{
    render::{Camera, MeshRenderable},
    Transform,
};
use crate::default_systems::mesh_renderer::MeshRendererSystem;
use crate::prelude::ObjectManager;

pub(crate) struct RenderContext {
    pub pixel_grid: PixelGrid,
    mesh_renderer: Option<MeshRendererSystem>,
}

#[derive(Error, Debug)]
pub enum RenderError {
    #[error("No camera present")]
    NoCamera,
}

impl RenderContext {
    pub fn new(buff_width: u32, buff_height: u32) -> RenderContext {
        RenderContext {
            pixel_grid: PixelGrid::new(buff_width, buff_height),
            mesh_renderer: None,
        }
    }

    pub fn configure_mesh_renderer(&mut self) {
        // lifetime may not live long enough
        // assignment requires that '1 must outlive 'a
        self.mesh_renderer = Some(MeshRendererSystem);
    }

    pub fn render_objects(&mut self, object_manager: &ObjectManager) -> Result<(), RenderError> {
        if let Some(ref mut mesh_renderer) = self.mesh_renderer {
            let width = self.pixel_grid.width;
            let height = self.pixel_grid.height;

            let camera_object = object_manager
                .objects_iter()
                .find(|obj| obj.component::<Camera>().is_some());

            if camera_object.is_none() {
                return Err(RenderError::NoCamera);
            }

            let camera = camera_object.unwrap().component::<Camera>().unwrap();

            let camera_transform = camera_object.unwrap().component::<Transform>().unwrap();

            for object in object_manager.objects_iter() {
                if !object.has_component::<MeshRenderable>() {
                    continue;
                }

                mesh_renderer.render_mesh_object(
                    &mut self.pixel_grid,
                    width,
                    height,
                    &object,
                    &camera,
                    &camera_transform,
                );
            }
        }

        Ok(())
    }
}
