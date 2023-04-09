/*use gfx_device_gl::{CommandBuffer, Resources};
use gfx_graphics::GfxGraphics;
use graphics::{color, draw_state, line::Shape, types::Vec2d};
use opengl_graphics::OpenGL;
use piston::{
    input::{RenderEvent, UpdateEvent},
    window::WindowSettings,
};
use piston_window::{
    math::Matrix2d,
    rectangle::{self, square},
    types::Color,
    Event, PistonWindow, Rectangle,
};

use super::input::{Input, self};

const RECT_SHAPE: [f64; 4] = [0.0, 0.0, 100.0, 100.0];
// struct for button in UI.
// TODO: put in parameters such as:
// position, size, funtionality, (and other things that don't come up to mind right now)
pub struct Button {
    dims: [f64; 4],
    // action: ,
    // what todo,
    color: Color,
    shape: Rectangle,
    hovered: bool,
}

impl Button {
    // constructor for Button
    pub fn new(pos: Vec2d, width: f64, height: f64, color: Color) -> Button {
        let dims = [pos[0], pos[1], width, height];
        let shape = Rectangle::new(color);
        Button {
            dims,
            color,
            shape,
            hovered: false,
        }
    }

    pub fn draw(&self, graphics: &mut GfxGraphics<Resources, CommandBuffer>, transform: Matrix2d) {

        self.shape.draw(
            self.dims,
            &piston_window::DrawState::default(),
            transform,
            graphics,
        );
    }

    // TODO: Add features such as the functionality
}*/
