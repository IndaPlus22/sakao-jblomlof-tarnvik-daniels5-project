use nalgebra::{Matrix2, Vector2};

use crate::vector::vector::Vec2;

/// If a polygon collides with the other polygon, this returns Some((normal, t)).
/// Such that t*self.velocity is the relative movement such that the polygons only touches.
pub fn collision_between_polygons(
    main_pol_vert: &Vec<[f64; 2]>,
    relative_velocity: &Vec2,
    static_pol_vert: &Vec<[f64; 2]>,
) -> Option<f64> {
    if relative_velocity.squared_length() == 0.0 {
        return None
    }
    // if any line intersects a line in the other, move main back until it only touches. If the next line is on the other side of the toucing line, repeat.
    let mut colliding = false;
    let mut prev_main_index = main_pol_vert.len() - 1;
    let mut main_index = 0;

    // The line from [furthest - 1] to [furthest] just got shifted repeat check for the next segment if the endpoints are on different sides.
    let mut furthest_index = 0;
    let mut furthest_dist = 0.0;
    while main_index < main_pol_vert.len() {

        let mut prev_static_index = static_pol_vert.len() - 1;
        let mut static_index = 0;
        while static_index < static_pol_vert.len() {
            let line1 = [main_pol_vert[prev_main_index], main_pol_vert[main_index]];
            let line2 = [static_pol_vert[prev_static_index], static_pol_vert[static_index]];
            match line_parallel(line1, line2) {
                Some(sol) => {println!("Implement here! collision.rs line 31-ish")}
                None => {
                    // they are not parallell
                    match line_intersecting(line1, line2) {
                        Some(sol) => {
                            colliding = true;
                            let mut dist: f64 = 0.0;
                            let t = sol.0;
                            if t > 0.0 {
                                // check distance needed to move out the first point. (the distance to move it to the intersecting line)
                                dist = dist.max(distance_needed_to_move(&(-(*relative_velocity)), line1, line2));
                            }
                            if t < 1.0 {
                                dist = dist.max(distance_needed_to_move(&(-(*relative_velocity)), [line1[1], line1[0]], line2));
                            }
                            if dist > furthest_dist {
                                furthest_dist = dist;
                                furthest_index = main_index
                            }
                        }
                        None => () // not intersecting
                    }
                }
            }

            prev_static_index = static_index;
            static_index += 1;
        }

        prev_main_index = main_index;
        main_index += 1;
    }
    if colliding {
        Some(furthest_dist)
    } else {
        None
    }
}

///check if two lines intersect each other returns the t and s for the lines such that t*vec_from_point0 + point0 is the collision and same for s.
/// # Panics
/// Panics if the two lines are parallel or either line is actually a point
fn line_intersecting(line1: [[f64; 2]; 2], line2: [[f64; 2]; 2]) -> Option<(f64, f64)> {
    let (t,s) = line_math(line1, line2);
    if s <= 1.0 && s >= 0.0 && t <= 1.0 && t >= 0.0 {
        Some((t, s))
    } else {
    None
    }
}

/// returns t, s
/// # Panic
/// panics if lines are parallell
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
        Some(res) => {return (res[0], res[1]);}
        None => {
            println!("{:?} ---- {:?}", matrix, vector);
        return (0.0,0.0)}
    }
    /*let result = decomp.solve(&vector).unwrap();
    
    (result[0], result[1])*/
}

/// Returns the Some(t0, tmax) such the line from point0 in the vector from point0 to point1 t0<=t<=tmax is the intersecting
/// First option signals if they are parallell, and the second one indicates if they are overlapping
fn line_parallel(line1: [[f64; 2]; 2], line2: [[f64; 2]; 2]) -> Option<Option<(f64,f64)>> {
    const ERROR_MARGIN: f64 = 0.001;
    let vec1 = Vec2::new(line1[1][0] - line1[0][0], line1[1][1] - line1[0][1]);
    let vec2 = Vec2::new(line2[1][0] - line2[0][0], line2[1][1] - line2[0][1]);
    if (vec1.x == 0.0 && vec1.y == 0.0) || (vec2.x == 0.0 && vec2.y == 0.0) {
        panic!("WEIRD");
    }
    if vec2.x == 0.0 {
        if vec1.x == 0.0 {
            return Some(None);
        } else {
            return None;
        }
    }
    let t = vec1.x /vec2.x;
    if (t * vec2.y - vec1.y).abs() < ERROR_MARGIN {
        return Some(None);
    }
    None
}


// THIS NEEDS FIX
fn distance_needed_to_move(negative_velocity: &Vec2, line1: [[f64;2];2], line2: [[f64; 2]; 2]) -> f64 {
    let point = line1[0];
    let move_line = [point, [point[0] + negative_velocity.x, point[1] + negative_velocity.y]];
    match line_parallel(move_line, line2) {
        Some(overlap) => {
            return 0.0;
        }
        None => ()
    }
    let (t,s) = line_math(move_line, line2);
    if t < 0.0 {
        // wrong direction
        return 0.0;
    }
    else if s <= 1.0 && s >= 0.0 {
        // moving in the right direction, and it will move just right
        return t;
    } else if s > 1.0 {
        // we are overshooting, reverse perspective
        let temp = distance_needed_to_move(&(-(*negative_velocity)), [line2[1], line2[0]], line1);
        return temp
    } else if s < 0.0 {
        // we are overshooting, but a different side.
        let temp = distance_needed_to_move(&(-(*negative_velocity)), line2, line1);
        return temp
    }
    // we should never reach here
    panic!();
}
