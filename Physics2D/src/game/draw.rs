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
const LIGHT_CERISE_COLOR: &str = "ec5f99";

pub fn draw(event: &Event, window: &mut PistonWindow) {
    // Update application window.
    window.draw_2d(event, |context, graphics, _| {
        // Fill the window with white colour.
        piston_window::clear(CERISE_COLOR, graphics);

        // TODO: put all graphics shit here
        play_bar(Vec2d::from([100.0, 100.0]));
    });
}

// TODO: 
fn play_bar(pos: Vec2d) { // pos is upper-left corner
    let size: Vec2d = Vec2d::from([30.0, 30.0]);
    let play_pos: Vec2d = Vec2d::from([pos[0] + 10.0, pos[1] - 10.0]);
    let restart_pos: Vec2d = Vec2d::from([pos[0] + size[0] + 10.0, pos[1] - size[1] - 10.0]);

    let mut play_button = Button::new(play_pos, size, color::hex(LIGHT_CERISE_COLOR));
    let mut restart_button = Button::new(restart_pos, size, color::hex(LIGHT_CERISE_COLOR));
}

// TODO: 
fn tool_box() {

}