use opengl_graphics::GlGraphics;
use piston::{Event, RenderArgs};

use super::{ui_objects::Objects};


pub fn draw(_event: &Event, args: &RenderArgs, gl: &mut GlGraphics, objects: &mut Objects) {
    gl.draw(args.viewport(), |context, gl: &mut GlGraphics| {
        for i in 0..objects.play_buttons.len() {
            objects.play_buttons[i].draw(gl, context.transform);
        }
        for i in 0..objects.tool_buttons.len() {
            objects.tool_buttons[i].draw(gl, context.transform);
        }

        objects.draw_selected_poses(gl, context.transform, args);
    });
}