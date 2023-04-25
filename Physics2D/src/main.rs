// extern crate piston_window;
// extern crate image;

use game::Game;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::{input::UpdateEvent, window::WindowSettings, Event, EventSettings, Events, Loop};

use crate::game::GameState;
mod game;
mod vector;

fn main() {
    use std::{thread, time};
    let opengl = OpenGL::V3_2;

    let mut window: Window =
        WindowSettings::new("PHYSICS", (game::SCREEN_WIDTH, game::SCREEN_HEIGHT))
            .exit_on_esc(true)
            .graphics_api(opengl)
            .build()
            .unwrap();

    let mut game = game::Game::new(opengl);

    game.init();

    let mut events = Events::new(EventSettings::new());
    // game loop
    while let Some(event) = events.next(&mut window) {
        match event {
            Event::Input(_, _) => {
                game.input(&event);
            }
            Event::Loop(Loop::Render(args)) => {
                game.draw(&event, &args);
            }
            Event::Loop(Loop::Update(_)) => {
                game.update(event.update_args().unwrap());
            }
            _ => {}
        }
    }
}
