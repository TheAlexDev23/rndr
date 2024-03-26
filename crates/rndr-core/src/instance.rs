use sdl2::{
    mouse::MouseUtil,
    render::UpdateTextureError,
    render::{Texture, TextureValueError, WindowCanvas},
    video::WindowBuildError,
    IntegerOrSdlError, Sdl, VideoSubsystem,
};

use thiserror::Error;

use crate::events::EventPump;
use crate::prelude::{Camera, FragShader, Object, PixelGrid, RenderContext, SceneContext};

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
    pub event_pump: EventPump,

    pub(crate) width: u32,
    pub(crate) height: u32,

    pub(crate) render_context: RenderContext,
    pub(crate) scene_context: SceneContext,

    pub(crate) sdl_instance: SdlInstance,
}

pub(crate) struct SdlInstance {
    pub sdl_ctx: Sdl,
    pub video: VideoSubsystem,
    pub canvas: WindowCanvas,
    pub buff_texture: Texture,
    pub mouse: MouseUtil,
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
            event_pump,
            width,
            height,
            render_context: RenderContext::new(buff_width, buff_height),
            scene_context: SceneContext {
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

    pub fn render(&mut self) {
        self.render_context.render(&mut self.scene_context);
    }

    pub fn apply_render(&mut self) -> Result<(), RenderError> {
        self.sdl_instance.buff_texture.update(
            None,
            self.render_context.pixel_grid.get_pixel_data(),
            (self.render_context.buff_width * 3) as usize,
        )?;

        self.sdl_instance
            .canvas
            .copy(&self.sdl_instance.buff_texture, None, None)
            .map_err(RenderError::SdlCanvasCopy)?;
        self.sdl_instance.canvas.present();

        self.render_context.pixel_grid.clear();

        Ok(())
    }

    pub fn register_frag_shader(&mut self, shader: Box<dyn FragShader>) {
        self.render_context.register_frag_shader(shader);
    }

    pub fn register_object(&mut self, object: Object) -> &mut Object {
        self.scene_context.objects.push(object);
        self.scene_context.objects.last_mut().unwrap()
    }

    pub fn get_camera(&mut self) -> &mut Camera {
        &mut self.render_context.camera
    }

    pub fn get_pixel_grid(&mut self) -> &mut PixelGrid {
        &mut self.render_context.pixel_grid
    }

    pub fn center_mouse(&mut self) {
        self.sdl_instance.mouse.warp_mouse_in_window(
            self.sdl_instance.canvas.window_mut(),
            (self.width) as i32 - 25,
            (self.height) as i32 - 25,
        );
    }
}
