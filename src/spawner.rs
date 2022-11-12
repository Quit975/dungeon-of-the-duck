use crate::prelude::*;

pub fn spawn_player(ecs : &mut World, pos : Point) {
    ecs.push(
        (
            Player, 
            pos, 
            Render {
                color : ColorPair::new(WHITE, BLACK),
                glyph : to_cp437('@')
            }
        )
    );
}

pub fn spawn_monster(ecs : &mut World, enemy_type : EnemyType, pos : Point) {
    ecs.push(
        (
            Enemy,
            pos,
            Render {
                color : ColorPair::new(RED, BLACK),
                glyph : to_cp437(match enemy_type {
                    EnemyType::Bat => '^',
                    EnemyType::Goblin => 'o',
                    EnemyType::Orc => '0',
                    EnemyType::Troll => 'X'
                })
            },
            enemy_type,
            RandomMovement
        )
    );
}

pub fn spawn_random_monster(ecs : &mut World, rng : &mut RandomNumberGenerator, pos : Point) {
    match rng.range(0, 4) {
        0 => spawn_monster(ecs, EnemyType::Bat, pos),
        1 => spawn_monster(ecs, EnemyType::Goblin, pos),
        2 => spawn_monster(ecs, EnemyType::Orc, pos),
        _ => spawn_monster(ecs, EnemyType::Troll, pos),
    }
}