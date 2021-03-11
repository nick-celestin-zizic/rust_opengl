pub extern crate regex;
pub extern crate model_parser;
pub extern crate image;


#[macro_use]
mod types;
mod opengl_backend;
mod game;


pub fn main() -> types::Maybe<()> {
    opengl_backend::main()?;

    Ok(())
}
