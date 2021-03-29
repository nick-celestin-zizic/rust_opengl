#![allow(dead_code)]
pub extern crate regex;
pub extern crate model_parser;
pub extern crate image;

#[macro_use]
mod types;
mod opengl_backend;
mod game;
mod util;

pub fn main() {
    if let Err(e) = opengl_backend::main() {
        println!("Could not launch game:\n'{}'", e)
    }
}
