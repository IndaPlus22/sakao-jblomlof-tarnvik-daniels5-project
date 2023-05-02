
use gfx_device_gl::{CommandBuffer, Resources};
use gfx_graphics::GfxGraphics;
use graphics::{color, types::{Vec2d, Matrix2d}, clear, rectangle, Polygon, draw_state::DrawState, Ellipse};
use opengl_graphics::GlGraphics;
use piston::{Event, RenderArgs};
use glutin_window::GlutinWindow as Window;

use crate::vector::vector::Vec2;

//use super::button::Button;
use super::Variables;

const CERISE_COLOR: [f32; 4] = [232.0 / 255.0, 61.0 / 255.0, 132.0 / 255.0, 1.0];
const LIGHT_CERISE: &str = "ec5f99";

// pub fn draw(event: &Event, window: &mut PistonWindow, variables: &Variables) {
//     // Update application window.
//     window.draw_2d(event, |context, graphics, _| {
//         // Fill the window with white colour.
//         piston_window::clear(CERISE_COLOR, graphics);

//         // TODO: For loop all objects in simulation and render them (I think that it needs to be assigned to a variable)
//         for item in &variables.objects {
//             item.draw(graphics, context.transform);
//         }
//     });
// }

pub fn draw(event: &Event, args: &RenderArgs, gl: &mut GlGraphics, variables: &mut Variables) {
    // Update application window.
    gl.draw(args.viewport(), |context, gl| {
        variables.win_size = args.window_size;
        // Fill the window with white colour.
        clear(CERISE_COLOR, gl);

        // TODO: For loop all objects in simulation and render them (I think that it needs to be assigned to a variable)
        for item in &variables.objects {
            item.draw(gl, context.transform, args);
        }
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
    gl: &mut GlGraphics,
) {
    // Polygon::new(rgb_to_color(131, 176, 247))

    Polygon::new(rgb_to_color(131, 176, 247)).draw(
        &conv_pos_size_to_vertices_rect(pos, size),
        &DrawState::default(),
        transform,
        gl,
    );
}

// Draws a polygon by polygon with vertices as corners.
pub fn draw_polygon(
    vertices: &[[f64; 2]],
    transform: Matrix2d,
    gl: &mut GlGraphics,
    args: &RenderArgs
) {
    let abs_vertices = rel_to_abs_pos_arr(&vertices, args.window_size);
    // println!("Drawing polygon at: {:?}", abs_vertices);
    Polygon::new(rgb_to_color(131, 176, 247)).draw(
        &abs_vertices,
        &DrawState::default(),
        transform,
        gl,
    );
}

// Draws a circle by ellipse.
pub fn draw_circle(
    pos: Vec2,
    radius: f64,
    transform: Matrix2d,
    gl: &mut GlGraphics,
    args: &RenderArgs
) {
    // For debugging
    // println!("Drawing circle at: ({}, {})", pos[0], pos[1]);
    // --------------
    let abs_pos = rel_to_abs_pos([pos.x, pos.y], args.window_size);
    // println!("Drawing circle at: ({}, {})", abs_pos[0], abs_pos[1]);
    // radius is relative to window x length
    let abs_radius = radius * args.window_size[0];
    let circle = graphics::ellipse::circle(abs_pos[0], abs_pos[1], abs_radius);
    Ellipse::new(color::hex("202d42")).draw(circle, &DrawState::default(), transform, gl);
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

// converts a relative position array (0. - 1.) to an absolute position array
fn rel_to_abs_pos_arr(rel_poses: &[Vec2d], win_size: Vec2d) -> Vec<Vec2d> {
    let mut abs_poses: Vec<Vec2d> = Vec::new();
    for i in 0..rel_poses.len() {
        abs_poses.push([rel_poses[i][0] * win_size[0], rel_poses[i][1] * win_size[1]]);
    }
    abs_poses
}

fn rel_to_abs_pos(rel_pos: Vec2d, win_size: Vec2d) -> Vec2d {
    [rel_pos[0] * win_size[0], rel_pos[1] * win_size[1]]
}

pub fn abs_to_rel_pos(abs_pos: Vec2d, win_size: Vec2d) -> Vec2d {
    [abs_pos[0] / win_size[0], abs_pos[1] / win_size[1]]
}

// Converts rgb to color. Helper because I(Toshi) likes to copy from google color picker.
fn rgb_to_color(r: u16, g: u16, b: u16) -> [f32; 4] {
    [r as f32 / 255.0, g as f32  / 255.0, b as f32  / 255.0, 1.0]
}
