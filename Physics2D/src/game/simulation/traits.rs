use gfx_device_gl::{Resources, CommandBuffer};
use graphics::types::{Vec2d};
use piston_window::math::Matrix2d;
use gfx_graphics::GfxGraphics;

use crate::vector::vector::Vec2;

pub trait Object {
    fn collisions(&self, other: &Box<dyn Object>, record: Option<collisionRecord>) -> Option<collisionRecord>;
    fn update(&mut self);
    fn draw(&self, graphics: &mut GfxGraphics<Resources, CommandBuffer>, transform: Matrix2d);
    fn getcenter(&self) -> Vec2;
    fn gettype(&self) -> String;
    fn getradius(&self) -> isize;
    fn getvertices(&self) -> Vec<[f64;2]>;
    fn setvel (&mut self, vel: Vec2);
}

pub struct collisionRecord {
    
}