use gfx_device_gl::{Resources, CommandBuffer};
use gfx_graphics::GfxGraphics;
use piston_window::types::{Matrix2d, Vec2d};

use super::ui_button::Button;

pub struct Toolbar {
    button_size: Vec2d,
    position: Vec2d,
    buttons: Vec<Button>,
}

impl Toolbar {
    pub fn new(button_size: Vec2d, position: Vec2d) -> Toolbar {
        Toolbar {
            button_size,
            position,
            buttons: Vec::new(),
        }
    }

    fn init_buttons(button_size: Vec2d, position: Vec2d) -> Vec<Button> {
        let move_tool = Button::new(
            [position[0], position[1]],
            button_size[0],
            button_size[1],
            [1.0, 0.0, 0.0, 1.0],
        );
        let scale_tool = Button::new(
            [position[0], position[1] + button_size[1]],
            button_size[0],
            button_size[1],
            [0.0, 1.0, 0.0, 1.0],
        );
        let rotate_tool = Button::new(
            [position[0], position[1] + button_size[1] * 2.0],
            button_size[0],
            button_size[1],
            [0.0, 0.0, 1.0, 1.0],
        );
        let draw_tool = Button::new(
            [position[0], position[1] + button_size[1] * 3.0],
            button_size[0],
            button_size[1],
            [0.0, 1.0, 1.0, 1.0],
        );
        let mut buttons = Vec::new();
        buttons.push(move_tool);
        buttons.push(scale_tool);
        buttons.push(rotate_tool);
        buttons.push(draw_tool);

        buttons
    }

    pub fn draw(&self, graphics: &mut GfxGraphics<Resources, CommandBuffer>, transform: Matrix2d) {
        for button in &self.buttons {
            button.draw(graphics, transform);
        }
    }
}
