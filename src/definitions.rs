use bevy::prelude::*;
pub const LEVEL: &str = include_str!("../assets/level1.txt");
pub const PLAYER_WIDTH: f32 = 30.0;
pub const PLAYER_HEIGHT: f32 = 60.0;
pub const TILE: f32 = 32.0;

pub enum CollisionAction {
    Wall,
    Death,
    Goal,
    Nothing,
}

#[derive(Component)]
pub struct CollisionTop {
    pub action: CollisionAction,
}

#[derive(Component)]
pub struct Collision {
    pub action: CollisionAction,
}

#[derive(Component)]
pub struct NoCollision;

#[derive(Component)]
pub struct LevelEntity;

#[derive(Component)]
pub struct Player;