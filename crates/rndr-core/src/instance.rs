use crate::{prelude::Camera, Object};

use sdl2::{
    mouse::MouseUtil,
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

pub struct Instance {
    pub pixel_grid: PixelGrid,
    pub event_pump: EventPump,

    pub(crate) width: u32,
    pub(crate) height: u32,

    pub(crate) buff_width: u32,
    pub(crate) buff_height: u32,

    pub(crate) scene_context: SceneContext,

    pub(crate) sdl_instance: SdlInstance,
}

pub(crate) struct SdlInstance {
    pub(crate) sdl_ctx: Sdl,
    pub(crate) video: VideoSubsystem,
    pub(crate) canvas: WindowCanvas,
    pub(crate) buff_texture: Texture,
    pub(crate) mouse: MouseUtil,
}

pub(crate) struct SceneContext {
    pub camera: Camera,
    pub objects: Vec<Object>,
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
            scene_context: SceneContext {
                camera: Camera::new(true),
                objects: Vec::new(),
            },
            sdl_instance: SdlInstance {
                sdl_ctx,
                video,
                buff_texture,
                canvas,
                mouse,
            },
        })
    }

    pub fn register_object(&mut self, object: Object) -> &mut Object {
        self.scene_context.objects.push(object);
        self.scene_context.objects.last_mut().unwrap()
    }

    pub fn get_camera(&mut self) -> &mut Camera {
        &mut self.scene_context.camera
    }

    pub fn center_mouse(&mut self) {
        self.sdl_instance.mouse.warp_mouse_in_window(
            self.sdl_instance.canvas.window_mut(),
            (self.width) as i32 - 25,
            (self.height) as i32 - 25,
        );
    }
}
