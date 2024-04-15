use sdl2::{
    render::UpdateTextureError,
    render::{Texture, TextureValueError, WindowCanvas},
    video::WindowBuildError,
    IntegerOrSdlError,
};

use thiserror::Error;

use crate::events::EventPump;
use crate::object::ObjectManager;
use crate::prelude::{Object, PixelGrid, RenderContext};
use crate::render::RenderError;

#[derive(Error, Debug)]
pub enum InitError {
    #[error("Could not initialize SDL: {0}")]
    SdlInit(String),
    #[error("Could not initialize SDL video subsystem: {0}")]
    SdlVideoInit(String),
    #[error("Could not initialize SDL window: {0}")]
    SdlWindowInit(#[from] WindowBuildError),
    #[error("Could not initialize SDL canvas: {0}")]
    SdlCanvasInit(IntegerOrSdlError),
    #[error("Could not initialize SDL buffer texture: {0}")]
    SdlBufferTextureInit(#[from] TextureValueError),
    #[error("Could not initialize SDL event pump: {0}")]
    SdlEventPumpInit(String),
}

#[derive(Error, Debug)]
pub enum RenderApplyError {
    #[error("Could not update buffer texture: {0}")]
    SdlUpdateTexture(#[from] UpdateTextureError),
    #[error("Could not copy buffer texture to canvas: {0}")]
    SdlCanvasCopy(String),
}

pub struct Instance {
    pub event_pump: EventPump,
    pub object_manager: ObjectManager,

    pub(crate) render_context: RenderContext,

    pub(crate) sdl_instance: SdlInstance,
}

pub(crate) struct SdlInstance {
    pub canvas: WindowCanvas,
    pub buff_texture: Texture,
}

impl Instance {
    pub fn init(
        width: u32,
        height: u32,
        buff_width: u32,
        buff_height: u32,
    ) -> Result<Instance, InitError> {
        let sdl_ctx = sdl2::init().map_err(InitError::SdlInit)?;
        let video = sdl_ctx.video().map_err(InitError::SdlVideoInit)?;
        let window = video
            .window("Test", width, height)
            .position_centered()
            .input_grabbed()
            .build()?;

        let canvas = window
            .into_canvas()
            .build()
            .map_err(InitError::SdlCanvasInit)?;

        let mouse = sdl_ctx.mouse();
        mouse.set_relative_mouse_mode(true);

        let event_pump = sdl_ctx.event_pump().map_err(InitError::SdlEventPumpInit)?;
        let buff_texture = canvas.create_texture_streaming(
            sdl2::pixels::PixelFormatEnum::RGB24,
            buff_width,
            buff_height,
        )?;

        Ok(Instance {
            event_pump,
            render_context: RenderContext::new(buff_width, buff_height),
            sdl_instance: SdlInstance {
                buff_texture,
                canvas,
            },
            object_manager: ObjectManager::new(),
        })
    }

    pub fn configure_mesh_rendering_system(&mut self) {
        self.render_context.configure_mesh_renderer();
    }

    pub fn render(&mut self) -> Result<(), RenderError> {
        self.render_context.render_objects(&self.object_manager)
    }

    pub fn apply_render(&mut self) -> Result<(), RenderApplyError> {
        self.sdl_instance.buff_texture.update(
            None,
            self.render_context.pixel_grid.get_pixel_data(),
            (self.render_context.pixel_grid.width * 3) as usize,
        )?;

        self.sdl_instance
            .canvas
            .copy(&self.sdl_instance.buff_texture, None, None)
            .map_err(RenderApplyError::SdlCanvasCopy)?;
        self.sdl_instance.canvas.present();

        self.render_context.pixel_grid.clear();

        Ok(())
    }

    pub fn register_object(&mut self, object: Object) -> u64 {
        self.object_manager.register_object(object)
    }
    pub fn get_object(&self, index: u64) -> Option<&Object> {
        self.object_manager.get_object(index)
    }
    pub fn get_object_mut(&mut self, index: u64) -> Option<&mut Object> {
        self.object_manager.get_object_mut(index)
    }

    pub fn get_pixel_grid(&mut self) -> &mut PixelGrid {
        &mut self.render_context.pixel_grid
    }
}
