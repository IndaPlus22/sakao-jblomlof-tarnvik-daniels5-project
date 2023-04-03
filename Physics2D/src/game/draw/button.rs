use graphics::{types::Vec2d, color};
use opengl_graphics::OpenGL;
use piston::{
    input::{RenderEvent, UpdateEvent},
    window::WindowSettings,
};
use piston_window::{Event, PistonWindow, types::Color, Rectangle, rectangle::square};

// struct for button in UI.
// TODO: put in parameters such as:
// position, size, funtionality, (and other things that don't come up to mind right now)
pub struct Button {
    dims: [f64; 4],
    // action: ,
    // what todo,
    color: Color,
    rect: Rectangle
}

impl Button {
    // constructor for Button
    pub fn new(pos: Vec2d, size: Vec2d, color: Color) -> Button {
        let rect = Rectangle::new(color);
        let dims = square(pos[0], pos[1], size[0]);
        Button {
            dims, color, rect
        }
    }

    pub fn draw(&self) {
        // rect.draw(dims, &draw_state::Default::default(), transform, g);
    }

    // TODO: Add features such as the functionality
}