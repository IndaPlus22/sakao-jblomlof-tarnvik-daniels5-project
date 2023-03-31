pub trait Object {
    fn collisions(&self, other: &dyn Object) -> Option<collisionRecord>;
    fn update(&self);
    fn draw(&self);
}

pub struct collisionRecord {
    
}