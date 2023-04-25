use opengl_graphics::GlGraphics;
use piston::{Event, RenderArgs};

use super::ui_objects::Objects;

pub fn draw(event: &Event, args: &RenderArgs, gl: &mut GlGraphics, objects: &mut Objects) {
    gl.draw(args.viewport(), |context, gl| {
        for i in 0..objects.buttons.len() {
            objects.buttons[i].draw(gl, context.transform);

            // drawing all the buttons in the toolbar
            objects.tool_bar.draw(gl, context.transform);
        }
    });
}