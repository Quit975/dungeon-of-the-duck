pub use crate::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Render {
    pub color : ColorPair,
    pub glyph : FontCharType,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Player;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Enemy;

pub enum EnemyType {
    Goblin,
    Orc,
    Troll,
    Bat
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RandomMovement;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MovementIntent {
    pub entity : Entity,
    pub destination : Point
}