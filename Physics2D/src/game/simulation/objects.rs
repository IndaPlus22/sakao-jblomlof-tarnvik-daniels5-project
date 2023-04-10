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
            velocity: Vec2::new(1.0, 1.0),
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
            for ownPoint in self.vertices.iter() {
                for otherPoint in other.getvertices().iter() {
                    //checkCollision(*ownPoint, *otherPoint);
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
}

impl Circle {
    pub fn new(center: Vec2, radius: isize, mass: usize) -> Circle {
        Circle {
            center,
            radius,
            mass,
            velocity: Vec2::new(1.0, 1.0),
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
        return None;
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
    
}

fn checkCollision(first: [[f64;2];2], second: [[f64;2];2]) -> bool {
    let temp = Matrix2::new(
        (first[0][0] - first[1][0]), (second[1][0]-second[0][0]),
        (first[0][1] - first[1][1]), (second[1][1]-second[0][1]),
    );

    let mut b = Vector2::new((first[0][0]-second[0][0]), (first[0][1] - second[0][1]));

    let decomp = temp.lu();

    let x = decomp.solve(&b).expect("HELLO");
    
    println!("{:?}", x);
    return true

}