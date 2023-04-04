use piston::{Event, PressEvent, MouseCursorEvent, ReleaseEvent, ButtonArgs};

pub fn input(event: &Event) {
    if let Some(button) = event.press_args() {
        println!("Pressed {:?}", button);
    }
    if let Some(button) = event.release_args() {
        println!("Released {:?}", button);
    }
    if let Some(pos) = event.mouse_cursor_args() {
        println!("Mouse moved to {:?}", pos);
    }
}
