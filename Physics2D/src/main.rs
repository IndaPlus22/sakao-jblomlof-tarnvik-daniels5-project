extern crate piston_window;

use opengl_graphics::OpenGL;
use piston::{
    input::{RenderEvent, UpdateEvent},
    window::WindowSettings,
};
use piston_window::{Event, PistonWindow};

mod game;

fn main() {
    use std::{thread, time};

    let mut window =
        WindowSettings::new("Chapter 2 Forces", (SCREEN_WIDTH, SCREEN_HEIGHT))
            .exit_on_esc(true)
            // .graphics_api(OpenGL::V3_2)
            .build()
            .unwrap();

    let mut game = game::Game::new();

    game.init();

    // game loop
    while let Some(event) = window.next() {
        if let Some(_) = event.render_args() {
            game.draw(&event, &mut window);
        }
        if let Some(_) = event.update_args() {
            game.update();
        }

        let duration = time::Duration::from_millis(10);
        thread::sleep(duration);
    }
}
