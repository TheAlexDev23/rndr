pub mod instance;
pub mod render;
pub mod scene;

pub mod prelude {
    pub use super::instance::*;
    pub use super::render::*;
    pub use super::scene::*;
}

pub mod events {
    pub use sdl2::event::*;
    pub use sdl2::keyboard::*;
    pub use sdl2::EventPump;
}
