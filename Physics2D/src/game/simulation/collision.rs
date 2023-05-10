use nalgebra::{Matrix2, Vector2};

use crate::{main, vector::vector::Vec2};

use super::objects::rotate_vertices;

/// Returns whatever the polygons approximate circles collide.
pub fn approx_are_colliding(centre1: Vec2, raduis1: f64, centre2: Vec2, radius2: f64) -> bool {
    let distance = (centre1 - centre2).squared_length();
    return distance < (raduis1 + radius2) * (raduis1 + radius2);
    // if distance^2 < rad_dist^2 so is dist < rad_dist
    // since distance is >= 0, rad >= 0;
}

/// If a polygon collides with the other polygon, this returns Some((normal, t, point_of_collision)).
/// Such that t*self.velocity is the relative movement such that the polygons only touches.
///
/// # Important
/// angle_diff_relative needs to be the relative angle the objects moved
pub fn collision_between_polygons(
    main_pol_vert: &Vec<[f64; 2]>,
    mass_center_main: Vec2,
    static_pol_vert: &Vec<[f64; 2]>,
    mass_center_static: Vec2,
    relative_movement: &Vec2,
    main_angle_move: f64,
    static_angle_move: f64,
) -> Option<(Vec2, Vec2, Vec2)> {
    if relative_movement.squared_length() != 0.0
        || main_angle_move != 0.0
        || static_angle_move != 0.0
    {
        let movement_main = make_movement_frame_of_reference_based(
            main_pol_vert,
            relative_movement,
            main_angle_move,
            mass_center_main,
            static_angle_move,
            mass_center_static,
        );
        let movement_static = make_movement_frame_of_reference_based(
            static_pol_vert,
            &(-(*relative_movement)),
            static_angle_move,
            mass_center_static,
            main_angle_move,
            mass_center_main,
        );
        calculate_scalar_distance(
            main_pol_vert,
            static_pol_vert,
            &movement_main,
            &movement_static,
            relative_movement
        )
    } else {
        None
    }
}

fn type_of_collision(
    main_pol_vert: &Vec<[f64; 2]>,
    static_pol_vert: &Vec<[f64; 2]>,
    relative_movement: &Vec2,
) -> CollisionType {
    let mut norms: Vec<Vec2> = Vec::new();
    let mut points_of_collision: Vec<Vec2> = Vec::new();
    let mut some_contact = false;

    let mut prev_index_for_main = main_pol_vert.len() - 1;
    for index_for_main in 0..main_pol_vert.len() {
        let line1 = [
            main_pol_vert[prev_index_for_main],
            main_pol_vert[index_for_main],
        ];

        let mut prev_static_index = static_pol_vert.len() - 1;
        for static_index in 0..static_pol_vert.len() {
            let line2 = [
                static_pol_vert[prev_static_index],
                static_pol_vert[static_index],
            ];

            if parallel_line(line1, line2) {
                let x_diff_same_line = line1[1][0] - line1[0][0];
                if x_diff_same_line != 0.0 {
                    let x_diff_first = line2[0][0] - line1[0][0];
                    let x_diff_second = line2[1][0] - line1[0][0];
                    if x_diff_first * x_diff_second < 0.0 {
                        norms.push(norm_of(line2));
                    } else {
                        let x_diff_first = line2[0][0] - line1[1][0];
                        let x_diff_second = line2[1][0] - line1[1][0];
                        if x_diff_first * x_diff_second < 0.0 {
                            norms.push(norm_of(line2));
                        }
                    }
                } else {
                    let y_diff_first = line2[0][1] - line1[0][1];
                    let y_diff_second = line2[1][1] - line1[0][1];
                    if y_diff_first * y_diff_second < 0.0 {
                        norms.push(norm_of(line2));
                    } else {
                        let y_diff_first = line2[0][1] - line1[1][1];
                        let y_diff_second = line2[1][1] - line1[1][1];
                        if y_diff_first * y_diff_second < 0.0 {
                            norms.push(norm_of(line2));
                        }
                    }
                }
            }

            let (t, s) = line_math(line1, line2);
            if t <= 1.0 && t >= 0.0 && s <= 1.0 && s >= 0.0 {
                let t_on_end = t == 1.0 || t == 0.0;
                let s_on_end = s == 1.0 || s == 0.0;
                match (t_on_end, s_on_end) {
                    (false, false) => return CollisionType::Interference,
                    (true, false) => {
                        some_contact = true;
                        norms.push(norm_of(line2));
                        let point = {
                            let x_diff = line1[1][0] - line1[0][0];
                            let y_diff = line1[1][1] - line1[0][1];
                            Vec2::new(line1[0][0] + t * x_diff, line1[0][1] + t * y_diff)
                        };
                        points_of_collision.push(point)
                    }
                    (false, true) => {
                        some_contact = true;
                        norms.push(norm_of(line1));
                        let point = {
                            let x_diff = line1[1][0] - line1[0][0];
                            let y_diff = line1[1][1] - line1[0][1];
                            Vec2::new(line1[0][0] + t * x_diff, line1[0][1] + t * y_diff)
                        };
                        points_of_collision.push(point)
                    }
                    (true, true) => {
                        // improve norm calculation for corner-corner-collision.
                        some_contact = true;
                        let point = {
                            let x_diff = line1[1][0] - line1[0][0];
                            let y_diff = line1[1][1] - line1[0][1];
                            Vec2::new(line1[0][0] + t * x_diff, line1[0][1] + t * y_diff)
                        };
                        points_of_collision.push(point)
                    }
                }
            }

            prev_static_index = static_index;
        }
        prev_index_for_main = index_for_main;
    }

    if some_contact {
        let mut total_norm = Vec2::new(0.0, 0.0);
        let mut avg_pos = Vec2::new(0.0, 0.0);
        if norms.len() == 0 {
            // this could be improved.
            total_norm = relative_movement.clone();
        } else {
            for n in norms {
                total_norm += n;
            }
        }
        for p in points_of_collision.iter() {
            avg_pos += *p;
        }
        avg_pos /= points_of_collision.len() as f64;
        //total_norm /= total_norm.length();
        return CollisionType::Touching(total_norm, avg_pos);
    } else {
        return CollisionType::No;
    }
}

/// Returns the vector that rotates the line (from index0 to index1) 90 degrees clockwise.
pub fn norm_of(line1: [[f64; 2]; 2]) -> Vec2 {
    // assuming all vertex is ordered counter-clockwise it is
    return Vec2::new(line1[1][1] - line1[0][1], line1[0][0] - line1[1][0]);
}
enum CollisionType {
    Interference,
    Touching(Vec2, Vec2),
    No,
}

/// returns t, s
/// returns -0.01, -0.01 if line are parallell (garbage return)
pub fn line_math(line1: [[f64; 2]; 2], line2: [[f64; 2]; 2]) -> (f64, f64) {
    let matrix = Matrix2::new(
        line1[0][0] - line1[1][0],
        line2[1][0] - line2[0][0],
        line1[0][1] - line1[1][1],
        line2[1][1] - line2[0][1],
    );
    let vector = Vector2::new(line1[0][0] - line2[0][0], line1[0][1] - line2[0][1]);
    let decomp = matrix.lu();
    match decomp.solve(&vector) {
        Some(res) => {
            return (res[0], res[1]);
        }
        None => {
            //println!("{:?} ---- {:?}", matrix, vector);
            return (-0.01, -0.01);
        }
    }
}

/// Returns norm and scalar, does not compute well with relative_vel.length == 0. Only valid when knows that polygons interfers
fn calculate_scalar_distance(
    main_pol_vert: &Vec<[f64; 2]>,
    static_pol_vert: &Vec<[f64; 2]>,
    movement_of_main: &Vec<Vec2>,
    movement_of_static: &Vec<Vec2>,
    relative_vel: &Vec2,
) -> Option<(Vec2, Vec2, Vec2)> {
    const MARGIN: f64 = 0.0000001; // MARGIN FOR when parallell lines interfere.
    let mut norms: Vec<Vec2> = Vec::new();
    let mut points_of_collision: Vec<Vec2> = Vec::new();
    let mut corner_corner_norm: Vec<Vec2> = Vec::new(); // corner-corner-only
    let mut max_move: Vec2 = Vec2::new(0.0, 0.0);

    let mut prev_index_for_main = main_pol_vert.len() - 1;
    for index_for_main in 0..main_pol_vert.len() {
        let line1 = [
            main_pol_vert[prev_index_for_main],
            main_pol_vert[index_for_main],
        ];

        let mut prev_static_index = static_pol_vert.len() - 1;
        for static_index in 0..static_pol_vert.len() {
            let line2 = [
                static_pol_vert[prev_static_index],
                static_pol_vert[static_index],
            ];

            // MIGHT NOT BE 100% correct for 2 parallell lines with relative direction in the same directon as the line
            // look for the t >= 0 such that endpoints on line1 moves to line2
            // but they need to move exactly onto line2
            // so also check if the opposite.

            //rays from "main" to static
            let move_1 = [
                line1[0],
                [
                    line1[0][0] + movement_of_main[prev_index_for_main].x,
                    line1[0][1] + movement_of_main[prev_index_for_main].y,
                ],
            ];

            let move_2 = [
                line1[1],
                [
                    line1[1][0] + movement_of_main[index_for_main].x,
                    line1[1][1] + movement_of_main[index_for_main].y,
                ],
            ];
            let move_3 = [
                line2[0],
                [
                    line2[0][0] + movement_of_static[prev_static_index].x,
                    line2[0][1] + movement_of_static[prev_static_index].y,
                ],
            ];

            let move_4 = [
                line2[1],
                [
                    line2[1][0] + movement_of_static[static_index].x,
                    line2[1][1] + movement_of_static[static_index].y,
                ],
            ];

            // from here we go from static towards the "main", thus changing direction on angle and vel (hence negative sign)

            // Each element is ((vec_from_base_line"+"direction_to_go), line_to_check_collision, origin_line, scalar (1 or -1))
            let compound_lines = [
                (move_1, line2, line1, 1.0),
                (move_2, line2, line1, 1.0),
                (move_3, line1, line2, -1.0),
                (move_4, line1, line2, -1.0),
            ];
            for (mov, colliding_line, origin, scalar) in compound_lines {
                let (t, s) = line_math(mov, colliding_line);
                if t <= 1.0 && t > 0.0 && s <= 1.0 && s >= 0.0 {
                    let prev_sq = max_move.squared_length();
                    let new_sq = t
                        * t
                        * Vec2::new(mov[1][0] - mov[0][0], mov[1][1] - mov[0][1]).squared_length();
                    // invariant t > 0, max_move.length >= 0.0 thus if t >= max_move.length so is t*t >= max_move.squared_length
                    if new_sq >= prev_sq - MARGIN {
                        // dont know what to do if they need to move the same length.
                        // should probably take the closest one.
                        if new_sq > prev_sq {
                            norms.clear();
                            points_of_collision.clear();
                            corner_corner_norm.clear();
                            // we need to move from our point to end point *
                            // scalar indicates if we going from "main" or static, (1 resp. -1)
                            max_move = scalar
                                * t
                                * Vec2::new(mov[1][0] - mov[0][0], mov[1][1] - mov[0][1]);
                        }
                        let point = {
                            let x_diff = colliding_line[1][0] - colliding_line[0][0];
                            let y_diff = colliding_line[1][1] - colliding_line[0][1];
                            Vec2::new(
                                colliding_line[0][0] + s * x_diff,
                                colliding_line[0][1] + s * y_diff,
                            )
                        };
                        points_of_collision.push(point);
                        let s_on_end = s == 1.0 || s == 0.0;
                        if !s_on_end || parallel_line(origin, colliding_line) {
                            norms.push(norm_of(colliding_line))
                        } else {
                            corner_corner_norm.push(relative_vel.clone())
                        }
                    }
                }
            }
            prev_static_index = static_index;
        }
        prev_index_for_main = index_for_main;
    }

    let mut total_norm = Vec2::new(0.0, 0.0);
    let norm_to_sum = {
        if norms.len() > 0 {
            norms
        } else if corner_corner_norm.len() > 0 {
            corner_corner_norm
        } else {
            return None;
        }
    };

    for n in norm_to_sum {
        // doing a hack. Since direction shouldnot matter.
        // we make sure they are all the same orientation
        // by a hack.
        let dot = Vec2::dot(*relative_vel, n);
        if dot < 0.0 {
            total_norm -= n
        } else if dot == 0.0 {
            let dot = Vec2::dot(*relative_vel, Vec2::new(n.y, -n.x));
            if dot < 0.0 {
                total_norm -= n
            }
        } else {
            total_norm += n
        }
    }
    let mut avg_pos = Vec2::new(0.0, 0.0);
    for p in points_of_collision.iter() {
        avg_pos += *p;
    }
    avg_pos /= points_of_collision.len() as f64;
    //println!("Norm: {} ; {}; SCALAR: {} ", total_norm.x, total_norm.y, -max_dist);
    return Some((total_norm, max_move, avg_pos));
}

// given a point, rotation_center, move, angle return a Vec2 in the direction
fn corner_vel(point: [f64; 2], center: Vec2, _move: Vec2, angle: f64) -> Vec2 {
    let x = point[0] - center.x;
    let y = point[1] - center.y;
    let sine = angle.sin();
    let cos_minus_one = angle.cos() - 1.0;
    // returns Rw - w + v where w = (x, y), R = rotation matrix, v = velocity
    Vec2::new(
        x * cos_minus_one - y * sine + _move.x,
        x * sine + y * cos_minus_one + _move.y,
    )
}

fn corner_collision_norm(_line1: [[f64; 2]; 2], line2: [[f64; 2]; 2]) -> Vec2 {
    // this could be improved.
    norm_of(line2)
}

// returns true if lines are parallel
fn parallel_line(line1: [[f64; 2]; 2], line2: [[f64; 2]; 2]) -> bool {
    const ERROR_MARGIN: f64 = 0.0001;
    let dx1 = line1[1][0] - line1[0][0];
    let dy1 = line1[1][1] - line1[0][1];
    let dx2 = line2[1][0] - line2[0][0];
    let dy2 = line2[1][1] - line2[0][1];

    if dx1 == 0.0 {
        if dx2 == 0.0 {
            return true;
        } else {
            return false;
        }
    } else if dx2 == 0.0 {
        return false;
    } else {
        let quotient1 = dy1 / dx1;
        let quotient2 = dy2 / dx2;
        return (quotient1.abs() - quotient2.abs()).abs() < ERROR_MARGIN;
    }
}

/// Returns true iff the point lies inside or on the edge of the polyogon.
/// Returns false if vertices contains fewer elements than expected.
pub fn point_in_polygon(point: Vec2, vertices: &Vec<[f64; 2]>) -> bool {
    let ray = [[10000.0 + point.x, point.y], [point.x, point.y]]; // sending the ray in positive direction.

    /* LINE MATH SEEMS BUGGED. incorrectly returns t. */
    let mut sum_of_passes = 0;
    let mut prev_vert_index = vertices.len() - 1;
    for index in 0..vertices.len() {
        let line = [vertices[index], vertices[prev_vert_index]];
        let (t, s) = line_math(ray, line);
        if t <= 1.0 && t >= 0.0 && s >= 0.0 && s <= 1.0 {
            sum_of_passes += 1;
            //println!("SUM_OF_PASSES: {}", sum_of_passes);
            //println!("  ray: {:?}   line {:?} WITH T: {}: S {}", ray, line, t, s)
        }
        prev_vert_index = index;
    }

    return sum_of_passes % 2 == 1;
}

fn make_movement_frame_of_reference_based(
    main_pol_vert: &Vec<[f64; 2]>,
    relative_vel: &Vec2,
    main_angular_speed: f64,
    main_mass_center: Vec2,
    static_angular_speed: f64,
    static_mass_center: Vec2,
) -> Vec<Vec2> {
    // We are creating a frame_of_reference from static one.
    // translate so that the static_mass_center is origin.
    let mut main_clone = main_pol_vert.clone();

    for vert in main_clone.iter_mut() {
        vert[0] -= static_mass_center.x;
        vert[1] -= static_mass_center.y;
    }

    // rotate them back as if static is viewing the world rotate
    let _sin = static_angular_speed.sin();
    let _cos = static_angular_speed.cos();
    for vert in main_clone.iter_mut() {
        let prev = vert.clone();
        vert[0] = prev[0] * _cos - prev[1] * _sin;
        vert[1] = prev[0] * _sin + prev[1] * _cos;
    }

    // translate back
    for vert in main_clone.iter_mut() {
        vert[0] += static_mass_center.x;
        vert[1] += static_mass_center.y;
    }

    // rotate them around the mass_center
    rotate_vertices(
        main_mass_center,
        &mut main_clone,
        -main_angular_speed,
        &mut Vec2::new(0.0, 0.0),
    );
    let mut movement = vec![];
    for i in 0..main_clone.len() {
        let x_diff = main_clone[i][0] - main_pol_vert[i][0] - relative_vel.x;
        let y_diff = main_clone[i][1] - main_pol_vert[i][1] - relative_vel.y;
        movement.push(Vec2::new(x_diff, y_diff));
    }
    movement
}
