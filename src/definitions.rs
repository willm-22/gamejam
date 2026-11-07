use bevy::prelude::*;
pub const LEVEL: &str = include_str!("../assets/level1.txt");
pub const PLAYER_WIDTH: f32 = 30.0;
pub const PLAYER_HEIGHT: f32 = 60.0;
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