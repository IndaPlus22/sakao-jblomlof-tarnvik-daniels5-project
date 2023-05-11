use graphics::types::{Matrix2d, Vec2d};
use opengl_graphics::GlGraphics;
use piston::RenderArgs;

use crate::{
    game::draw::{abs_to_rel_pos, draw_circle_color},
    vector::vector::Vec2,
};

use super::ui_button::Button;

const SPRITES: [&str; 5] = [
    "sprites/ui/tool_bar/move.png", 
    "sprites/ui/tool_bar/scale.png", 
    "sprites/ui/tool_bar/rotate.png", 
    "sprites/ui/tool_bar/draw.png",
    "sprites/ui/tool_bar/delete.png",
    ];

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

    pub fn draw(&self, graphics: &mut GlGraphics, transform: Matrix2d, args: &RenderArgs) {
        for button in &self.buttons {
            button.draw(graphics, transform);
        }
        for pos in &self.selected_poses {
            draw_circle_color(
                Vec2::new(pos[0], pos[1]),
                0.01,
                [48. / 255., 110. / 255., 122. / 255., 0.7],
                transform,
                graphics,
                args,
            )
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

    for i in 0..5 {
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
            &SPRITES[i],
        );
        buttons.push(button);
    }
    buttons
}
