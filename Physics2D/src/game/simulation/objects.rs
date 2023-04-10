use gfx_device_gl::{Resources, CommandBuffer};
use gfx_graphics::GfxGraphics;
use graphics::types::Vec2d;
use nalgebra::Vector2;
use nalgebra::Matrix2;
use piston_window::types::Matrix2d;

use super::{traits::{collisionRecord, Object}};
use crate::{game::draw::{draw_rect, draw_circle, draw_polygon}, vector::vector::Vec2};


pub struct Point {
    x: f64,
    y: f64
}




pub struct Rectangle {
    center: Vec2,
    vertices: Vec<[f64;2]>,
    mass: f64,
    velocity: Vec2,
    potnrg: f64,
    form: String,
}

pub struct Circle {
    center: Vec2,
    radius: f64,
    mass: f64,
    velocity: Vec2,
    potnrg: f64,
    form: String,
}

impl Rectangle {
    pub fn new(center: Vec2, vertices: Vec<[f64;2]>, mass: f64) -> Rectangle {
        Rectangle {
            center,
            vertices,
            mass,
            velocity: Vec2::new(0.2, 0.2),
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
        if other.gettype() == "Rectangle" {
            //Create lines of the vertices for both objects
            let mut lines = Vec::new();
            let mut otherlines = Vec::new();
            for i in 0..self.vertices.len() {
                if i == self.vertices.len()-1 {
                    lines.push([self.vertices[i], self.vertices[0]]);
                } else {
                    lines.push([self.vertices[i], self.vertices[i+1]]);
                }
            }
            for i in 0..other.getvertices().len() {
                if i == other.getvertices().len()-1 {
                    otherlines.push([other.getvertices()[i], other.getvertices()[0]]);
                } else {
                    otherlines.push([other.getvertices()[i], other.getvertices()[i+1]]);
                }
            }
            //Here we check for collisions
            for line in lines.iter() {
                for otherline in otherlines.iter() {
                    if checkCollision(*line, *otherline) {
                        //println!("Collision");
                        return Some(collisionRecord {});
                    }
                }
            }
        } else if other.gettype() == "Circle" {
            let mut lines = Vec::new();
            for i in 0..self.vertices.len() {
                if i == self.vertices.len()-1 {
                    lines.push([self.vertices[i], self.vertices[0]]);
                } else {
                    lines.push([self.vertices[i], self.vertices[i+1]]);
                }
            }
            for line in lines.iter() {
                if checkCircleCollisionWithPolygon(other.getcenter(), other.getradius(), *line) {
                    println!("Collision");
                    return Some(collisionRecord {});
                }
            }
            
        }
        return record;
    }
    fn update(&mut self) {
        //self.center += self.velocity;
        
        for point in self.vertices.iter_mut() {
            point[0]+= self.velocity.x;
            point[1]+= self.velocity.y;
        }
    }
    fn draw(&self, graphics: &mut GfxGraphics<Resources, CommandBuffer>, transform: Matrix2d) {
        //draw_rect(self.center, [(self.width) as f64, self.height as f64], transform, graphics)
        draw_polygon(self.vertices.as_slice(), transform,  graphics, self.getcenter())
    }
    fn getcenter(&self) -> Vec2 {
        return self.center;
    }
    fn gettype(&self) -> String {
        return self.form.clone();
    }
    fn getradius(&self) -> f64 {
        return 10.0;
    }
    fn getvertices(&self) -> Vec<[f64;2]> {
        return self.vertices.to_vec()
    }
    fn setvel(&mut self, vel: Vec2) {
        self.velocity = vel;
    }
}

impl Circle {
    pub fn new(center: Vec2, radius: f64, mass: f64) -> Circle {
        Circle {
            center,
            radius,
            mass,
            velocity: Vec2::new(0.2, 0.2),
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
        } else if other.gettype() == "Rectangle" {
            let mut lines = Vec::new();
            for i in 0..other.getvertices().len() {
                if i == other.getvertices().len()-1 {
                    lines.push([other.getvertices()[i], other.getvertices()[0]]);
                } else {
                    lines.push([other.getvertices()[i], other.getvertices()[i+1]]);
                }
            }
            for line in lines.iter() {
                if checkCircleCollisionWithPolygon(self.center, self.radius, *line) {
                    println!("Collision");
                    return Some(collisionRecord {});
                }
            }
        }
        return record;
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
    fn getradius(&self) -> f64 {
        return self.radius;
    }
    fn getvertices(&self) -> Vec<[f64;2]> {
        return vec![]
    }
    fn setvel (&mut self, vel: Vec2) {
        self.velocity = vel;
    }
    
}

//check if two lines intersect each other
fn checkCollision(line1: [[f64;2];2], line2: [[f64;2];2]) -> bool {
    let matrix = Matrix2::new(line1[0][0] - line1[1][0], line2[0][0] - line2[1][0], line1[0][1] - line1[1][1], line2[0][1] - line2[1][1]);
    let vector = Vector2::new(line2[0][0] - line1[0][0], line2[0][1] - line1[0][1]);
    let decomp = matrix.lu();
    let result = decomp.solve(&vector);
    match result {
        Some(solution) => {
            return solution[0] >= 0.0 && solution[0] <= 1.0 && solution[1] >= 0.0 && solution[1] <= 1.0;
        }
        None => {return false}
    }
}

fn checkCircleCollisionWithPolygon(pos: Vec2, radius: f64, vertices: [[f64;2];2]) -> bool{
    let v = Vector2::new(vertices[1][0] - vertices[0][0], vertices[1][1] - vertices[0][1]);
    let k = Vector2::new(pos.x - vertices[0][0], pos.y - vertices[0][1]);
    
    let negative_p_half = v.dot(&k)/v.norm_squared();
    let sqroot = ((negative_p_half*negative_p_half) - (k.norm_squared() - radius*radius)/v.norm_squared()).sqrt();

    //Does not have a solution
    if sqroot.is_nan() {
        return false;
    }

    let t = negative_p_half - sqroot;

    if t>=0. && t<=1. {
        return true
    }


    return false

}
