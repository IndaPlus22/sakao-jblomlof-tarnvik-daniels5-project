use graphics::color;
use piston_window::types::Vec2d;

use super::ui_button::Button;

pub struct Objects {
    pub buttons: [Button; 2]
}

impl Objects {
    pub fn new () -> Objects{
        Objects{
            buttons: Self::create_buttons()
        }        

    }

    pub fn create_buttons () -> [Button; 2]{
        let pos = Vec2d::from([0.0, 0.0]);
        let width = 40.0;
        let height = 40.0;
        let play_pos: Vec2d = Vec2d::from([pos[0] + 10.0, pos[1] + 10.0]);
        let restart_pos: Vec2d = Vec2d::from([pos[0]+ width + 20.0, pos[1] + 10.0]);

        let mut play_button = Button::new(play_pos, width, height, [0.0,0.0,0.0,1.0]);
        let mut restart_button = Button::new(restart_pos, width, height, [0.0,0.0,0.0,1.0]);

        [play_button, restart_button]
    }
}