use bevy::prelude::*;
pub const LEVEL: &str = include_str!("../assets/level1.txt");

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
pub struct Velocity {pub x: f32, pub y : f32}
impl Velocity {
    pub fn init(&mut self, x: f32, y: f32) {
        self.x = x;
        self.y = y;
    }

    pub fn new(x: f32, y: f32) -> Velocity {
        Velocity {x, y}
    }

    pub fn mult(&mut self, num: f32) {
        self.x = self.x * num;
    }

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

#[derive(Component)]
pub struct Size {
    pub width: f32,
    pub height: f32
}

pub const PLAYER_SIZE: Size = Size {width: 20.0, height: 20.0};