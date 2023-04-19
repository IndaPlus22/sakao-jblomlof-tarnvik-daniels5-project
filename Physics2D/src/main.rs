extern crate piston_window;

use opengl_graphics::OpenGL;
use piston::{
    input::UpdateEvent,
    window::WindowSettings, Loop,
};
use piston_window::{Event, PistonWindow};
mod game;
mod vector;


fn main() {
    use std::{thread, time};

    let mut window: PistonWindow =
        WindowSettings::new("PHYSICS", (game::SCREEN_WIDTH, game::SCREEN_HEIGHT))
            .exit_on_esc(true)
            .graphics_api(OpenGL::V3_2)
            .build()
            .unwrap();

    let mut game = game::Game::new();

    game.init();

    // game loop
    while let Some(event) = window.next() {
        match event {
            Event::Input(_, _) => {
                game.input(&event);
            }
            Event::Loop(Loop::Render(_)) => {
                game.draw(&event, &mut window);
            }
            Event::Loop(Loop::Update(_)) => {
                game.update(event.update_args().unwrap());
            }
            _ => {}
        }

        // TODO: Delete this code (this is the old code. If match doesnt work check this bit of code)
        // if let Some(_) = event.render_args() {
        //     game.draw(&event, &mut window);
        // }
        // if let Some(update_args) = event.update_args() {
        //     game.update(update_args);
        // }
        // // input handling
        // if let Some(_) = event.mouse_cursor_args() {
        //     game.input(&event);
        // }
        // if let Some(_) = event.button_args() {
        //     game.input(&event);
        // }
        // -------------------------------------------------------------------------------
    }
}
