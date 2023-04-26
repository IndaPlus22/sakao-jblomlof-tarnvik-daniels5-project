use piston::{Event, MouseCursorEvent, PressEvent, ReleaseEvent};

use crate::game::{GameState, Tool, Variables, simulation::objects};

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
                variables.current_tool = Tool::Move;
            } else if objects.tool_bar.buttons[1].hover {
                // TODO: scale tool
                variables.current_tool = Tool::Scale;
            } else if objects.tool_bar.buttons[2].hover {
                // TODO: rotate tool
                variables.current_tool = Tool::Rotate;
            } else if objects.tool_bar.buttons[3].hover {
                // TODO: Draw tool
                variables.current_tool = Tool::Draw;
            }
        }

        match variables.current_tool {
            Tool::Move => {}
            Tool::Scale => {}
            Tool::Rotate => {}
            Tool::Draw => {
                variables.objects.push(Box::new(objects::Rectangle::new(
                    vec![[0.15, 0.1], [0.15, 0.2], [0.25, 0.2], [0.25, 0.1]],
                    10.0,
                )));
            }
            _ => {}
        }
    }
    // if let Some(button) = event.release_args() {
    //     println!("Released {:?}", button);
    // }
}
