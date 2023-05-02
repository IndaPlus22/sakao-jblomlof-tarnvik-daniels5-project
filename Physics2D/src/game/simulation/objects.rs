use std::f64::consts::PI;

use gfx_device_gl::{CommandBuffer, Resources};
use gfx_graphics::GfxGraphics;
use graphics::types::Matrix2d;
use nalgebra::Matrix2;
use nalgebra::Vector2;
use opengl_graphics::GlGraphics;
use piston::RenderArgs;

use super::collision::approx_are_colliding;
use super::collision::collision_between_polygons;
use super::traits::{collisionRecord, Object};
use crate::{
    game::draw::{draw_circle, draw_polygon, draw_rect},
    vector::vector::Vec2,
};

pub struct Rectangle {
    center_of_mass: Vec2,
    circle_center: Vec2,
    radius: f64,
    vertices: Vec<[f64; 2]>,
    mass: f64,
    velocity: Vec2,
    potnrg: f64,
    form: String,
    staticshape: bool,
}

pub struct Circle {
    center_of_mass: Vec2,
    radius: f64,
    mass: f64,
    velocity: Vec2,
    potnrg: f64,
    form: String,
    staticshape: bool,
}

impl Rectangle {
    pub fn new(vertices: Vec<[f64; 2]>, mass: f64) -> Rectangle {
        let c = approx_circle_hitbox(&vertices);
        Rectangle {
            center_of_mass: calc_mass_center(&vertices),
            circle_center: c.0,
            radius: c.1,
            vertices,
            mass,
            velocity: Vec2::new(0., 0.),
            potnrg: 0.0,
            form: "Rectangle".to_string(),
            staticshape: false,
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
            if approx_are_colliding(self.circle_center, self.radius, other.get_circle_center(), other.getradius()) {
                let relative_velocity = self.velocity - other.getvel();
                match collision_between_polygons(
                    &self.vertices,
                    &relative_velocity,
                    &other.getvertices(),
                ) {
                    Some((norm, scalar_of_vel)) => {
                        // scalar_of_vel should be improved, it works on the relative distance, not the distance
                        // print normal x and y
                        
                        return Some(collisionRecord {
                            desired_movement: match record {
                                Some(value) => value.desired_movement,
                                None => Vec2::new(0.0, 0.0),
                            } + scalar_of_vel * self.velocity,
                            impulse: calculate_impulse(self.velocity-other.getvel(), self.mass, other.get_mass(), norm, 1.0)
                        });
                    }
                    None => (),
                }
            }
        } else if other.gettype() == "Circle" {
            let mut lines = Vec::new();
            for i in 0..self.vertices.len() {
                if i == self.vertices.len() - 1 {
                    lines.push([self.vertices[i], self.vertices[0]]);
                } else {
                    lines.push([self.vertices[i], self.vertices[i + 1]]);
                }
            }
            let mut max_distance = Vec2::new(0.0, 0.0);
            for line in lines.iter() {
                let (collision, local_collision_offset) =
                    checkCircleCollisionWithPolygon(other.getcenter(), other.getradius(), *line);
                if collision && max_distance.squared_length() < local_collision_offset.squared_length() {
                    max_distance = local_collision_offset;
                }
            }
            if max_distance.squared_length() != 0.0 {
                let relative_speed = self.velocity - other.getvel();
                let impulse = calculate_impulse(relative_speed, self.mass, other.get_mass(), max_distance, 1.0);
                return Some(collisionRecord {
                    desired_movement: match record {
                        Some(value) => value.desired_movement,
                        None => Vec2::new(0.0, 0.0),
                    } + Vec2::unit_vector(relative_speed) * max_distance.length()*-1.0,
                    impulse: impulse
                });
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
                self.velocity+=value.impulse;
                self.moverelative(value.desired_movement + self.velocity);
            }
            None => self.moverelative(self.velocity),
        }
    }
    fn draw(&self, graphics: &mut GlGraphics, transform: Matrix2d, args: &RenderArgs) {
        //draw_rect(self.center, [(self.width) as f64, self.height as f64], transform, graphics)
        //draw_circle(self.circle_center, self.radius, transform, graphics);
        draw_polygon(
            self.vertices.as_slice(),
            transform,
            graphics,
            args,
        );
        // pls dont i am angry :()
        // draw_circle(self.getcenter(), 1.0, transform, graphics, args)
    }
    fn getcenter(&self) -> Vec2 {
        return self.center_of_mass;
    }
    fn gettype(&self) -> String {
        return self.form.clone();
    }
    fn get_circle_center(&self) -> Vec2 {
        return self.circle_center;
    }
    fn getradius(&self) -> f64 {
        return self.radius;
    }
    fn getvertices(&self) -> Vec<[f64; 2]> {
        return self.vertices.to_vec();
    }
    fn getvel(&self) -> Vec2 {
        self.velocity
    }
    fn setvel(&mut self, vel: Vec2) {
        self.velocity = vel;
    }
    fn moverelative(&mut self, pos: Vec2) {
        self.center_of_mass += pos;
        self.circle_center += pos;
        for point in self.vertices.iter_mut() {
            point[0] += pos.x;
            point[1] += pos.y;
        }
    }
    fn set_static(&mut self, set: bool) {
        self.staticshape = set;
    }
    fn get_mass (&self) -> f64 {
        return self.mass;
    }
}

impl Circle {
    pub fn new(center: Vec2, radius: f64, mass: f64) -> Circle {
        Circle {
            center_of_mass: center,
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
            let distance = (self.center_of_mass - othercenter).length();
            if distance < (self.radius + other.getradius()) as f64 {
                let distance = (self.center_of_mass - othercenter);
                let overlap = (self.radius + other.getradius()) as f64 - distance.length();
                let axis = Vec2::unit_vector(distance);
                let posmovment = axis * overlap;

                let impulse = calculate_impulse(self.velocity - other.getvel(), self.mass, other.get_mass(), distance, 1.);

                return Some(collisionRecord {
                    desired_movement: match record {
                        Some(value) => value.desired_movement,
                        None => Vec2::new(0.0, 0.0),
                    } + posmovment,
                    impulse: impulse
                });
            }
        } else if other.gettype() == "Rectangle" {
            let mut lines = Vec::new();
            for i in 0..other.getvertices().len() {
                if i == other.getvertices().len() - 1 {
                    lines.push([other.getvertices()[i], other.getvertices()[0]]);
                } else {
                    lines.push([other.getvertices()[i], other.getvertices()[i + 1]]);
                }
            }
            let mut max_distance = Vec2::new(0.0, 0.0);
            for line in lines.iter() {
                let (collision, local_collision_offset) =
                    checkCircleCollisionWithPolygon(self.center_of_mass, self.radius, *line);
                if collision && local_collision_offset.squared_length() > max_distance.squared_length() {
                    max_distance = local_collision_offset;
                }
            }
            if max_distance.squared_length() != 0.0 {
                let relative_speed = self.velocity - other.getvel();
                //Calulcate the normal of the collision
                let normal = Vec2::unit_vector(max_distance);
                let impulse = calculate_impulse(relative_speed, self.mass, other.get_mass(), normal, 1.);
                return Some(collisionRecord {
                    desired_movement: match record {
                        Some(value) => value.desired_movement,
                        None => Vec2::new(0.0, 0.0),
                    } + max_distance * Vec2::unit_vector(relative_speed),
                    impulse: impulse
                    //return Some(collisionRecord {desired_movement: local_collision_offset*-1.0});
                     //The -1.0 is to make sure the circle moves away from the rectangle and not into it since the offset is based on the rectangle
                });
            }
            
        }
        return record;
    }
    fn update(&mut self, record: &Option<collisionRecord>, dt: f64) {
        if self.staticshape {
            return;
        }
        match record {
            Some(value) => {
                self.velocity += value.impulse;
                self.moverelative(value.desired_movement+self.velocity);
                
            }
            None => {self.moverelative(self.velocity)}
        }
        

    }
    fn draw(&self, graphics: &mut GlGraphics, transform: Matrix2d, args: &RenderArgs) {
        draw_circle(self.center_of_mass, self.radius as f64, transform, graphics, args);
    }
    fn getcenter(&self) -> Vec2 {
        return self.center_of_mass;
    }
    fn gettype(&self) -> String {
        return self.form.clone();
    }
    fn get_circle_center(&self) -> Vec2 {
        return self.center_of_mass;
    }
    fn getradius(&self) -> f64 {
        return self.radius;
    }
    fn getvertices(&self) -> Vec<[f64; 2]> {
        return vec![];
    }
    fn getvel(&self) -> Vec2 {
        self.velocity
    }
    fn setvel(&mut self, vel: Vec2) {
        self.velocity = vel;
    }

    fn moverelative(&mut self, pos: Vec2) {
        self.center_of_mass += pos;
    }
    fn set_static(&mut self, set: bool) {
        self.staticshape = set;
    }
    fn get_mass (&self) -> f64 {
        return self.mass;
    }
}

//check if two lines intersect each other
fn checkCollision(line1: [[f64; 2]; 2], line2: [[f64; 2]; 2]) -> (bool, Vec2) {
    let matrix = Matrix2::new(
        line1[0][0] - line1[1][0],
        line2[1][0] - line2[0][0],
        line1[0][1] - line1[1][1],
        line2[1][1] - line2[0][1],
    );
    let vector = Vector2::new(line1[0][0] - line2[0][0], line1[0][1] - line2[0][1]);
    let decomp = matrix.lu();
    let result = decomp.solve(&vector);
    match result {
        Some(solution) => {
            let line = Vec2::new(line1[1][0] - line1[0][0], line1[1][1] - line1[0][1]);
            let mut offset = Vec2::new(0.0, 0.0);
            if solution[0] >= 0.5 {
                offset = line * (solution[0] - 1.0);
            } else {
                offset = line * solution[0];
            }

            return (
                solution[0] >= 0.0
                    && solution[0] <= 1.0
                    && solution[1] >= 0.0
                    && solution[1] <= 1.0,
                offset,
            );
        }
        None => return (false, Vec2::new(0.0, 0.0)),
    }
}

fn checkCircleCollisionWithPolygon(
    pos: Vec2,
    radius: f64,
    vertices: [[f64; 2]; 2],
) -> (bool, Vec2) {
    let v = Vector2::new(
        vertices[1][0] - vertices[0][0],
        vertices[1][1] - vertices[0][1],
    );
    let k = Vector2::new(pos.x - vertices[0][0], pos.y - vertices[0][1]);

    let negative_p_half = v.dot(&k) / v.norm_squared();
    let sqroot = ((negative_p_half * negative_p_half)
        - (k.norm_squared() - radius * radius) / v.norm_squared())
    .sqrt();

    //Does not have a solution
    if sqroot.is_nan() {
        return (false, Vec2::new(0.0, 0.0));
    }

    let t = negative_p_half - sqroot;

    if t >= 0. && t <= 1. {
        let line = Vec2::new(
            vertices[1][0] - vertices[0][0],
            vertices[1][1] - vertices[0][1],
        );
        let line_from_point_to_start = Vec2::new(vertices[0][0] - pos.x, vertices[0][1] - pos.y);
        let projection_line = vector_projection(line_from_point_to_start, line);
        let offset = line_from_point_to_start - projection_line;

        let tt = (offset.x + pos.x) / line.x;
        if tt >= 0.0 && tt <= 1.0 {
            let k = radius / offset.length() - 1.0;
            return (true, offset * k);
        } else {
            let alt_one = Vec2::new(vertices[0][0] - pos.x, vertices[0][1] - pos.y);
            let alt_two = Vec2::new(vertices[1][0] - pos.x, vertices[1][1] - pos.y);
            if alt_one.length() < alt_two.length() {
                let k = radius / alt_one.length() - 1.0;
                return (true, alt_one * k);
            } else {
                let k = radius / alt_two.length() - 1.0;
                return (true, alt_two * k);
            }
        }

        return (true, offset * 10.0);
    }

    return (false, Vec2::new(0.0, 0.0));
}

/// Calculate center of mass for a homogeneous polygon.
/// Returns a Vec2 of the point where the center mass is located.
/// Takes a reference to a vec of points such that it is ordered after connecting vertices.
/// # Panics
/// Panics if vertices consists of 1 or 0 elements
fn calc_mass_center(vert: &Vec<[f64; 2]>) -> Vec2 {
    assert!(vert.len() >= 2);
    if vert.len() == 2 {
        // a line, return the average of x and y
        return Vec2::new(
            (vert[0][0] + vert[1][0]) / 2.0,
            (vert[0][1] + vert[1][1]) / 2.0,
        );
    }
    // using math from https://en.wikipedia.org/wiki/Centroid
    // more specifically https://en.wikipedia.org/wiki/List_of_centroids#2-D_Centroids
    // more specifically https://www.mathopenref.com/coordcentroid.html

    // Problem - Concave shapes.
    // Solution - Ear-algorithm. https://en.wikipedia.org/wiki/Polygon_triangulation
    //https://www.geometrictools.com/Documentation/TriangulationByEarClipping.pdf

    // Find triangles and "tear them apart" from the main polygon.
    // This could definitly be improved.
    let mut vertices = vert.clone();
    let mut sum_of_centre = Vec2::new(0.0, 0.0);
    let mut area_sum = 0.0;

    while vertices.len() > 3 {
        for i in 0..vertices.len() {
            let (iplus1, iplus2) = if i == vertices.len() - 1 {
                (0, 1)
            } else if i == vertices.len() - 2 {
                (i + 1, 0)
            } else {
                (i + 1, i + 2)
            };
            let vec1 = Vec2::new(
                vertices[iplus1][0] - vertices[i][0],
                vertices[iplus1][1] - vertices[i][1],
            );
            let vec2 = Vec2::new(
                vertices[iplus2][0] - vertices[iplus1][0],
                vertices[iplus2][1] - vertices[iplus1][1],
            );
            // get the angle between, PI -.. because the angle is between the VECTORS, and not the LINES
            let theta = PI - f64::acos(Vec2::dot(vec1, vec2) / (vec1.length() * vec2.length()));

            if theta > PI / 2.0 {
                continue;
            }

            // now we need to see if the last side, from iplus2 to i goes over any other lines.
            // we do this by looking if any other vertex lies in the triangle.
            // a vertex lies in the triangle if and only if can be expressed as a linear combination with restraints
            // https://mathworld.wolfram.com/TriangleInterior.html
            let vec2 = Vec2::new(
                vertices[iplus2][0] - vertices[i][0],
                vertices[iplus2][1] - vertices[i][1],
            );

            let matrix = Matrix2::new(vec1.x, vec2.x, vec1.y, vec2.y);
            let decomp = matrix.lu();
            let mut vertex_inside = false;
            for j in 0..vertices.len() {
                if j == i || j == iplus1 || j == iplus2 {
                    continue;
                }
                let equal_vector = Vector2::new(
                    vertices[j][0] - vertices[i][0],
                    vertices[j][1] - vertices[i][1],
                );
                let sol = decomp.solve(&equal_vector);
                match sol {
                    Some(solution) => {
                        if solution.x >= 0.0 && solution.y >= 0.0 && solution.x + solution.y <= 1.0
                        {
                            vertex_inside = true;
                            break;
                        }
                    }
                    None => {
                        () // this should maybe panic, only happens if the 2 vectors have same direction
                    }
                };
            }
            if vertex_inside {
                continue;
            }

            // we found an ear, remove it.
            let temp = calc_triangle_mass_center_and_area([
                vertices[i],
                vertices[iplus1],
                vertices[iplus2],
            ]);
            sum_of_centre += temp.0;
            area_sum += temp.1;
            vertices.remove(iplus1);
            break;
        }
    }
    let temp = calc_triangle_mass_center_and_area([vertices[0], vertices[1], vertices[2]]);
    sum_of_centre += temp.0;
    area_sum += temp.1;
    return sum_of_centre / area_sum;
}

fn calc_triangle_mass_center_and_area(vertices: [[f64; 2]; 3]) -> (Vec2, f64) {
    let length1 = Vec2::new(
        vertices[1][0] - vertices[0][0],
        vertices[1][1] - vertices[0][1],
    )
    .length();
    let length2 = Vec2::new(
        vertices[2][0] - vertices[0][0],
        vertices[2][1] - vertices[0][1],
    )
    .length();
    let length3 = Vec2::new(
        vertices[1][0] - vertices[2][0],
        vertices[1][1] - vertices[2][1],
    )
    .length();
    let semi_perimiter = (length1 + length2 + length3) / 2.0;
    let area = (semi_perimiter
        * (semi_perimiter - length1)
        * (semi_perimiter - length2)
        * (semi_perimiter - length3))
        .sqrt();
    let centroid = Vec2::new(
        (vertices[0][0] + vertices[1][0] + vertices[2][0]) / 3.0,
        (vertices[0][1] + vertices[1][1] + vertices[2][1]) / 3.0,
    );
    return (centroid * area, area);
}

/// Returns a circle such that all vertices lies inside the circle,
/// This function aims to lower the radius of that fuction.
fn approx_circle_hitbox(vertices: &Vec<[f64; 2]>) -> (Vec2, f64) {
    // make a guess where the center lies, calculate the distance to all vertices
    // move to a new point. The point towarads the vertex with the longest distance
    // the moved distance should be (max_dist - min_dist) / 2
    // repeat a number of times, or until some condition is satisfied.
    const NUMBER_OF_ITERATION_FOR_APPROXIMATION: usize = 10;

    let mut point = Vec2::new(vertices[0][0], vertices[0][1]);
    let mut min_sq: f64 = f64::MAX;
    let mut radius_sq: f64 = 0.0;
    let mut best_direction = Vec2::new(0.0, 0.0);
    for _ in 0..NUMBER_OF_ITERATION_FOR_APPROXIMATION {
        min_sq = f64::MAX;
        radius_sq = 0.0;
        for vertex in vertices.iter() {
            let vec = Vec2::new(vertex[0] - point.x, vertex[1] - point.y);
            let norm_sq = vec.squared_length();
            if radius_sq < norm_sq {
                radius_sq = norm_sq;
                best_direction = vec;
            }
            min_sq = min_sq.min(norm_sq)
        }
        let (r, m) = (radius_sq.sqrt(), min_sq.sqrt());
        let scalar = (1.0 - m / r) / 4.0;
        point += best_direction * scalar;
    }
    radius_sq = 0.0;
    for vertex in vertices.iter() {
        let norm_sq = Vec2::new(vertex[0] - point.x, vertex[1] - point.y).squared_length();
        radius_sq = radius_sq.max(norm_sq);
    }

    return (point, radius_sq.sqrt());
}

pub fn vector_projection(a: Vec2, b: Vec2) -> Vec2 {
    let dot = Vec2::dot(a, b);
    let length = b.length() * b.length();
    return b * (dot / length);
}


fn calculate_impulse(relative_speed: Vec2, mass: f64, mass_other: f64, normal: Vec2, restitution: f64) -> Vec2 {
    let normal_unit = Vec2::unit_vector(normal);
    let j = -(1.0 + restitution) * Vec2::dot(relative_speed, normal_unit) / (1.0/mass + 1.0/mass_other);
    return (j/mass)*normal_unit;
}