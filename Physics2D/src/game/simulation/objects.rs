use gfx_device_gl::{Resources, CommandBuffer};
use gfx_graphics::GfxGraphics;
use graphics::types::Vec2d;
use piston_window::types::Matrix2d;

use super::{traits::{collisionRecord, Object}};
use crate::game::draw::{draw_rect, draw_circle};

pub struct Rectangle {
    center: Vec2d,
    height: isize,
    width: isize,
    mass: usize,
    velocity: f64,
    potnrg: f64,
}

pub struct Circle {
    center: Vec2d,
    radius: isize,
    mass: usize,
    velocity: f64,
    potnrg: f64,
}

impl Rectangle {
    pub fn new(center: Vec2d, height: isize, width: isize, mass: usize) -> Rectangle {
        Rectangle {
            center,
            height,
            width,
            mass,
            velocity: 0.0,
            potnrg: 0.0,
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
    fn update(&self) {

    }
    fn draw(&self, graphics: &mut GfxGraphics<Resources, CommandBuffer>, transform: Matrix2d) {
        draw_rect(self.center, [(self.width) as f64, self.height as f64], transform, graphics)
    }
    fn getcenter(&self) -> Vec2d {
        return self.center;
    }
}

impl Circle {
    pub fn new(center: Vec2d, radius: isize, mass: usize) -> Circle {
        Circle {
            center,
            radius,
            mass,
            velocity: 0.0,
            potnrg: 0.0,
        }
    }
}

impl Object for Circle {
    fn collisions(
        &self,
        other: &Box<dyn Object>,
        record: Option<collisionRecord>,
    ) -> Option<collisionRecord> {
        return record;
    }
    fn update(&self) {}
    fn draw(&self, graphics: &mut GfxGraphics<Resources, CommandBuffer>, transform: Matrix2d) {
        draw_circle(self.center, self.radius as f64, transform, graphics);
    }
    fn getcenter(&self) -> Vec2d {
        return self.center;
    }
}
