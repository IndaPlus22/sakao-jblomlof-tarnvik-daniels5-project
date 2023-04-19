
use gfx_device_gl::{CommandBuffer, Resources};
use gfx_graphics::GfxGraphics;
use graphics::{color, types::Vec2d};
use piston_window::{*, types::{Matrix2d}};

use crate::vector::vector::Vec2;

//use super::button::Button;
use super::Variables;

const CERISE_COLOR: [f32; 4] = [232.0 / 255.0, 61.0 / 255.0, 132.0 / 255.0, 1.0];
const LIGHT_CERISE: &str = "ec5f99";

pub fn draw(event: &Event, window: &mut PistonWindow, variables: &Variables) {
    // Update application window.
    window.draw_2d(event, |context, graphics, _| {
        // Fill the window with white colour.
        piston_window::clear(CERISE_COLOR, graphics);

        // TODO: put all graphics shit here
        /*let buttons = play_bar(Vec2d::from([10.0, 10.0]));
        for i in 0..buttons.len() {
            buttons[i].draw(graphics, context.transform);
        }*/

        // TODO: For loop all objects in simulation and render them (I think that it needs to be assigned to a variable)
        for item in &variables.objects {
            item.draw(graphics, context.transform);
        }

        // DEBUG funcs ------------------------------------------------
        // draw_rect([100.0, 100.0], [50.0, 50.0], context.transform, graphics);
        // draw_circle([200.0, 200.0], 70.0, context.transform, graphics);
        // draw_polygon(&[[150.0, 190.0], [220.0, 320.0], [380.0, 400.0], [410.0, 280.0], [300.0, 150.0]], context.transform, graphics);
        // ---------------------------------------------------------
    });
}

// TODO: should be something that is in initialization of game and probably in something alike "fn init_menu"
pub fn init() {
    let buttons = play_bar(Vec2d::from([0.0, 0.0]));
}

// TODO:
fn play_bar(pos: Vec2d) /*-> [Button; 2]*/ {
    // pos is upper-left corner
    let size: Vec2d = Vec2d::from([40.0, 40.0]);
    let width = 40.0;
    let height = 40.0;
    let play_pos: Vec2d = Vec2d::from([pos[0] + 40.0, pos[1]]);
    let restart_pos: Vec2d = Vec2d::from([pos[0] + size[0] + 60.0, pos[1]]);

    //let mut play_button = Button::new(play_pos, width, height, [0.0,0.0,0.0,1.0]);
    //let mut restart_button = Button::new(restart_pos, width, height, [0.0,0.0,0.0,1.0]);

    //[play_button, restart_button]
}

// TODO:
fn tool_box() {}

// Draws a rectangle by polygon. size[0] is width and size[1] is height.
pub fn draw_rect(
    pos: Vec2,
    size: [f64; 2],
    transform: Matrix2d,
    g: &mut GfxGraphics<Resources, CommandBuffer>,
) {
    Polygon::new(rgb_to_color(131, 176, 247)).draw(
        &conv_pos_size_to_vertices_rect(pos, size),
        &piston_window::DrawState::default(),
        transform,
        g,
    );
}

// Draws a polygon by polygon with vertices as corners.
pub fn draw_polygon(
    vertices: &[[f64; 2]],
    transform: Matrix2d,
    g: &mut GfxGraphics<Resources, CommandBuffer>,
    pos: Vec2
) {
    Polygon::new(rgb_to_color(131, 176, 247)).draw(
        vertices,
        &piston_window::DrawState::default(),
        transform,
        g,
    );
}

// Draws a circle by ellipse.
pub fn draw_circle(
    pos: Vec2,
    radius: f64,
    transform: Matrix2d,
    g: &mut GfxGraphics<Resources, CommandBuffer>,
) {
    // For debugging
    // println!("Drawing circle at: ({}, {})", pos[0], pos[1]);
    // --------------

    let circle = graphics::ellipse::circle(pos.x, pos.y, radius);
    Ellipse::new(color::hex(LIGHT_CERISE)).draw(circle, &piston_window::DrawState::default(), transform, g);
}

// Converts two vec2d one for position and one for size to a 4x2 array of corners. ONLY works for rectangles.
fn conv_pos_size_to_vertices_rect(pos: Vec2, size: [f64; 2]) -> [[f64; 2]; 4]{
    let corners: [[f64; 2]; 4] = [
        [pos.x - size[0]/2.0, pos.y - size[1]/2.0],
        [pos.x - size[0]/2.0, pos.y + size[1]/2.0],
        [pos.x + size[0]/2.0, pos.y + size[1]/2.0],
        [pos.x + size[0]/2.0, pos.y - size[1]/2.0]
    ];
    corners
}

// Converts rgb to color. Helper because I(Toshi) likes to copy from google color picker.
fn rgb_to_color(r: u16, g: u16, b: u16) -> [f32; 4] {
    [r as f32 / 255.0, g as f32  / 255.0, b as f32  / 255.0, 1.0]
}
