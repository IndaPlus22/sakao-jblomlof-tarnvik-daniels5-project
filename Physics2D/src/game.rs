use opengl_graphics::OpenGL;
use piston::{
    input::{RenderEvent, UpdateEvent},
    window::WindowSettings,
};
use piston_window::{Event, PistonWindow};

mod update;
mod draw;

pub const SCREEN_WIDTH: u32 = 640;
pub const SCREEN_HEIGHT: u32 = 480;

pub struct Game {

}

impl Game {
    pub fn new() -> Game{
        Game {
            
        }
    }

    pub fn init(&mut self) {

    }

    pub fn update(&mut self) {
        update::update()
    }

    pub fn draw(&mut self, event: &Event, window: &mut PistonWindow) {
        draw::draw(&event, window);
    }
}