use graphics::types::Vec2d;

use super::traits::Object;

pub struct Rectangle{
    center: Vec2d,
    height: isize,
    width: isize,
    mass: usize,
    velocity: f64,
    potnrg: f64,
}

impl Object for Rectangle {
    fn update(&self) {
        
    }
    fn draw(&self) {
        
    }
}