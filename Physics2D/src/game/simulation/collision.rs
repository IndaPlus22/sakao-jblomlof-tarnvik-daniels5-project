use nalgebra::{Matrix2, Vector2};

use crate::vector::vector::Vec2;

/// Returns whatever the polygons approximate circles collide.
pub fn approx_are_colliding(centre1: Vec2, raduis1: f64, centre2: Vec2, radius2: f64) -> bool {
    let distance = (centre1 - centre2).squared_length();
    return distance < (raduis1 + radius2) * (raduis1 + radius2);
    // if distance^2 < rad_dist^2 so is dist < rad_dist
    // since distance is >= 0, rad >= 0;
}

/// If a polygon collides with the other polygon, this returns Some((normal, t)).
/// Such that t*self.velocity is the relative movement such that the polygons only touches.
pub fn collision_between_polygons(
    main_pol_vert: &Vec<[f64; 2]>,
    relative_movement: &Vec2,
    static_pol_vert: &Vec<[f64; 2]>,
) -> Option<(Vec2, f64)> {
    match type_of_collision(main_pol_vert, static_pol_vert, relative_movement) {
        CollisionType::No => return None,
        CollisionType::Touching(norm) => return Some((norm, 0.0)),
        CollisionType::Interference => {
            if relative_movement.squared_length() != 0.0 {
                return Some(calculate_scalar_distance(
                    main_pol_vert,
                    static_pol_vert,
                    relative_movement,
                ));
            } else {
                return None;
            }
        }
    }
}

fn type_of_collision(
    main_pol_vert: &Vec<[f64; 2]>,
    static_pol_vert: &Vec<[f64; 2]>,
    relative_movement: &Vec2,
) -> CollisionType {
    let mut norms: Vec<Vec2> = Vec::new();
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

            let (t, s) = line_math(line1, line2);
            if t <= 1.0 && t >= 0.0 && s <= 1.0 && s >= 0.0 {
                let t_on_end = t == 1.0 || t == 0.0;
                let s_on_end = s == 1.0 || s == 0.0;
                match (t_on_end, s_on_end) {
                    (false, false) => return CollisionType::Interference,
                    (true, false) => {
                        norms.push(norm_of(line2));
                    }
                    (false, true) => {
                        norms.push(norm_of(line1));
                    }
                    (true, true) => {
                        // improve norm calculation for corner-corner-collision.
                        some_contact = true;
                    }
                }
            }

            prev_static_index = static_index;
        }
        prev_index_for_main = index_for_main;
    }

    if some_contact {
        let mut total_norm = Vec2::new(0.0, 0.0);
        if norms.len() == 0 {
            total_norm = relative_movement.clone();
        } else {
            for n in norms {
                total_norm += n;
            }
        }
        //total_norm /= total_norm.length();
        return CollisionType::Touching(total_norm);
    } else {
        return CollisionType::No;
    }
}

fn norm_of(line1: [[f64; 2]; 2]) -> Vec2 {
    // assuming all vertex is ordered counter-clockwise it is
    return Vec2::new(line1[1][1] - line1[0][1], line1[0][0] - line1[1][0]);
}
enum CollisionType {
    Interference,
    Touching(Vec2),
    No,
}

/// returns t, s
/// returns -0.01, -0.01 if line are parallell (garbage)
fn line_math(line1: [[f64; 2]; 2], line2: [[f64; 2]; 2]) -> (f64, f64) {
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
    relative_movement: &Vec2,
) -> (Vec2, f64) {
    let mut norms: Vec<Vec2> = Vec::new();
    let mut corner_corner_norm: Vec<Vec2> = Vec::new(); // corner-corner-only
    let neg_vel = -*relative_movement;
    let mut max_dist: f64 = 0.0;

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

            let move_1 = [line1[0], [line1[0][0] + neg_vel.x, line1[0][1] + neg_vel.y]];
            let move_2 = [line1[1], [line1[1][0] + neg_vel.x, line1[1][1] + neg_vel.y]];
            let move_3 = [line2[0], [line2[0][0] - neg_vel.x, line2[0][1] - neg_vel.y]];
            let move_4 = [line2[1], [line2[1][0] - neg_vel.x, line2[1][1] - neg_vel.y]];

            // Each element is (vec_from_base_line"+"relative_vel, line_to_check_collision, origin_line)
            let compound_lines = [
                (move_1, line2, line1),
                (move_2, line2, line1),
                (move_3, line1, line2),
                (move_4, line1, line2),
            ];

            for (mov, colliding_line, origin) in compound_lines {
                let (t, s) = line_math(mov, colliding_line);
                if t > 0.0 && s <= 1.0 && s >= 0.0 {
                    if t >= max_dist {
                        if t > max_dist {
                            norms.clear();
                            corner_corner_norm.clear();
                            max_dist = t;
                        }
                        let s_on_end = s == 1.0 || s == 0.0;
                        if !s_on_end || parallel_line(origin, colliding_line) {
                            norms.push(norm_of(colliding_line))
                        } else {
                            corner_corner_norm.push(corner_collision_norm(origin, colliding_line))
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
            vec![relative_movement.clone()]
        }
    };

    for n in norm_to_sum {
        total_norm += n;
    }

    return (total_norm, -max_dist);
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