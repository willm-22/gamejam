use bevy::prelude::*;
pub const LEVEL: &str = include_str!("../assets/level1.txt");

pub const TILE: f32 = 32.0;

#[derive(Component)]
pub struct CollisionTop;

#[derive(Component)]
pub struct Collision;

#[derive(Component)]
pub struct NoCollision;

#[derive(Component)]
pub struct LevelEntity;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Size {
    pub width: f32,
    pub height: f32
}

pub const PLAYER_SIZE: Size = Size {width: 20.0, height: 60.0};