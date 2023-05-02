use std::fs::File;
use std::io::Write;

// boilerplate use for the game
use glutin_window::GlutinWindow as Window;
use graphics::types::Vec2d;
use opengl_graphics::{GlGraphics, OpenGL, Texture};
use piston::{Event, RenderArgs, UpdateArgs};

// IMPORTS form our code
use self::ui::{ui_draw, ui_input};
use crate::game::ui::ui_objects::Objects;
use crate::{
    game::simulation::{objects, traits},
    vector::vector::Vec2,
};

// MODULES
mod draw;
mod update;
mod simulation {
    pub mod collision;
    pub mod objects;
    pub mod traits;
}
mod ui {
    pub mod toolbar;
    pub mod ui_button;
    pub mod ui_draw;
    pub mod ui_input;
    pub mod ui_objects;
}

// constants
pub const SCREEN_WIDTH: u32 = 600;
pub const SCREEN_HEIGHT: u32 = 600;
pub const GRAVITY: Vec2d = [0.0, -1.0];

//Game state
#[derive(PartialEq)]
pub enum GameState {
    Running,
    Paused,
}

// Game struct
//TODO game_state probably shouldn't just be public, something smart should happen instead
pub struct Game {
    pub gl: GlGraphics,
    variables: Variables,
    ui_objects: Objects,
}

// Game impl
impl Game {
    // Constructor for the game
    pub fn new(opengl: OpenGL) -> Game {
        let ui_objects: Objects = Objects::new();
        let mut game_state = GameState::Paused;
        Game {
            gl: GlGraphics::new(opengl),
            variables: Variables {
                objects: vec![],
                game_state,
            },
            ui_objects,
        }
    }

    // A function that only runs ones when the game starts
    pub fn init(&mut self) /*-> std::io::Result<()>*/{
        draw::init();
        //let mut file = File::create("test.txt")?;
        //file.write_all(b"plz work")?;
        // TEMPORARY CODE TO TEST OBJECTS
        //self.variables
        //    .objects
        //    .push(Box::new(objects::Circle::new(Vec2::new(100.0, 50.), 20.0, 10.0)));
        self.variables
            .objects
            .push(Box::new(objects::Rectangle::new(
                vec![[0.15, 0.1], [0.15, 0.3], [0.25, 0.2], [0.25, 0.1]],
                10.0,
            )));
            self.variables
            .objects
            .push(Box::new(objects::Rectangle::new(
                vec![[0.4, 0.1], [0.42, 0.15], [0.4, 0.2], [0.45, 0.18], [0.5, 0.2], [0.48, 0.15], [0.5, 0.1], [0.45, 0.12]],
                10.0,
            )));
        // self.variables
        // .objects
        // .push(Box::new(objects::Rectangle::new(
        //     Vec2::new(300., 100.),
        //     vec![[110.0,50.0],[100.0,60.0],[120.0,70.0],[120.0,50.0]],
        //     10.0,
        // )));
        self.variables.objects[0].setvel(Vec2::new(0.0, 0.0));
        self.variables.objects[1].setvel(Vec2::new(-0.0001, 0.0));


        //self.variables.objects[2].setvel(Vec2::new(0.1, 0.0));
        //self.variables.objects[1].set_static(true);
        //Ok(())
    }

    // A function that runs every update
    pub fn update(&mut self, update_args: UpdateArgs) {
        if self.variables.game_state == GameState::Running {
            update::update(update_args, &mut self.variables);
        }
    }

    // A function that runs every frame
    // pub fn draw(&mut self, event: &Event, window: &mut PistonWindow, gl: &mut opengl_graphics::GlGraphics) {
    //     draw::draw(&event, window, &self.variables);
    //     ui_draw::draw(event, window, &mut self.ui_objects);
    // }
    pub fn draw(&mut self, event: &Event, args: &RenderArgs) {
        draw::draw(event, args, &mut self.gl, &self.variables);
        ui_draw::draw(event, args, &mut self.gl, &mut self.ui_objects);
    }

    // A function that runs every time the user does inputs
    pub fn input(&mut self, event: &Event) {
        ui_input::input(event, &mut self.ui_objects, &mut self.variables);
    }
}

// A struct that holds all the variables that the game needs
pub struct Variables {
    objects: Vec<Box<dyn traits::Object>>,
    game_state: GameState,
}
