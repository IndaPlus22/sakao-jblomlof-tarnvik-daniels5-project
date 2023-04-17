use piston::Event;
use piston_window::PistonWindow;

use super::ui_objects::Objects;

pub fn draw(event: &Event, window: &mut PistonWindow) {
    window.draw_2d(event, |context, graphics, _| {
    let buttons = Objects::new().buttons;
        for i in 0..buttons.len() {
            buttons[i].draw(graphics, context.transform);
        }
    });
}