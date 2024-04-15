pub mod instance;
pub mod object;
pub mod render;

pub mod default_components;
pub mod default_objects;
pub mod default_systems;

pub mod prelude {
    pub use super::instance::*;
    pub use super::object::*;
    pub use super::render::*;
}

pub mod events {
    pub use sdl2::event::*;
    pub use sdl2::keyboard::*;
    pub use sdl2::EventPump;
}
