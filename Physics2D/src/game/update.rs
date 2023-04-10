
// TODO: make it
use piston::{UpdateArgs, Event, *};

use super::Variables;
use super::traits::collisionRecord;

pub fn update(update_args: UpdateArgs, variables: &mut Variables) {

    // TODO 
    // collisions
    // using collisionsREcord
    let mut list_of_collisions: Vec<Option<collisionRecord>> = Vec::with_capacity(variables.objects.len());
    for (outerI, obj) in variables.objects.iter().enumerate() {
        //temporrary
        let mut collisionRec: Option<collisionRecord> = None;
        for (innerI, other) in variables.objects.iter().enumerate() {
            if outerI == innerI {
                continue;
            }

            collisionRec = obj.collisions(other, collisionRec);
            //TODO
            // * Add
            // PsuedO: collisionRec = obj.collision(other, collisionRec)
        }
        list_of_collisions.push(collisionRec)
    }

    for obj in &mut variables.objects {
        obj.update();
    }
}