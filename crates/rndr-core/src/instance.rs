use sdl2::{
    render::{Texture, TextureValueError, UpdateTextureError, WindowCanvas},
    video::WindowBuildError,
    IntegerOrSdlError, Sdl, VideoSubsystem,
};

use thiserror::Error;

use super::events::EventPump;
use super::pixel::PixelGrid;

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
pub enum RenderError {
    #[error("Could not update buffer texture: {0}")]
    SdlUpdateTexture(#[from] UpdateTextureError),
    #[error("Could not copy buffer texture to canvas: {0}")]
    SdlCanvasCopy(String),
}

pub struct Instance {
    pub pixel_grid: PixelGrid,
    pub event_pump: EventPump,

    width: u32,
    height: u32,

    buff_width: u32,
    buff_height: u32,

    sdl_instance: SdlInstance,
}

struct SdlInstance {
    sdl_ctx: Sdl,
    video: VideoSubsystem,
    canvas: WindowCanvas,
    buff_texture: Texture,
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
            .build()?;

        let canvas = window
            .into_canvas()
            .build()
            .map_err(InitError::SdlCanvasInit)?;

        let event_pump = sdl_ctx.event_pump().map_err(InitError::SdlEventPumpInit)?;
        let buff_texture = canvas.create_texture_streaming(
            sdl2::pixels::PixelFormatEnum::RGB24,
            buff_width,
            buff_height,
        )?;

        Ok(Instance {
            pixel_grid: PixelGrid::new(buff_width, buff_height),
            event_pump,
            width,
            height,
            buff_width,
            buff_height,
            sdl_instance: SdlInstance {
                sdl_ctx,
                video,
                buff_texture,
                canvas,
            },
        })
    }

    pub fn render(&mut self) -> Result<(), RenderError> {
        self.sdl_instance.buff_texture.update(
            None,
            self.pixel_grid.get_pixel_data(),
            (self.buff_width * 3) as usize,
        )?;

        self.sdl_instance.canvas.clear();
        self.sdl_instance
            .canvas
            .copy(&self.sdl_instance.buff_texture, None, None)
            .map_err(RenderError::SdlCanvasCopy)?;
        self.sdl_instance.canvas.present();

        Ok(())
    }
}
