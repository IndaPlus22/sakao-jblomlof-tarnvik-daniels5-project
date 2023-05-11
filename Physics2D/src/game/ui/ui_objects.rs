use graphics::types::Vec2d;

use super::{ui_button::Button, toolbar::Toolbar};

const SPRITES: [&str; 5] = [
    "sprites/ui/play_bar/play.png", 
    "sprites/ui/play_bar/pause.png",
    "sprites/ui/play_bar/save.png", 
    "sprites/ui/play_bar/reset.png", 
    "sprites/ui/play_bar/clear.png"];

pub struct Objects {
    pub buttons: Vec<Button>,
    pub tool_bar: Toolbar,
}

impl Objects {
    pub fn new() -> Objects {
        Objects {
            buttons: Self::create_buttons(),
            tool_bar: Toolbar::new([40.0, 40.0], [10.0, 60.0]),
        }
    }

    pub fn create_buttons () -> Vec<Button> {
        let pos = Vec2d::from([0.0, 0.0]);
        let width = 40.0;
        let height = 40.0;
    
        let mut buttons = Vec::new();
    
        for i in 0..5 {
            let button_pos = Vec2d::from([pos[0] + 10.*(i as f64 + 1.) + width*(i as f64), pos[1] + 10.]);
            let button = Button::new(button_pos, width, height, [0.0,0.0,0.0,1.0], SPRITES[i]);
            buttons.push(button);
        }
    
        buttons
    }
}
