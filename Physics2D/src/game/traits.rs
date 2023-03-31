pub trait Object {
    fn collisions(&self, other: &Box<dyn Object>, record: Option<collisionRecord>) -> Option<collisionRecord>;
    fn update(&self);
    fn draw(&self);
}

pub struct collisionRecord {
    
}