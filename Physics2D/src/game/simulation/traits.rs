use gfx_device_gl::{Resources, CommandBuffer};
use graphics::types::{Vec2d};
use piston_window::math::Matrix2d;
use gfx_graphics::GfxGraphics;

use crate::vector::vector::Vec2;

pub trait Object {
    fn collisions(&self, other: &Box<dyn Object>, record: Option<collisionRecord>) -> Option<collisionRecord>;
    fn update(&mut self, record: &Option<collisionRecord>, dt: f64);
    fn draw(&self, graphics: &mut GfxGraphics<Resources, CommandBuffer>, transform: Matrix2d);
    fn getcenter(&self) -> Vec2;
    fn gettype(&self) -> String;
    fn get_circle_center(&self) -> Vec2;
    fn getradius(&self) -> f64;
    fn getvertices(&self) -> Vec<[f64;2]>;
    fn getvel(&self) -> Vec2;
    fn setvel (&mut self, vel: Vec2);
    fn moverelative (&mut self, pos: Vec2);
    fn set_static (&mut self, set: bool);
    fn get_mass (&self) -> f64;
}

pub struct collisionRecord {
    pub desired_movement: Vec2,
    pub impulse: Vec2,
}