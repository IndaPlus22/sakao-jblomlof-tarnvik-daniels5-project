use graphics::{types::Vec2d, color, draw_state, line::Shape};
use opengl_graphics::OpenGL;
use piston::{
    input::{RenderEvent, UpdateEvent},
    window::WindowSettings,
};
use gfx_device_gl::{CommandBuffer, Resources};
use gfx_graphics::GfxGraphics;
use piston_window::{Event, PistonWindow, types::Color, Rectangle, rectangle::{square, self}, math::Matrix2d};


const RECT_SHAPE: [f64; 4] = [0.0,0.0,100.0,100.0];
// struct for button in UI.
// TODO: put in parameters such as:
// position, size, funtionality, (and other things that don't come up to mind right now)
pub struct Button {
    dims: [f64; 4],
    // action: ,
    // what todo,
    color: Color,
    shape: Rectangle
}

impl Button {
    // constructor for Button
    pub fn new(pos: Vec2d, size: Vec2d, color: Color) -> Button {
        let dims = square(pos[0] - size[0], pos[1], size[0]);
        let shape = Rectangle::new(color);
        Button {
            dims, color, shape
        }
    }

    pub fn draw(
        &self,
        graphics: &mut GfxGraphics<Resources, CommandBuffer>,
        transform: Matrix2d
    ) {
        self.shape.draw(self.dims, &piston_window::DrawState::default(), transform, graphics);
    }

    // TODO: Add features such as the functionality
}