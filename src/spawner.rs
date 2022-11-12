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

pub fn spawn_monster (ecs : &mut World, pos : Point) {
    ecs.push(
        (
            Enemy,
            pos,
            Render {
                color : ColorPair::new(RED, BLACK),
                glyph : to_cp437('X')
            }
        )
    );
}