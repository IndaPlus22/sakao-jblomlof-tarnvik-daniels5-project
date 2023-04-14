use gfx_device_gl::{Resources, CommandBuffer};
use gfx_graphics::GfxGraphics;
use graphics::types::Vec2d;
use nalgebra::Vector2;
use nalgebra::Matrix2;
use piston_window::types::Matrix2d;

use super::{traits::{collisionRecord, Object}};
use crate::{game::draw::{draw_rect, draw_circle, draw_polygon}, vector::vector::Vec2};







pub struct Rectangle {
    center: Vec2,
    vertices: Vec<[f64;2]>,
    mass: f64,
    velocity: Vec2,
    potnrg: f64,
    form: String,
    staticshape: bool,
}

pub struct Circle {
    center: Vec2,
    radius: f64,
    mass: f64,
    velocity: Vec2,
    potnrg: f64,
    form: String,
    staticshape: bool,
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
            staticshape: false
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
                    //This is to add a line between the start vertice and the end vertice
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
            //Here we check for collisions by comparing all the lines to each other
            //NOTE: OPTIMIZATION TIPS
            //1. over approx with a circle and see if that collides
            //2. Calculate the closest verticie to the mass center of the polygon and ignore all other verticies
            for line in lines.iter() {
                for otherline in otherlines.iter() {
                    let (collision, local_collision_offset) = checkCollision(*line, *otherline);
                    if collision {
                        return Some(collisionRecord {desired_movement: match record {
                            Some(value) => value.desired_movement,
                            None => Vec2::new(0.0, 0.0)
                        } + local_collision_offset});
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
                let (collision, local_collision_offset) = checkCircleCollisionWithPolygon(other.getcenter(), other.getradius(), *line);
                if collision {
                    return Some(collisionRecord {desired_movement: match record {
                        Some(value) => value.desired_movement,
                        None => Vec2::new(0.0, 0.0)
                    } + local_collision_offset});
                }
            }
            
        }
        return record;
    }
    fn update(&mut self, record: &Option<collisionRecord>, dt: f64) {
        //self.center += self.velocity;
        if self.staticshape {
            return;
        }
        match record {
            Some(value) => {
                self.moverelative(value.desired_movement+ self.velocity);
            }
            None => {self.moverelative(self.velocity)}
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
    fn moverelative (&mut self, pos: Vec2) {
        for point in self.vertices.iter_mut() {
            point[0]+= pos.x;
            point[1]+= pos.y;
        }
    }
    fn set_static (&mut self, set: bool) {
        self.staticshape = set;
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
            staticshape: false,
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
                let distance = (self.center - othercenter);
                let overlap = (self.radius + other.getradius()) as f64 - distance.length();
                let axis = Vec2::unit_vector(distance);
                let posmovment = axis * overlap;

                return Some(collisionRecord {desired_movement: match record {
                    Some(value) => value.desired_movement,
                    None => Vec2::new(0.0, 0.0)
                } + posmovment});
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
                let (collision, local_collision_offset) = checkCircleCollisionWithPolygon(self.center, self.radius, *line);
                if collision {
                    //return Some(collisionRecord {desired_movement: local_collision_offset*-1.0}); 
                    return Some(collisionRecord {desired_movement: match record {
                        Some(value) => value.desired_movement,
                        None => Vec2::new(0.0, 0.0)
                    } + local_collision_offset*-1.0});//The -1.0 is to make sure the circle moves away from the rectangle and not into it since the offset is based on the rectangle
                }

            }
        }
        return record;
    }
    fn update(&mut self, record: &Option<collisionRecord>, dt: f64) {
        if self.staticshape {
            return;
        }
        self.center += self.velocity;
        match record {
            Some(value) => {
                self.moverelative(value.desired_movement);
            }
            None => {}
        }
    }
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
    fn moverelative (&mut self, pos: Vec2) {
        self.center +=pos;
    }
    fn set_static (&mut self, set: bool) {
        self.staticshape = set;
    }
    
}

//check if two lines intersect each other
fn checkCollision(line1: [[f64;2];2], line2: [[f64;2];2]) -> (bool, Vec2) {
    let matrix = Matrix2::new(line1[0][0] - line1[1][0], line2[1][0] - line2[0][0], line1[0][1] - line1[1][1], line2[1][1] - line2[0][1]);
    let vector = Vector2::new(line1[0][0] - line2[0][0], line1[0][1] - line2[0][1]);
    let decomp = matrix.lu();
    let result = decomp.solve(&vector);
    match result {
        Some(solution) => {
            
            let line = Vec2::new(line1[1][0] - line1[0][0], line1[1][1] - line1[0][1]);
            let mut offset = Vec2::new(0.0, 0.0);
            if solution[0] >= 0.5 {
                offset = line*(solution[0]-1.0);
            } else {
                offset = line*solution[0];
            }
            

            println!("Solution: {}, {}", solution[0], solution[1]);
            return (solution[0] >= 0.0 && solution[0] <= 1.0 && solution[1] >= 0.0 && solution[1] <= 1.0, offset);

            
        }
        None => {return (false, Vec2::new(0.0, 0.0))}
    }
}

fn checkCircleCollisionWithPolygon(pos: Vec2, radius: f64, vertices: [[f64;2];2]) -> (bool, Vec2){
    let v = Vector2::new(vertices[1][0] - vertices[0][0], vertices[1][1] - vertices[0][1]);
    let k = Vector2::new(pos.x - vertices[0][0], pos.y - vertices[0][1]);
    
    let negative_p_half = v.dot(&k)/v.norm_squared();
    let sqroot = ((negative_p_half*negative_p_half) - (k.norm_squared() - radius*radius)/v.norm_squared()).sqrt();


    //Does not have a solution
    if sqroot.is_nan() {
        return (false, Vec2::new(0.0, 0.0));
    }

    let t = negative_p_half - sqroot;


    if t>=0. && t<=1. {

        let line = Vec2::new(vertices[1][0] - vertices[0][0], vertices[1][1] - vertices[0][1]);
        let line_from_point_to_start = Vec2::new(vertices[0][0] - pos.x, vertices[0][1] - pos.y);
        let projection_line = vector_projection(line_from_point_to_start, line);
        let offset = line_from_point_to_start - projection_line;

        let tt = (offset.x + pos.x)/line.x;
        if tt >= 0.0 && tt <= 1.0 {
            let k = radius/offset.length() - 1.0;
            return (true, offset*k);
        } else {
            let alt_one = Vec2::new(vertices[0][0] - pos.x, vertices[0][1] - pos.y);
            let alt_two = Vec2::new(vertices[1][0] - pos.x, vertices[1][1] - pos.y);
            if alt_one.length() < alt_two.length() {
                let k = radius/alt_one.length() - 1.0;
                return (true, alt_one*k);
            } else {
                let k = radius/alt_two.length() - 1.0;
                return (true, alt_two*k);
            }
        }

        return (true, offset*10.0);
        
    }

    


    return (false, Vec2::new(0.0, 0.0));

}


pub fn vector_projection(a: Vec2, b: Vec2) -> Vec2 {
    let dot = Vec2::dot(a, b);
    let length = b.length()*b.length();
    return b*(dot/length);
}