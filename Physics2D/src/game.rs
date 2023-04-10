// bolierplate use for the game
use graphics::types::Vec2d;
use opengl_graphics::OpenGL;
use piston::{
    input::{RenderEvent, UpdateEvent},
    window::WindowSettings,
    UpdateArgs,
};
use piston_window::{Event, PistonWindow};

// IMPORTS form our code
use self::input::Input;
use crate::{game::simulation::{objects, traits}, vector::vector::Vec2};

// MODULES
mod draw;
mod input;
mod update;
mod simulation {
    pub mod objects;
    pub mod traits;
}
mod ui {
    pub mod ui_button;
    pub mod ui_draw;
    pub mod ui_input;
}

// constants
pub const SCREEN_WIDTH: u32 = 640;
pub const SCREEN_HEIGHT: u32 = 480;
pub const GRAVITY: Vec2d = [0.0, -1.0];

// Game struct
pub struct Game {
    variables: Variables,
    inputs: Input,
}

// Game impl
impl Game {
    // Constructor for the game
    pub fn new() -> Game {
        let inputs = input::Input::new();
        Game {
            variables: Variables { objects: vec![] },
            inputs,
        }
    }

    // A function that only runs ones when the game starts
    pub fn init(&mut self) {
        draw::init();

        // TEMPORARY CODE TO TEST OBJECTS
        self.variables
            .objects
            .push(Box::new(objects::Circle::new(Vec2::new(100., 100.0), 20, 10)));
        self.variables
            .objects
            .push(Box::new(objects::Circle::new(Vec2::new(200.0, 100.), 20, 10)));
        self.variables
            .objects
            .push(Box::new(objects::Rectangle::new(
                Vec2::new(300., 100.),
                vec![[50.0,50.0],[50.0,70.0],[70.0,70.0],[70.0,50.0]],
                10,
            )));
        self.variables
            .objects
            .push(Box::new(objects::Rectangle::new(
                Vec2::new(300., 100.),
                vec![[100.0,100.0],[100.0,120.0],[120.0,120.0],[120.0,100.0]],
                10,
            )));

        self.variables.objects[3].setvel(Vec2::new(0.0, 0.0));
    }

    // A function that runs every update
    pub fn update(&mut self, update_args: UpdateArgs) {
        update::update(update_args, &mut self.variables);
    }

    // A function that runs every frame
    pub fn draw(&mut self, event: &Event, window: &mut PistonWindow) {
        draw::draw(&event, window, &self.variables);
    }

    // A function that runs every time the user does inputs
    pub fn input(&mut self, event: &Event) {
        self.inputs.input(&event);
    }
}

// A struct that holds all the variables that the game needs
pub struct Variables {
    objects: Vec<Box<dyn traits::Object>>,
}
