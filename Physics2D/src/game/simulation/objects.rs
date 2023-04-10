use gfx_device_gl::{Resources, CommandBuffer};
use gfx_graphics::GfxGraphics;
use graphics::types::Vec2d;
use piston_window::types::Matrix2d;

use super::{traits::{collisionRecord, Object}};
use crate::{game::draw::{draw_rect, draw_circle}, vector::vector::Vec2};

//Import the vector.rs so that i can use Vec2



pub struct Rectangle {
    center: Vec2,
    height: isize,
    width: isize,
    mass: usize,
    velocity: Vec2,
    potnrg: f64,
    form: String,
}

pub struct Circle {
    center: Vec2,
    radius: isize,
    mass: usize,
    velocity: Vec2,
    potnrg: f64,
    form: String,
}

impl Rectangle {
    pub fn new(center: Vec2, height: isize, width : isize, mass: usize) -> Rectangle {
        Rectangle {
            center,
            height,
            width,
            mass,
            velocity: Vec2::new(1.0, 1.0),
            potnrg: 0.0,
            form: "Rectangle".to_string(),
        }
    }
}

impl Object for Rectangle {
    fn collisions(
        &self,
        other: &Box<dyn Object>,
        record: Option<collisionRecord>,
    ) -> Option<super::traits::collisionRecord> {
        //Vi kommer behöva göra en if statement eller en case switch som kollar följande
        //Om other är en circel
        //Om other är en rectanglel
        //Osv
        //Då jag inte lyckas hitta någon generell formel för alla möjliga former.
        return record;
    }
    fn update(&mut self) {
        self.center += self.velocity
    }
    fn draw(&self, graphics: &mut GfxGraphics<Resources, CommandBuffer>, transform: Matrix2d) {
        draw_rect(self.center, [(self.width) as f64, self.height as f64], transform, graphics)
    }
    fn getcenter(&self) -> Vec2 {
        return self.center;
    }
    fn gettype(&self) -> String {
        return self.form.clone();
    }
    fn getradius(&self) -> isize {
        return self.width;
    }
}

impl Circle {
    pub fn new(center: Vec2, radius: isize, mass: usize) -> Circle {
        Circle {
            center,
            radius,
            mass,
            velocity: Vec2::new(1.0, 1.0),
            potnrg: 0.0,
            form: "Circle".to_string(),
        }
    }
}

impl Object for Circle {
    fn collisions(
        &self,
        other: &Box<dyn Object>,
        record: Option<collisionRecord>,
    ) -> Option<collisionRecord> {
        if other.gettype() == "Circle" {
            let othercenter = other.getcenter();
            let distance = (self.center - othercenter).length();
            if distance < (self.radius + other.getradius()) as f64 {
                println!("Collision");
                return Some(collisionRecord {});
            }
        }
        return None;
    }
    fn update(&mut self) {}
    fn draw(&self, graphics: &mut GfxGraphics<Resources, CommandBuffer>, transform: Matrix2d) {
        draw_circle(self.center, self.radius as f64, transform, graphics);
    }
    fn getcenter(&self) -> Vec2 {
        return self.center;
    }
    fn gettype(&self) -> String {
        return self.form.clone();
    }
    fn getradius(&self) -> isize {
        return self.radius;
    }
}
