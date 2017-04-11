#![crate_name="orbclient"]
#![crate_type="lib"]
#![feature(asm)]
#![feature(const_fn)]

#![deny(warnings)]

extern crate core;
extern crate syscall;

#[macro_use]
extern crate bitflags;

pub static FONT: &'static [u8] = include_bytes!("../res/unifont.font");

pub use color::Color;
pub use imp::{get_display_size, EventIter, Window};
pub use input::*;
pub use graphicspath::GraphicsPath;
pub use renderer::Renderer;

pub mod color;
pub mod input;
pub mod graphicspath;
pub mod renderer;

#[derive(Clone, Copy, Debug)]
pub enum WindowFlag {
    Async,
    Resizable,
    Unclosable
}

#[cfg(target_os = "redox")]
#[path="imp/orbital.rs"]
mod imp;

#[cfg(not(target_os = "redox"))]
#[path="imp/sdl2.rs"]
mod imp;
