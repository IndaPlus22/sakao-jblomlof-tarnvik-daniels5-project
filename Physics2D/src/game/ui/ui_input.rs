use piston::{Event, PressEvent, ReleaseEvent, MouseCursorEvent};

use super::ui_objects::Objects;

pub fn input(event: &Event, objects: &mut Objects){

    if let Some(pos) = event.mouse_cursor_args() {
        for i in 0..2 {
            objects.buttons[i].check_hover(pos);
            println!("Button {}: hover={}", i, objects.buttons[i].hover);
        }
    }
    if let Some(button) = event.press_args() {
        for i in 0..2 {
            println!("Button {}: hover={}", i, objects.buttons[i].hover);
            if objects.buttons[i].hover{
                println!("Wow you pressed a button");
                break;
            }
        }
    }
    if let Some(button) = event.release_args() {
        println!("Released {:?}", button);
    }
}