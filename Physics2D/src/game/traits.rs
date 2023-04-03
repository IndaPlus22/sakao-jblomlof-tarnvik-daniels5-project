use graphics::types::Vec2d;

pub trait Object {
    fn collisions(&self, other: &Box<dyn Object>, record: Option<collisionRecord>) -> Option<collisionRecord>;
    fn update(&self);
    fn draw(&self);
    fn getcenter(&self) -> Vec2d;
}

pub struct collisionRecord {
    
}