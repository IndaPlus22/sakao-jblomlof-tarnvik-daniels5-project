use piston::{Event, PressEvent, ReleaseEvent, MouseCursorEvent};

use crate::game::objects;

use super::ui_button::Button;
use super::ui_objects::Objects;

pub fn input(event: &Event){
    let mut objects = Objects::new();

    if let Some(button) = event.press_args() {
        println!("Pressed {:?}", button);
    }
    if let Some(button) = event.release_args() {
        println!("Released {:?}", button);
    }
    if let Some(pos) = event.mouse_cursor_args() {
        objects.buttons[0].check_hover(pos);
        objects.buttons[1].check_hover(pos);
    }
    
}