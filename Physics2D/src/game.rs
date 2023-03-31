use graphics::types::Vec2d;
use opengl_graphics::OpenGL;
use piston::{
    input::{RenderEvent, UpdateEvent},
    window::WindowSettings, UpdateArgs,
};
use piston_window::{Event, PistonWindow};

mod update;
mod draw;
mod objects;
mod traits;

pub const SCREEN_WIDTH: u32 = 640;
pub const SCREEN_HEIGHT: u32 = 480;
pub const GRAVITY: Vec2d = [0.0,-1.0];

pub struct Game {
    variables: Variables,
}

impl Game {
    pub fn new() -> Game{
        Game {
            variables: Variables {
                objects: vec![],
            }
        }
    }

    pub fn init(&mut self) {

    }

    pub fn update(&mut self, update_args: UpdateArgs) {
        update::update(update_args, &mut self.variables);
    }

    pub fn draw(&mut self, event: &Event, window: &mut PistonWindow) {
        draw::draw(&event, window);
    }
}

pub struct Variables {
    objects: Vec<Box<dyn traits::Object>>,

}