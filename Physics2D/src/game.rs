use graphics::types::Vec2d;
use opengl_graphics::OpenGL;
use piston::{
    input::{RenderEvent, UpdateEvent},
    window::WindowSettings, UpdateArgs,
};
use piston_window::{Event, PistonWindow};

use self::input::Input;

mod update;
mod draw;
mod input;
mod objects;
mod traits;
mod button;

pub const SCREEN_WIDTH: u32 = 640;
pub const SCREEN_HEIGHT: u32 = 480;
pub const GRAVITY: Vec2d = [0.0,-1.0];

pub struct Game {
    variables: Variables,
    inputs: Input,
}

impl Game {
    pub fn new() -> Game{
        let inputs = input::Input::new();
        Game {
            variables: Variables {
                objects: vec![],
            },
            inputs
        }
    }

    pub fn init(&mut self) {
        draw::init();

        // TEMPORARY CODE TO TEST OBJECTS
        self.variables.objects.push(Box::new(objects::Circle::new([100.0, 100.0], 20, 10)));
        self.variables.objects.push(Box::new(objects::Circle::new([200.0, 100.0], 20, 10)));
        self.variables.objects.push(Box::new(objects::Rectangle::new([300.0, 100.0], 20, 10, 10)));
    }


    pub fn update(&mut self, update_args: UpdateArgs) {
        update::update(update_args, &mut self.variables);
    }

    pub fn draw(&mut self, event: &Event, window: &mut PistonWindow) {
        draw::draw(&event, window, &self.variables);
    }

    pub fn input(&mut self, event: &Event) {
        self.inputs.input(&event);
    }
}

pub struct Variables {
    objects: Vec<Box<dyn traits::Object>>,

}