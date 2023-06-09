use graphics::types::Vec2d;
use std::{
    fs::{File},
    io::{BufRead, BufReader, Write},
};

use piston::{Event, MouseCursorEvent, PressEvent, ReleaseEvent};

use crate::{
    game::{
        simulation::{
            objects::{self, approx_circle_hitbox, Circle, Polygon},
            traits::{self},
        },
        GameState, Tool, Variables,
    },
    vector::vector::Vec2,
};

use super::ui_objects::Objects;

pub fn input(event: &Event, objects: &mut Objects, variables: &mut Variables) {
    if let Some(pos) = event.mouse_cursor_args() {
        for i in 0..5 {
            objects.play_buttons[i].check_hover(pos);
        }
        for i in 0..objects.tool_buttons.len() {
            objects.tool_buttons[i].check_hover(pos);
        }
        for i in 0..variables.objects.len() {
            if variables.objects[i].get_selected(0) == 1 {
                // Move tool
                variables.objects[i].set_pos(Vec2::new(
                    pos[0] / variables.win_size[0],
                    pos[1] / variables.win_size[1],
                ))
            } else if variables.objects[i].get_selected(1) == 1 {
                // Scale tool
                if variables.objects[i].gettype() == "Circle" {
                    rescale_circle(variables, i, pos);
                } else if variables.objects[i].gettype() == "Polygon" {
                    rescale_polygon(variables, i, pos);
                }
            } else if variables.objects[i].get_selected(2) == 1 {
                // Rotate tool (only works for rects(polygons))
                if variables.objects[i].gettype() == "Polygon" {
                    rotate_polygon(variables, i, pos);
                }
            }

            variables.objects[i].check_hover(Vec2::new(
                pos[0] / variables.win_size[0],
                pos[1] / variables.win_size[1],
            ));
        }

        variables.last_mouse_pos = pos;
    }

    if let Some(button) = event.press_args() {
        // for matching tools
        match_tools(variables, button, objects, variables.win_size);

        if objects.play_buttons[0].get_hover() {
            variables.game_state = GameState::Running;
        } else if objects.play_buttons[1].get_hover() {
            //PAUSE BUTTON (pauses simulation)
            variables.game_state = GameState::Paused;
        } else if objects.play_buttons[2].get_hover() {
            //SAVE BUTTON (saves current objects to file)
            match save(&mut variables.objects) {
                Ok(()) => (),
                Err(e) => eprintln!("Error saving objects: {}", e),
            }
        } else if objects.play_buttons[3].get_hover(){
            //RESET BUTTON (resets simulation to saved state)
            match load(&mut variables.objects) {
                Ok(()) => (),
                Err(e) => eprintln!("Error loading objects: {}", e),
            }
        } else if objects.play_buttons[4].get_hover(){
            //CLEAR BUTTON (clear all objects from the simulation)
            variables.objects.clear();
        }

        // Should not be able to interact with the tool bar if the game is running
        if variables.game_state == GameState::Paused {
            if objects.tool_buttons[0].get_hover() {
                // : Move tool
                println!("Move tool selected");
                variables.current_tool = Tool::Move;
            } else if objects.tool_buttons[1].get_hover() {
                // : scale tool
                println!("Scale tool selected");
                variables.current_tool = Tool::Scale;
            } else if objects.tool_buttons[2].get_hover() {
                // : rotate tool
                println!("Rotate tool selected");
                variables.current_tool = Tool::Rotate;
            } else if objects.tool_buttons[3].get_hover() {
                // : draw tool
                println!("Draw tool selected");
                variables.current_tool = Tool::Draw;
                objects.selected_poses.clear();
            } else if objects.tool_buttons[4].get_hover() {
                // : delete tool
                println!("Delete tool selected");
                variables.current_tool = Tool::Delete;
            }
        }

        // DEBUG BUTTONS
        // if button == piston::Button::Keyboard(piston::Key::T) {
        //     variables.objects[0].rescale(1.2);

        // }
        // --------------------------------------
    }
    if let Some(button) = event.release_args() {
        if button == piston::Button::Mouse(piston::MouseButton::Left) {
            for i in 0..variables.objects.len() {
                for j in 0..3 {
                    if variables.objects[i].get_selected(j) == 1 {
                        if j == 1 {
                            // if it was scale fix circle center thingy
                            let v = variables.objects[i].getvertices();
                            variables.objects[i].set_circle_center(approx_circle_hitbox(&v));
                            variables.objects[i].calclulate_inertia();
                            // println!("fixed");
                        }
                        variables.objects[i].set_selected(j, 0);
                    }
                }
            }
        }
    }
}

fn match_tools(
    variables: &mut Variables,
    button: piston::Button,
    objects: &mut Objects,
    win_size: Vec2d,
) {
    match variables.current_tool {
        Tool::Move => {
            select_object(variables, button, 0);
            // If hover, select object
            // If selected make able to move object with mouse
        }
        Tool::Scale => {
            // TODO: Same as move but with scale
            select_object(variables, button, 1);
        }
        Tool::Rotate => {
            // TODO: Same as move but with rotate
            select_object(variables, button, 2);
        }
        Tool::Draw => {
            if button == piston::Button::Mouse(piston::MouseButton::Left) {
                println!("Left mouse button pressed");
                objects.add_selected_button(variables.last_mouse_pos, win_size);

                println!("Selected poses: {:?}", objects.selected_poses);
            }
            if button == piston::Button::Keyboard(piston::Key::Return) {
                variables.objects.push(Box::new(objects::Polygon::new(
                    objects.selected_poses.clone(),
                    10.0,
                )));
                objects.selected_poses.clear();
                variables.current_tool = Tool::None;
                println!("made polygon");
            }
        }
        Tool::Delete => {
            let hovered_i = check_hover_obj(variables);
            if let Some(i) = hovered_i {
                if button == piston::Button::Mouse(piston::MouseButton::Left) {
                    variables.objects.remove(i);
                }
            }
        }
        _ => {}
    }
}

fn rescale_polygon(variables: &mut Variables, i: usize, m_pos: Vec2d) {
    let local_m_pos = Vec2::new(
        m_pos[0] / variables.win_size[0] - variables.objects[i].get_pos().x,
        m_pos[1] / variables.win_size[1] - variables.objects[i].get_pos().y,
    );
    // one vertex with local coordinates
    let local_vertex = Vec2::new(
        variables.objects[i].getvertices()[0][0] - variables.objects[i].get_pos().x,
        variables.objects[i].getvertices()[0][1] - variables.objects[i].get_pos().y,
    );

    variables.objects[i].rescale(local_m_pos.length() / local_vertex.length());
}

fn rescale_circle(variables: &mut Variables, i: usize, m_pos: Vec2d) {
    let radius = (variables.objects[i].get_pos().x - m_pos[0] / variables.win_size[0]).powf(2.0)
        + (variables.objects[i].get_pos().y - m_pos[1] / variables.win_size[1]).powf(2.0);
    // TODO: Set radius
    let old_radius = variables.objects[i].getradius();
    variables.objects[i].rescale(radius.sqrt() / old_radius);
}

fn rotate_polygon(variables: &mut Variables, i: usize, m_pos: Vec2d) {
    let angle: f64 = m_pos[1] / 10000.;
    println!("angle: {}", angle);
    // TODO: fix so it changes the objects vertices and not the copy (getvertices)
    variables.objects[i].rotate(angle);
}

fn check_hover_obj(variables: &mut Variables) -> Option<usize> {
    for i in 0..variables.objects.len() {
        if variables.objects[i].get_hover() {
            return Some(i);
        }
    }
    None
}

fn select_object(variables: &mut Variables, button: piston::Button, func: u8) {
    let hovered_i = check_hover_obj(variables);
    if let Some(i) = hovered_i {
        if button == piston::Button::Mouse(piston::MouseButton::Left) {
            variables.objects[i].set_selected(func, 1);
        }
    }
}

pub fn save(objects: &mut Vec<Box<dyn traits::Object>>) -> std::io::Result<()> {
    let mut obj_vec = Vec::new();
    for ob in objects {
        let shape = ob.gettype();
        let center = ob.getcenter();
        let velocity = ob.getvel();
        let mass = ob.get_mass();
        let static_shape = ob.get_static();
        let angular_velocity = ob.get_angular_vel();

        let mut obj_map = serde_json::Map::new();
        obj_map.insert("shape".to_string(), serde_json::json!(shape));

        if shape == "Polygon" {
            let vertices = ob.getvertices();
            obj_map.insert("vertices".to_string(), serde_json::json!(vertices));
        } else if shape == "Circle" {
            let radius = ob.getradius();
            obj_map.insert("radius".to_string(), serde_json::json!(radius));
        }

        obj_map.insert("center".to_string(), serde_json::json!(center));
        obj_map.insert("velocity".to_string(), serde_json::json!(velocity));
        obj_map.insert("mass".to_string(), serde_json::json!(mass));
        obj_map.insert("static shape".to_string(), serde_json::json!(static_shape));
        obj_map.insert("angular velocity".to_string(), serde_json::json!(angular_velocity));
        obj_vec.push(serde_json::json!(obj_map));
    }

    let obj_json = serde_json::to_string_pretty(&obj_vec)?;
    let mut file = File::create("objects.json")?;
    file.write_all(b"")?;
    file.write_all(obj_json.as_bytes())?;
    Ok(())
}

pub fn load(objects: &mut Vec<Box<dyn traits::Object>>) -> std::io::Result<()> {
    let file = File::open("objects.json")?;
    let reader = BufReader::new(file);
    let json_string: String = reader.lines().map(|line| line.unwrap()).collect();
    let json_objs: Vec<serde_json::Value> = serde_json::from_str(&json_string)?;
    objects.clear();

    for obj in json_objs {
        let shape = obj["shape"].as_str().unwrap();
        let center: Vec2 = serde_json::from_value(obj["center"].clone())?;
        let velocity: Vec2 = serde_json::from_value(obj["velocity"].clone())?;
        let mass: f64 = serde_json::from_value(obj["mass"].clone())?;
        let static_shape = serde_json::from_value(obj["static shape"].clone())?;
        let angular_velocity = serde_json::from_value(obj["angular velocity"].clone())?;
        let mut new_obj: Box<dyn traits::Object>;
        if shape == "Polygon" {
            let vertices: Vec<[f64; 2]> = serde_json::from_value(obj["vertices"].clone())?;
            new_obj = Box::new(Polygon::new(vertices, mass));
        } else {
            let radius: f64 = serde_json::from_value(obj["radius"].clone())?;
            new_obj = Box::new(Circle::new(center, radius, mass));
        }
        new_obj.setvel(velocity);
        new_obj.set_static(static_shape);
        new_obj.set_angular_vel(angular_velocity);
        objects.push(new_obj);
    }
    Ok(())
}