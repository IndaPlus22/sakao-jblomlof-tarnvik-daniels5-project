// use std::thread::sleep;

// TODO: make it
use piston::UpdateArgs;

use crate::vector::vector::Vec2;

use super::simulation::collision::resolve_interfers_simple;
use super::traits::collisionRecord;
use super::Variables;
const AMOUNT_OF_ITERATION: usize = 2;
pub fn update(update_args: UpdateArgs, variables: &mut Variables) {
    let mut list_of_collisions: Vec<Option<collisionRecord>> =
        Vec::with_capacity(variables.objects.len());
    for (outer_i, obj) in variables.objects.iter().enumerate() {
        //temporrary
        let mut collision_rec: Option<collisionRecord> = None;
        for (inner_i, other) in variables.objects.iter().enumerate() {
            if outer_i == inner_i {
                continue;
            }

            collision_rec = obj.collisions(other, collision_rec);
        }
        list_of_collisions.push(collision_rec)
    }
    for (index, obj) in variables.objects.iter_mut().enumerate() {
        obj.update(&list_of_collisions[index], update_args.dt);
    }

    /*  Resolve interfers */
    for _ in 0..AMOUNT_OF_ITERATION {
        let mut list_of_resolves = vec![];
        for (outer_i, obj) in variables.objects.iter().enumerate() {
            //temporrary
            let mut resolve_interfers: Option<Vec2> = None;
            for (inner_i, other) in variables.objects.iter().enumerate() {
                if outer_i == inner_i {
                    continue;
                }

                resolve_interfers = resolve_interfers_simple(obj, other, resolve_interfers);
            }
            list_of_resolves.push(resolve_interfers)
        }
        for (index, obj) in variables.objects.iter_mut().enumerate() {
            match list_of_resolves[index] {
                Some(val) => {
                    obj.moverelative(val)
                }
                None => {}
            }
        }
    }
}
