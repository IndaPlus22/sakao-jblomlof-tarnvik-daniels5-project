use piston::Event;
use piston_window::PistonWindow;

use super::ui_objects::Objects;

pub fn draw(event: &Event, window: &mut PistonWindow, objects: &mut Objects) {
    window.draw_2d(event, |context, graphics, _| {
        for i in 0..objects.buttons.len() {
            objects.buttons[i].draw(graphics, context.transform);
        }
    });
}