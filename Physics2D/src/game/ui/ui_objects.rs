use piston_window::types::Vec2d;

use super::{ui_button::Button, toolbar::Toolbar};

pub struct Objects {
    pub buttons: [Button; 5]
    pub tool_bar: Toolbar,
}

impl Objects {
    pub fn new() -> Objects {
        Objects {
            buttons: Self::create_buttons(),
            tool_bar: Toolbar::new([30.0, 30.0], [10.0, 60.0]),
        }
    }


    pub fn create_buttons () -> [Button; 5]{
        let pos = Vec2d::from([0.0, 0.0]);
        let width = 40.0;
        let height = 40.0;
        let play_pos: Vec2d = Vec2d::from([pos[0] + 10.0, pos[1] + 10.0]);
        let pause_pos: Vec2d = Vec2d::from([pos[0]+ width + 20.0, pos[1] + 10.0]);
        let save_pos: Vec2d = Vec2d::from([pos[0]+ 2. * width + 30.0, pos[1] + 10.0]);
        let restart_pos: Vec2d = Vec2d::from([pos[0]+ 3. * width + 40.0, pos[1] + 10.0]);
        let clear_pos: Vec2d = Vec2d::from([pos[0]+ 4. * width + 50.0, pos[1] + 10.0]);

        let play_button = Button::new(play_pos, width, height, [0.0,0.0,0.0,1.0]);
        let pause_button = Button::new(pause_pos, width, height, [0.0,0.0,0.0,1.0]);
        let save_button = Button::new(save_pos, width, height, [0.0,0.0,0.0,1.0]);
        let restart_button = Button::new(restart_pos, width, height, [0.0,0.0,0.0,1.0]);
        let clear_button = Button::new(clear_pos, width, height, [0.0,0.0,0.0,1.0]);


        [play_button, pause_button, save_button, restart_button, clear_button]
    }
}
