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
    mass: usize,
    velocity: Vec2,
    potnrg: f64,
    form: String,
}

pub struct Circle {
    center: Vec2,
    radius: isize,
    mass: usize,
    velocity: Vec2,
    potnrg: f64,
    form: String,
}

impl Rectangle {
    pub fn new(center: Vec2, vertices: Vec<[f64;2]>, mass: usize) -> Rectangle {
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
            for line in lines.iter() {
                for otherline in otherlines.iter() {
                    if checkCollision(*line, *otherline) {
                        //println!("Collision");
                        return Some(collisionRecord {});
                    }
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
    fn getradius(&self) -> isize {
        return 10;
    }
    fn getvertices(&self) -> Vec<[f64;2]> {
        return self.vertices.to_vec()
    }
    fn setvel(&mut self, vel: Vec2) {
        self.velocity = vel;
    }
}

impl Circle {
    pub fn new(center: Vec2, radius: isize, mass: usize) -> Circle {
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
    fn getradius(&self) -> isize {
        return self.radius;
    }
    fn getvertices(&self) -> Vec<[f64;2]> {
        return vec![]
    }
    fn setvel (&mut self, vel: Vec2) {
        self.velocity = vel;
    }
    
}

//Use linear algebra to check if two lines intersect each other
fn checkCollision(line1: [[f64;2];2], line2: [[f64;2];2]) -> bool {
    let mut matrix = Matrix2::new(line1[0][0] - line1[1][0], line2[0][0] - line2[1][0], line1[0][1] - line1[1][1], line2[0][1] - line2[1][1]);
    let mut vector = Vector2::new(line2[0][0] - line1[0][0], line2[0][1] - line1[0][1]);
    let decomp = matrix.lu();
    let mut result = decomp.solve(&vector);
    if result.is_some() {
        let result = result.unwrap();
        if result[0] >= 0.0 && result[0] <= 1.0 && result[1] >= 0.0 && result[1] <= 1.0 {
            return true;
        }
    }
    return false;
}
