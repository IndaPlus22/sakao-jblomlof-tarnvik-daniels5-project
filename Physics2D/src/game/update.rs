use piston::UpdateArgs;

use super::Variables;
use super::traits::collisionRecord;

pub fn update(update_args: UpdateArgs, variables: &mut Variables) {

    // TODO 
    // collisions
    // using collisionsREcord
    let mut list_of_collisions: Vec<Option<collisionRecord>> = Vec::with_capacity(variables.objects.len());
    for (outerI, obj) in variables.objects.iter().enumerate() {
        //temporrary
        let mut collisionsRec: Option<collisionRecord> = None;
        for (innerI, other) in variables.objects.iter().enumerate() {
            if outerI == innerI {
                continue;
            }
            //TODO
            // * Add
            // PsuedO: collisionRec = obj.collision(other, collisionRec)
        }
        list_of_collisions.push(collisionRec)
    }

    variables.objects.iter().map(|x| x.update());

    
}