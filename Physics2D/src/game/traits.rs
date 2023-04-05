use gfx_device_gl::{Resources, CommandBuffer};
use graphics::types::{Vec2d};
use piston_window::math::Matrix2d;
use gfx_graphics::GfxGraphics;

pub trait Object {
    fn collisions(&self, other: &Box<dyn Object>, record: Option<collisionRecord>) -> Option<collisionRecord>;
    fn update(&self);
    fn draw(&self, graphics: &mut GfxGraphics<Resources, CommandBuffer>, transform: Matrix2d);
    fn getcenter(&self) -> Vec2d;
}

pub struct collisionRecord {
    
}