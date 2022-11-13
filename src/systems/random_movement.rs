use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(RandomMovement)]
pub fn random_movement(
    ecs : &mut SubWorld, 
    commands : &mut CommandBuffer
) {
    let mut moving_entities = 
        <(Entity, &Point, &RandomMovement)>::query();
    
    moving_entities.iter(ecs)
    .for_each(|(entity, pos, _)| {
        let mut rng = RandomNumberGenerator::new();
        let destination = match rng.range(0, 4) {
            0 => Point::new(-1, 0),
            1 => Point::new(1, 0),
            2 => Point::new(0, -1),
            _ => Point::new(0, 1),
        } + *pos;

        commands.push(((), MovementIntent{ entity : *entity, destination}));
    });
}