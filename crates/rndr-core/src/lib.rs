pub mod camera;
pub mod instance;
pub mod object;
pub mod pixel;

pub use instance::Instance;
pub use object::Object;

mod render;

pub mod prelude {
    pub use super::camera::*;
    pub use super::instance::*;
    pub use super::object::*;
    pub use super::pixel::*;
}

pub mod events {
    pub use sdl2::event::*;
    pub use sdl2::keyboard::*;
    pub use sdl2::EventPump;
}
