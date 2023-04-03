mod button;

use graphics::{types::Vec2d, color};
use opengl_graphics::OpenGL;
use piston::{
    input::{RenderEvent, UpdateEvent},
    window::WindowSettings,
};
use piston_window::{Event, PistonWindow};

use button::Button;

const CERISE_COLOR: [f32; 4] = [232.0/255.0, 61.0/255.0, 132.0/255.0, 1.0];
const LIGHT_CERISE: &str = "ec5f99";


pub fn draw(event: &Event, window: &mut PistonWindow) {
    // Update application window.
    window.draw_2d(event, |context, graphics, _| {
        // Fill the window with white colour.
        piston_window::clear(CERISE_COLOR, graphics);

        // TODO: put all graphics shit here
        let mut buttons = play_bar(Vec2d::from([0.0, 0.0]));
        for i in 0..buttons.len() {
            buttons[i].draw(graphics, context.transform);
        }
    });
}

// TODO: 
pub fn init() {
    let mut buttons = play_bar(Vec2d::from([0.0, 0.0]));
}

// TODO:
fn play_bar(pos: Vec2d) -> [Button; 2] { // pos is upper-left corner
    let size: Vec2d = Vec2d::from([40.0, 40.0]);
    let play_pos: Vec2d = Vec2d::from([pos[0] + 40.0, pos[1]]);
    let restart_pos: Vec2d = Vec2d::from([pos[0] + size[0] + 60.0, pos[1]]);

    let mut play_button = Button::new(play_pos, size, color::hex(LIGHT_CERISE));
    let mut restart_button = Button::new(restart_pos, size, color::hex(LIGHT_CERISE));

    [play_button, restart_button]
}

// TODO:
fn tool_box() {

}