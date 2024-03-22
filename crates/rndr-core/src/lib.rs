pub mod instance;
pub mod pixel;

pub use instance::Instance;

pub mod events {
    pub use sdl2::event::*;
    pub use sdl2::keyboard::*;
    pub use sdl2::EventPump;
}
