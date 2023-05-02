
use std::{fs::{OpenOptions, File}, io::Write};

use piston::{Event, MouseCursorEvent, PressEvent, ReleaseEvent};

use crate::game::{GameState, Variables, simulation::traits::{Object, self}};

use super::ui_objects::Objects;

use serde::{Serialize};

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
            Save(&mut variables.objects);
        } else if objects.buttons[3].hover{
            Load();
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

pub fn Save (objects: &mut Vec<Box<dyn traits::Object>>) -> std::io::Result<()> {
    let mut file = File::create("objects.json")?;
    for ob in objects{

        let shape = ob.gettype();
        file.write_all(shape.as_bytes())?;
        file.write_all(b"\n")?;

        if shape == "Rectangle" {

        } else if shape == Circle

        let center = ob.getcenter();
        let center_json = serde_json::to_string(&center).unwrap();
        file.write_all(center_json.as_bytes())?;
        file.write_all(b"\n")?;
        
        let velocity = ob.getvel();
        let velocity_json = serde_json::to_string(&velocity).unwrap();
        file.write_all(velocity_json.as_bytes())?;
        file.write_all(b"\n")?;
        
        let mass = ob.get_mass();
        let mass_json = serde_json::to_string(&mass).unwrap();
        file.write_all(mass_json.as_bytes())?;
        file.write_all(b"\n")?;
    }
    Ok(())
}

//TODO: Restart button functionality aka reset the simulation to the last saved state 
//TODO: If there is no saved state defualt is an empty file? 
pub fn Load (){

}
