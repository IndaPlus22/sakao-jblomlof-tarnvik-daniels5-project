// boilerplate use for the game
use graphics::types::Vec2d;
use piston::{
    UpdateArgs,
};
use piston_window::{Event, PistonWindow};

// IMPORTS form our code
use self::ui::{ui_draw, ui_input};
use crate::game::ui::ui_objects::Objects;
use crate::{game::simulation::{objects, traits}, vector::vector::Vec2};

// MODULES
mod draw;
mod update;
mod simulation {
    pub mod objects;
    pub mod traits;
}
mod ui {
    pub mod ui_button;
    pub mod ui_draw;
    pub mod ui_input;
    pub mod ui_objects;
}

// constants
pub const SCREEN_WIDTH: u32 = 640;
pub const SCREEN_HEIGHT: u32 = 480;
pub const GRAVITY: Vec2d = [0.0, -1.0];

// Game struct
pub struct Game {
    variables: Variables,
    objects: Objects,
}

// Game impl
impl Game {
    // Constructor for the game
    pub fn new() -> Game {
        let objects: Objects = Objects::new();
        Game {
            variables: Variables { objects: vec![] },
            objects
        }
    }

    // A function that only runs ones when the game starts
    pub fn init(&mut self) {
        draw::init();

        // TEMPORARY CODE TO TEST OBJECTS
        self.variables
            .objects
            .push(Box::new(objects::Circle::new(Vec2::new(100.0, 50.), 20.0, 10.0)));
        self.variables
            .objects
            .push(Box::new(objects::Rectangle::new(
                Vec2::new(300., 100.),
                vec![[50.0,50.0],[50.0,70.0],[70.0,70.0],[70.0,50.0]],
                10.0,
            )));
        self.variables
            .objects
            .push(Box::new(objects::Rectangle::new(
                Vec2::new(300., 100.),
                vec![[150.0,50.0],[150.0,70.0],[170.0,70.0],[170.0,50.0]],
                10.0,
            )));
        self.variables.objects[0].setvel(Vec2::new(-0.5, 0.));
        self.variables.objects[1].setvel(Vec2::new(0.0, 0.0));
        self.variables.objects[2].setvel(Vec2::new(-0.5, 0.0));
        self.variables.objects[1].set_static(true);

    }

    // A function that runs every update
    pub fn update(&mut self, update_args: UpdateArgs) {
        update::update(update_args, &mut self.variables);
    }

    // A function that runs every frame
    pub fn draw(&mut self, event: &Event, window: &mut PistonWindow) {
        draw::draw(&event, window, &self.variables);
        ui_draw::draw(event, window, &mut self.objects);
    }

    // A function that runs every time the user does inputs
    pub fn input(&mut self, event: &Event) {
        ui_input::input(event, &mut self.objects);
    }
}

// A struct that holds all the variables that the game needs
pub struct Variables {
    objects: Vec<Box<dyn traits::Object>>,
}
