use graphics::types::Vec2d;
use piston::{Event, MouseCursorEvent, PressEvent};

use crate::game::{simulation::objects, GameState, Tool, Variables};

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

        variables.last_mouse_pos = pos;
    }
    if let Some(button) = event.press_args() {
        // for matching tools
        match_tools(variables, button, objects, variables.win_size);

        if objects.buttons[0].hover {
            variables.game_state = GameState::Running;
        } else if objects.buttons[1].hover {
            variables.game_state = GameState::Paused;
        } else if objects.buttons[2].hover {
            //TODO: Save button functionality aka save the current objects in a file
        } else if objects.buttons[3].hover {
            //TODO: Restart button functionality aka reset the simulation to the last saved state
            //TODO: If there is no saved state defualt is an empty file?
        } else if objects.buttons[4].hover {
            //TODO: Clear button functionality aka delete all objects, making the thing blank
        }

        // Should not be able to interact with the tool bar if the game is running
        if variables.game_state == GameState::Paused {
            if objects.tool_bar.buttons[0].hover {
                // TODO: Move tool
                println!("Move tool selected");
                variables.current_tool = Tool::Move;
            } else if objects.tool_bar.buttons[1].hover {
                // TODO: scale tool
                println!("Scale tool selected");
                variables.current_tool = Tool::Scale;
            } else if objects.tool_bar.buttons[2].hover {
                // TODO: rotate tool
                println!("Rotate tool selected");
                variables.current_tool = Tool::Rotate;
            } else if objects.tool_bar.buttons[3].hover {
                // TODO: Draw tool
                println!("Draw tool selected");
                variables.current_tool = Tool::Draw;
                objects.tool_bar.selected_poses.clear();
            }
        }
    }
    // if let Some(button) = event.release_args() {
    //     println!("Released {:?}", button);
    // }
}

fn match_tools(variables: &mut Variables, button: piston::Button, objects: &mut Objects, win_size: Vec2d) {
    match variables.current_tool {
        Tool::Move => {
            // TODO: Check all objects for hover
            // If hover, select object
            // If selected make able to move object with mouse
        }
        Tool::Scale => {
            // TODO: Same as move but with scale
        }
        Tool::Rotate => {
            // TODO: Same as move but with rotate
        }
        Tool::Draw => {
            if button == piston::Button::Mouse(piston::MouseButton::Left) {
                println!("Left mouse button pressed");
                objects
                    .tool_bar
                    .add_selected_button(variables.last_mouse_pos, win_size);

                println!("Selected poses: {:?}", objects.tool_bar.selected_poses);
            }
            if button == piston::Button::Keyboard(piston::Key::Return) {
                variables.objects.push(Box::new(objects::Rectangle::new(
                    objects.tool_bar.selected_poses.clone(),
                    10.0,
                )));
                objects.tool_bar.selected_poses.clear();
                variables.current_tool = Tool::None;
                println!("made polygon");
            }
        }
        _ => {}
    }
}

fn check_hover_obj() {

}
