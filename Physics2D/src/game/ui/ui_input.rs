use std::{fs::{File}, io::{Write, BufReader, BufRead}};

use piston::{Event, MouseCursorEvent, PressEvent};

use crate::{game::{GameState, Variables, simulation::{traits::{self}, objects::{Rectangle, Circle}}}, vector::vector::Vec2};

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
    if let Some(_button) = event.press_args() {
        if objects.buttons[0].hover { 
            //PLAY BUTTON (starts/un pauses simulation)
            variables.game_state = GameState::Running;
        } else if objects.buttons[1].hover{
            //PAUSE BUTTON (pauses simulation) 
            variables.game_state  = GameState::Paused;
        } else if objects.buttons[2].hover{ 
            //SAVE BUTTON (saves current objects to file)
            match save(&mut variables.objects) {
                Ok(()) => (),
                Err(e) => eprintln!("Error saving objects: {}", e),
            }
        } else if objects.buttons[3].hover{ 
            //RESET BUTTON (resets simulation to saved state)
            match load(&mut variables.objects) {
                Ok(()) => (),
                Err(e) => eprintln!("Error loading objects: {}", e),
            }
        } else if objects.buttons[4].hover{ 
            //CLEAR BUTTON (clear all objects from the simulation)
            variables.objects.clear();
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

pub fn save(objects: &mut Vec<Box<dyn traits::Object>>) -> std::io::Result<()> {
    let mut obj_vec = Vec::new();
    for ob in objects {
        let shape = ob.gettype();
        let center = ob.getcenter();
        let velocity = ob.getvel();
        let mass = ob.get_mass();

        let mut obj_map = serde_json::Map::new();
        obj_map.insert("shape".to_string(), serde_json::json!(shape));

        if shape == "Rectangle" {
            let vertices = ob.getvertices();
            obj_map.insert("vertices".to_string(), serde_json::json!(vertices));
        } else if shape == "Circle" {
            let radius = ob.getradius();
            obj_map.insert("radius".to_string(), serde_json::json!(radius));
        }

        obj_map.insert("center".to_string(), serde_json::json!(center));
        obj_map.insert("velocity".to_string(), serde_json::json!(velocity));
        obj_map.insert("mass".to_string(), serde_json::json!(mass));
        
        obj_vec.push(serde_json::json!(obj_map));
    }

    let obj_json = serde_json::to_string_pretty(&obj_vec)?;
    let mut file = File::create("objects.json")?;
    file.write_all(obj_json.as_bytes())?;
    Ok(())
}

pub fn load(objects: &mut Vec<Box<dyn traits::Object>>) -> std::io::Result<()> {
    let file = File::open("objects.json")?;
    let reader = BufReader::new(file);
    let json_string: String = reader.lines().map(|line| line.unwrap()).collect();
    let json_objs: Vec<serde_json::Value> = serde_json::from_str(&json_string)?;

    for obj in json_objs {
        let shape = obj["shape"].as_str().unwrap();
        let center: Vec2 = serde_json::from_value(obj["center"].clone())?;
        let velocity: Vec2 = serde_json::from_value(obj["velocity"].clone())?;
        let mass: f64 = serde_json::from_value(obj["mass"].clone())?;
        let mut new_obj: Box<dyn traits::Object>;
        if shape == "Rectangle" {
            let vertices: Vec<[f64; 2]> = serde_json::from_value(obj["vertices"].clone())?;
            new_obj = Box::new(Rectangle::new(vertices, mass));
        } else {
            let radius: f64 = serde_json::from_value(obj["radius"].clone())?;
            new_obj = Box::new(Circle::new(center, radius, mass));
        }
        new_obj.setvel(velocity);
        objects.push(new_obj);
    }
    Ok(())
}