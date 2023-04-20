use gfx_device_gl::{CommandBuffer, Resources};
use gfx_graphics::{GfxGraphics};
use graphics::Image;
use opengl_graphics::Texture;
use piston_window::types::{Matrix2d, Vec2d};
use std::path::Path;

use super::ui_button::Button;

const SPRITES: [&str; 1] = ["sprites/ui/tool_bar/test.png"];

pub struct Toolbar {
    button_size: Vec2d,
    position: Vec2d,
    pub buttons: Vec<Button>,
    // textures: Vec<Texture>,
}

impl Toolbar {
    pub fn new(button_size: Vec2d, position: Vec2d) -> Toolbar {
        // let textures = load_sprites();
        let buttons = init_buttons(button_size, position);
        Toolbar {
            button_size,
            position,
            buttons,
        }
    }

    pub fn draw(&self, graphics: &mut GfxGraphics<Resources, CommandBuffer>, transform: Matrix2d) {
        for button in &self.buttons {
            button.draw(graphics, transform);
        }
    }
}

fn load_sprites() -> Vec<Texture> {
    let image = Image::new().rect([0.0, 0.0, 100.0, 100.0]);
    //A texture to use with the image
    let texture = Texture::from_path(Path::new(SPRITES[0]), &opengl_graphics::TextureSettings::new()).unwrap();
    vec![texture]
}

fn init_buttons(button_size: Vec2d, position: Vec2d) -> Vec<Button> {
    let mut buttons = Vec::new();

    for i in 0..4 {
        let button = Button::new(
            [position[0], position[1] + button_size[1] * i as f64],
            button_size[0],
            button_size[1],
            [
                0. + i as f32 / 4.,
                0. + i as f32 / 4.,
                0. + i as f32 / 4.,
                1.0,
            ],
        );
        buttons.push(button);
    }
    buttons
}
