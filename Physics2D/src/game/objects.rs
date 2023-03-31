use graphics::types::Vec2d;

use super::traits::{Object, collisionRecord};

pub struct Rectangle{
    center: Vec2d,
    height: isize,
    width: isize,
    mass: usize,
    velocity: f64,
    potnrg: f64,
}

impl Object for Rectangle {
    fn collisions(&self, other: &Box<dyn Object>, record: Option<collisionRecord>) -> Option<super::traits::collisionRecord> {
        
        return record;
    }
    fn update(&self) {
        
    }
    fn draw(&self) {
        
    }
}