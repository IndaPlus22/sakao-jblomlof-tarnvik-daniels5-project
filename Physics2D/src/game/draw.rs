use opengl_graphics::OpenGL;
use piston::{
    input::{RenderEvent, UpdateEvent},
    window::WindowSettings,
};
use piston_window::{Event, PistonWindow};

const CERISE_COLOR: [f32; 4] = [232.0/255.0, 61.0/255.0, 132.0/255.0, 1.0];

pub fn draw(event: &Event, window: &mut PistonWindow) {

    // Update application window.
    window.draw_2d(event, |context, graphics, _| {
        // Fill the window with white colour.
        piston_window::clear(CERISE_COLOR, graphics);
    });
}