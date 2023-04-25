
// use std::thread::sleep;

// TODO: make it
use piston::UpdateArgs;

use super::Variables;
use super::traits::collisionRecord;

pub fn update(update_args: UpdateArgs, variables: &mut Variables) {
    // TODO 
    // collisions
    // using collisionsREcord
    let mut list_of_collisions: Vec<Option<collisionRecord>> = Vec::with_capacity(variables.objects.len());
    for (outer_i, obj) in variables.objects.iter().enumerate() {
        //temporrary
        let mut collision_rec: Option<collisionRecord> = None;
        for (inner_i, other) in variables.objects.iter().enumerate() {
            if outer_i == inner_i {
                continue;
            }
            
            collision_rec = obj.collisions(other, collision_rec);
            //TODO
            // * Add
            // PsuedO: collisionRec = obj.collision(other, collisionRec)
        }
        list_of_collisions.push(collision_rec)
    }
    for (index, obj) in variables.objects.iter_mut().enumerate() {
        obj.update(&list_of_collisions[index], update_args.dt);
    }
}