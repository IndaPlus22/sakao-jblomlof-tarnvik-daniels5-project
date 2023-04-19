use piston::{Event, PressEvent, ReleaseEvent, MouseCursorEvent};

use crate::game::{GameState, Variables};

use super::ui_objects::Objects;

pub fn input(event: &Event, objects: &mut Objects, variables: &mut Variables){

    if let Some(pos) = event.mouse_cursor_args() {
        for i in 0..2 {
            objects.buttons[i].check_hover(pos);
            println!("Button {}: hover={}", i, objects.buttons[i].hover);
        }
    }
    if let Some(button) = event.press_args() {
        if objects.buttons[0].hover{
            variables.game_state = GameState::Running;
        } else if objects.buttons[1].hover{
            variables.game_state  = GameState::Paused;
        } 
    }
    // if let Some(button) = event.release_args() {
    //     println!("Released {:?}", button);
    // }
}