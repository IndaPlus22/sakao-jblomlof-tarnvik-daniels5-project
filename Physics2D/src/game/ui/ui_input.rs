use piston::{Event, MouseCursorEvent, PressEvent, ReleaseEvent};

use crate::game::{GameState, Variables};

use super::ui_objects::Objects;

pub fn input(event: &Event, objects: &mut Objects, variables: &mut Variables) {
    if let Some(pos) = event.mouse_cursor_args() {
        for i in 0..5 {
            objects.buttons[i].check_hover(pos);
            // println!("Button {}: hover={}", i, objects.buttons[i].hover);
        }
        for i in 0..objects.tool_bar.buttons.len() {
            objects.tool_bar.buttons[i].check_hover(pos);
            // println!("TOOLBAR: Button {}: hover={}", i, objects.tool_bar.buttons[i].hover);
        }
    }
    if let Some(button) = event.press_args() {
        if objects.buttons[0].hover {
            variables.game_state = GameState::Running;
        } else if objects.buttons[1].hover{
            variables.game_state  = GameState::Paused;
        } else if objects.buttons[2].hover{
            //TODO: Save button functionality aka save the current objects in a file
        } else if objects.buttons[3].hover{
            //TODO: Restart button functionality aka reset the simulation to the last saved state 
            //TODO: If there is no saved state defualt is an empty file? 
        } else if objects.buttons[4].hover{
            //TODO: Clear button functionality aka delete all objects, making the thing blank
        }
        
        for i in 0..objects.tool_bar.buttons.len() {
            if objects.tool_bar.buttons[i].hover {
                println!("TOOLBAR: Button {} pressed", i);
            }
        }
    }
    // if let Some(button) = event.release_args() {
    //     println!("Released {:?}", button);
    // }
}
