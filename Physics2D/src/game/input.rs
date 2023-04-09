use piston::{Event, PressEvent, MouseCursorEvent, ReleaseEvent, ButtonArgs};

pub struct Input {
    mouse_pos: [f64; 2]
}

impl Input {
    pub fn new () -> Input{
        Input { mouse_pos: [0.0, 0.0] }
    }

    pub fn input(&mut self, event: &Event){
        
        if let Some(button) = event.press_args() {
            println!("Pressed {:?}", button);
        }
        if let Some(button) = event.release_args() {
            println!("Released {:?}", button);
        }
        if let Some(pos) = event.mouse_cursor_args() {
            println!("Mouse moved to {:?}", pos);
            self.mouse_pos = pos;
        }
        
    }
}
