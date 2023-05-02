
use graphics::{types::{Vec2d, Matrix2d}};
use opengl_graphics::GlGraphics;

use crate::game::draw::abs_to_rel_pos;

use super::ui_button::Button;

const SPRITES: [&str; 1] = ["sprites/ui/tool_bar/bro.png"];

pub struct Toolbar {
    button_size: Vec2d,
    position: Vec2d,
    pub buttons: Vec<Button>,
    pub selected_poses: Vec<Vec2d>,
    // textures: Vec<Texture>,
}

impl Toolbar {
    pub fn new(button_size: Vec2d, position: Vec2d) -> Toolbar {
        // let textures = load_sprites();
        let buttons = init_buttons(button_size, position);
        let selected_poses = Vec::new();
        Toolbar {
            button_size,
            position,
            buttons,
            selected_poses, // Only for drawtool
        }
    }

    pub fn draw(&self, graphics: &mut GlGraphics, transform: Matrix2d) {
        for button in &self.buttons {
            button.draw(graphics, transform);
        }
    }

    pub fn add_selected_button(&mut self, pos: Vec2d, win_size: Vec2d) {
        self.selected_poses.push(abs_to_rel_pos(pos, win_size));
    }
}

// fn load_sprites() -> Vec<Texture> {
    // let img = image::open("sprites/ui/tool_bar/test.png").unwrap();
    // let image_width = img.width();
    // let image_height = img.height();
    // let image_rgba = img.to_rgba8();

    // //A texture to use with the image
    // let texture = Texture::from_image(&image_rgba, &TextureSettings::new());
    // vec![texture]
 // }

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
            &SPRITES[0],
        );
        buttons.push(button);
    }
    buttons
}
