extern crate model_parser;
extern crate image;
extern crate glium;

mod types;
mod opengl_backend;

pub fn main() -> types::Maybe<()> {
    opengl_backend::main()?;

    Ok(())
}
