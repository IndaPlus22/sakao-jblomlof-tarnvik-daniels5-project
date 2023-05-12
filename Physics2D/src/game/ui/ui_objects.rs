use graphics::types::{Vec2d, Matrix2d};
use opengl_graphics::GlGraphics;
use piston::RenderArgs;

use crate::{game::draw::{abs_to_rel_pos, draw_circle_color}, vector::vector::Vec2};

use super::{ui_button::Button};

const PLAY_SPRITES: [&str; 5] = [
    "sprites/ui/play_bar/play.png", 
    "sprites/ui/play_bar/pause.png",
    "sprites/ui/play_bar/save.png", 
    "sprites/ui/play_bar/reset.png", 
    "sprites/ui/play_bar/clear.png"];

const TOOL_SPRITES: [&str; 5] = [
    "sprites/ui/tool_bar/move.png", 
    "sprites/ui/tool_bar/scale.png", 
    "sprites/ui/tool_bar/rotate.png", 
    "sprites/ui/tool_bar/draw.png",
    "sprites/ui/tool_bar/delete.png"];

pub struct Objects {
    pub play_buttons: Vec<Button>,
    pub tool_buttons: Vec<Button>,
    pub selected_poses: Vec<Vec2d>
}

impl Objects {
    pub fn new() -> Objects {
        let selected_poses = Vec::new();

        Objects {
            play_buttons: Button::init_buttons(
                [40.0, 40.0], 
                [10.0, 10.0], 
                PLAY_SPRITES, 
                [1.25, 0.0]),
            tool_buttons: Button::init_buttons(
                [40.0, 40.0], 
                [10.0, 60.0], 
                TOOL_SPRITES, 
                [0.0, 1.0]),
            selected_poses 
        }
    }

    pub fn add_selected_button(&mut self, pos: Vec2d, win_size: Vec2d) {
        self.selected_poses.push(abs_to_rel_pos(pos, win_size));
    }

    pub fn draw_selected_poses (&self, graphics: &mut GlGraphics, transform: Matrix2d, args: &RenderArgs){
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

    /*pub fn get_play_buttons (&self) -> Vec<Button>{
        self.play_buttons
    }

    pub fn get_tool_buttons (&self) -> Vec<Button>{
        self.tool_buttons
    }

    pub fn get_selected_poses (&self) -> Vec<Vec2d>{
        self.selected_poses
    }*/
}
