use graphics::types::Matrix2d;
use nalgebra::Matrix2;
use nalgebra::Vector2;
use opengl_graphics::GlGraphics;
use piston::RenderArgs;

use super::collision::approx_are_colliding;
use super::collision::collision_between_polygons;
use super::collision::line_math;
use super::collision::norm_of;
use super::traits::{collisionRecord, Object};
use crate::{
    game::draw::{draw_circle, draw_polygon},
    vector::vector::Vec2,
};

pub struct Rectangle {
    center_of_mass: Vec2,
    circle_center: Vec2,
    radius: f64,
    vertices: Vec<[f64; 2]>,
    mass: f64,
    angular_velocity: f64, // Positive direction clockwise.
    velocity: Vec2,
    potnrg: f64,
    form: String,
    staticshape: bool,
    triangulations: Vec<Vec<usize>>,
    inertia: f64,
    density: f64,
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
    pub fn new(mut vertices: Vec<[f64; 2]>, mass: f64) -> Rectangle {
        let c = approx_circle_hitbox(&vertices);
        make_vertices_anti_clockwise(&mut vertices);
        let (center_of_mass, triangles, total_area,triangle_inertia ,triangle_propeties) = calc_mass_center(&vertices);
        Rectangle {
            center_of_mass: center_of_mass,
            circle_center: c.0,
            radius: c.1,
            vertices,
            mass,
            angular_velocity: 0.01,
            velocity: Vec2::new(0.2, 0.2),
            potnrg: 0.0,
            form: "Rectangle".to_string(),
            staticshape: false,
            triangulations: triangles,
            density: mass / total_area,
            inertia: calculate_moment_of_inertia_of_polygon(triangle_inertia, triangle_propeties, center_of_mass, mass),
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
            if approx_are_colliding(
                self.circle_center,
                self.radius,
                other.get_circle_center(),
                other.getradius(),
            ) {
                let relative_velocity = self.velocity - other.getvel();

                // this works for simple collision (collision point between mass_centers) but probably not for complex collision (point of collision on same side of mass_center ( e.g left of both).)
                // not hard to fix better. Store a "recent move" for each vertex and use that, since currently the angle is used to calculate that.
                let added_angle = self.angular_velocity + other.get_angular_vel();
                match collision_between_polygons(
                    &self.vertices,
                    self.center_of_mass,
                    &other.getvertices(),
                    other.getcenter(),
                    &relative_velocity,
                    added_angle,
                ) {
                    Some((norm, move_to_resolve, point_of_collision)) => {
                        // scalar_of_vel should be improved, it works on the relative distance, not the distance
                        let impulse = calculate_impulse(
                            self.velocity - other.getvel(),
                            self.mass,
                            other.get_mass(),
                            norm,
                            1.0,
                            point_of_collision-self.center_of_mass,
                            point_of_collision-other.getcenter(),
                            self.inertia,
                            other.get_inertia(),
                        );
                        return Some(collisionRecord {
                            desired_movement: match record {
                                Some(value) => value.desired_movement,
                                None => Vec2::new(0.0, 0.0),
                            } + move_to_resolve,
                            impulse: impulse*norm/self.mass,
                            impulse_angular: impulse*Vec2::dot(norm, point_of_collision-self.center_of_mass)/self.inertia,
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
                if collision
                    && max_distance.squared_length() < local_collision_offset.squared_length()
                {
                    max_distance = local_collision_offset;
                }
            }
            if max_distance.squared_length() != 0.0 {
                let relative_speed = self.velocity - other.getvel();
                let impulse = calculate_impulse(
                    relative_speed,
                    self.mass,
                    other.get_mass(),
                    max_distance,
                    1.0,
                    Vec2::new(0.0, 0.0),
                    Vec2::new(0.0, 0.0),
                    1.0,
                    1.0,
                );
                return Some(collisionRecord {
                    desired_movement: match record {
                        Some(value) => value.desired_movement,
                        None => Vec2::new(0.0, 0.0),
                    } + Vec2::unit_vector(relative_speed)
                        * max_distance.length()
                        * -1.0,
                    impulse: impulse*max_distance/self.mass,
                    impulse_angular: 0.0,
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
                self.velocity += value.impulse;
                self.moverelative(value.desired_movement + self.velocity);
                println!("impulse: {:?}", value.impulse_angular);
                self.angular_velocity += value.impulse_angular;
            }
            None => self.moverelative(self.velocity),
        }
        rotate_vertices(
            self.center_of_mass,
            &mut self.vertices,
            self.angular_velocity,
            &mut self.circle_center,
        );
    }
    fn draw(&self, graphics: &mut GlGraphics, transform: Matrix2d, args: &RenderArgs) {
        //draw_rect(self.center, [(self.width) as f64, self.height as f64], transform, graphics)
        // draw the circle that approximates the polygon
        //draw_circle(self.circle_center, self.radius, transform, graphics, args);
        //draw_polygon(self.vertices.as_slice(), transform, graphics, args);
        for tri in self.triangulations.iter() {
            let mut draw_tri = Vec::new();
            for p in tri.iter() {
                draw_tri.push(self.vertices[*p]);
            }
            draw_polygon(draw_tri.as_slice(), transform, graphics, args);
        }
        // draw point of collision
        //draw_circle(self.temp, 0.01, transform, graphics, args);
        // draw the mass_centre
        draw_circle(self.getcenter(), 0.001, transform, graphics, args)
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
    fn get_angular_vel(&self) -> f64 {
        self.angular_velocity
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
    fn get_mass(&self) -> f64 {
        return self.mass;
    }
    fn get_inertia(&self) -> f64 {
        return self.inertia;
    }
}

fn calculate_moment_of_inertia_of_polygon(
    triangles: Vec<Vec<[f64;2]>>,
    triangle_propeties: Vec<(Vec2, f64)>,
    polygon_center: Vec2,
    mass: f64,
) -> f64 {
    let mut moment_total = 0.0;
    for (triangle, properties) in triangles.iter().zip(triangle_propeties.iter()) {
        let mut moment = 0.0;
        let p1 = Vec2::new(triangle[0][0], triangle[0][1]);
        let p2 = Vec2::new(triangle[1][0], triangle[1][1]);
        let p3 = Vec2::new(triangle[2][0], triangle[2][1]);
        let center = properties.0;
        let area = properties.1;
        let width_total = (p2-p1).length();
        let height = 2.0*area/width_total;
        let density = mass/area;
        let u1 = p2 - p1;
        let u2 = p3-p1;

        let p4 = p1 + Vec2::dot(u1, u2)*u1/Vec2::dot(u1, u1);
        let w1 = (p4 - p1).length();
        let w2 = (p4 - p2).length();
        let I1 = density*(height*w1*w1*w1/4.0 + height*height*height*w1/12.0);
        let I2 = density*(height*w2*w2*w2/4.0 + height*height*height*w2/12.0);
        
        if Vec2::cross(p1-p3, p4-p3) > 0.0 {
            moment += I1;
        } else {
            moment -= I1;
        }
        if Vec2::cross(p4-p3, p2-p3) > 0.0 {
            moment += I2;
        } else {
            moment -= I2;
        }
        moment_total += moment - mass*(p3-polygon_center).squared_length();

    }
    println!("moment of inertia: {}", moment_total);
    return moment_total.abs();
}

fn rotate_vertices(
    center: Vec2,
    vertices: &mut Vec<[f64; 2]>,
    angle: f64,
    circle_center: &mut Vec2,
) {
    for point in vertices.iter_mut() {
        let x = point[0] - center.x;
        let y = point[1] - center.y;
        point[0] = x * angle.cos() - y * angle.sin() + center.x;
        point[1] = x * angle.sin() + y * angle.cos() + center.y;
    }
    //Rotate the circle center around the center of mass
    let x = circle_center.x - center.x;
    let y = circle_center.y - center.y;
    circle_center.x = x * angle.cos() - y * angle.sin() + center.x;
    circle_center.y = x * angle.sin() + y * angle.cos() + center.y;
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

                let impulse = calculate_impulse(
                    self.velocity - other.getvel(),
                    self.mass,
                    other.get_mass(),
                    distance,
                    1.,
                    Vec2::new(0.0, 0.0),
                    Vec2::new(0.0, 0.0),
                    1.0,
                    1.0
                );

                return Some(collisionRecord {
                    desired_movement: match record {
                        Some(value) => value.desired_movement,
                        None => Vec2::new(0.0, 0.0),
                    } + posmovment,
                    impulse: impulse*distance/self.mass,
                    impulse_angular: 0.0,
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
                if collision
                    && local_collision_offset.squared_length() > max_distance.squared_length()
                {
                    max_distance = local_collision_offset;
                }
            }
            if max_distance.squared_length() != 0.0 {
                let relative_speed = self.velocity - other.getvel();
                //Calulcate the normal of the collision
                let normal = Vec2::unit_vector(max_distance);
                let impulse =
                    calculate_impulse(relative_speed, self.mass, other.get_mass(), normal, 1., Vec2::new(0.0, 0.0),Vec2::new(0.0, 0.0), 1.0, 1.0);
                return Some(collisionRecord {
                    desired_movement: match record {
                        Some(value) => value.desired_movement,
                        None => Vec2::new(0.0, 0.0),
                    } + max_distance * Vec2::unit_vector(relative_speed),
                    impulse: impulse*normal/self.mass, //return Some(collisionRecord {desired_movement: local_collision_offset*-1.0});
                                      //The -1.0 is to make sure the circle moves away from the rectangle and not into it since the offset is based on the rectangle
                    impulse_angular: 0.0,
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
                self.moverelative(value.desired_movement + self.velocity);
            }
            None => self.moverelative(self.velocity),
        }
    }
    fn draw(&self, graphics: &mut GlGraphics, transform: Matrix2d, args: &RenderArgs) {
        draw_circle(
            self.center_of_mass,
            self.radius as f64,
            transform,
            graphics,
            args,
        );
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
    fn get_angular_vel(&self) -> f64 {
        0.0 // IF NEEDED TO KEEP TRACK; Change this
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
    fn get_mass(&self) -> f64 {
        return self.mass;
    }
    fn get_inertia(&self) -> f64 {
        return 0.0;
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
fn calc_mass_center(vert: &Vec<[f64; 2]>) -> (Vec2, Vec<Vec<usize>>, f64, Vec<Vec<[f64;2]>>, Vec<(Vec2, f64)>) {
    assert!(vert.len() >= 2);
    if vert.len() == 2 {
        // a line, return the average of x and y
        return (
            Vec2::new(
                (vert[0][0] + vert[1][0]) / 2.0,
                (vert[0][1] + vert[1][1]) / 2.0,
            ),
            vec![vec![0,1]],
            0.0,
            vec![],
            vec![],
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
    let mut triangles = vec![];
    let mut triangle_propeties = vec![];
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
            // Check if the iplus1 vertex is a concave vertex
            let line1 = [vertices[iplus1], vertices[i]];
            let norm_of_first_line = norm_of(line1);
            // if the scalar of the projection of vec2 onto norm_of_first_line is > 0.0 , abort
            if Vec2::dot(vec2, norm_of_first_line) > 0.0 {
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
            triangles.push(vec![vertices[i], vertices[iplus1], vertices[iplus2]]);
            triangle_propeties.push(temp);
            sum_of_centre += temp.0*temp.1;
            area_sum += temp.1;
            vertices.remove(iplus1);
            break;
        }
    }
    let temp = calc_triangle_mass_center_and_area([vertices[0], vertices[1], vertices[2]]);
    sum_of_centre += temp.0*temp.1;
    area_sum += temp.1;
    triangles.push(vertices);
    

    let mut mapped_index = Vec::new();
    for tri in &triangles {
        let mut middle_stage = Vec::new();
        for p in tri {
            for (index, real_p) in vert.iter().enumerate() {
                if p == real_p {
                    middle_stage.push(index);
                    break;
                }
            }
        }
        mapped_index.push(middle_stage);
    }
    return (sum_of_centre / area_sum, mapped_index, area_sum, triangles, triangle_propeties);
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
    return (centroid, area);
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
    let mut min_sq: f64;
    let mut radius_sq: f64;
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

fn calculate_impulse(
    relative_speed: Vec2,
    mass: f64,
    mass_other: f64,
    normal: Vec2,
    restitution: f64,
    r: Vec2,
    r_other: Vec2,
    inertia: f64,
    inertia_other: f64,
) -> f64 {
    let normal_unit = Vec2::unit_vector(normal);
    //let j = -(1.0 + restitution) * Vec2::dot(relative_speed, normal_unit)
    //    / (1.0 / mass + 1.0 / mass_other);
    let part1 = Vec2::dot(normal, normal)*((1.0/mass) + (1.0/mass_other));
    let part2 = (Vec2::dot(r, normal)*Vec2::dot(r, normal))/inertia;
    let part3 = (Vec2::dot(r_other, normal)*Vec2::dot(r_other, normal))/inertia_other;
    let j = -(1.0 + restitution)*Vec2::dot(relative_speed, normal)/(part1 + part2 + part3);
    return j;
}

/// reverses the vec, if it is ordered wrong. Vertices consiting of 2 points, might be problematic.
/// # Panics
/// Panics if length of the passed Vec is 1 or 0.
fn make_vertices_anti_clockwise(vertices: &mut Vec<[f64; 2]>) {
    let point = {
        // taking middle point of the first edge.
        let x_diff = vertices[1][0] - vertices[0][0];
        let y_diff = vertices[1][1] - vertices[0][1];
        let temp_vector = 0.5 * Vec2::new(x_diff, y_diff);
        [
            vertices[0][0] + temp_vector.x,
            vertices[0][1] + temp_vector.y,
        ]
    };
    let first_line = [vertices[1], vertices[0]];
    let normal = norm_of(first_line);
    let ray = [point, [point[0] + normal.x, point[1] + normal.y]];

    let mut passed_lines = 0;
    let mut prev_index = 1;
    for index in 2..vertices.len() {
        let colliding_line = [vertices[index], vertices[prev_index]];
        let (t, s) = line_math(ray, colliding_line);
        // if the lines are parallel, we should not need to worry.

        if t > 0.0 && s < 1.0 && s > 0.0 {
            passed_lines += 1;
        } else if t > 0.0 && s == 1.0 {
            // we are intersecting another vertex. This should only count if the next vertex is on the other side. since then we are actually crossing from inside to outside or vice-versa
            // Invariant, prev_vertex cannot lie on the ray
            // The vertices lies on different sides if solution to lines intersecting lies within 0.0 < ss < 1.0
            let prev_vertex = vertices[prev_index];
            let index_plus_one = {
                if index == vertices.len() - 1 {
                    0
                } else {
                    index + 1
                }
            };
            let next_vertex = vertices[index_plus_one];
            let vertex_line = [next_vertex, prev_vertex];
            let (_, new_s) = line_math(ray, vertex_line);
            if new_s > 0.0 && new_s < 1.0 {
                passed_lines += 1;
            }
        }
        prev_index = index;
    }

    if passed_lines % 2 == 1 {
        // passed an odd amount, our ray started towards the inside.
        vertices.reverse();
    }
}
