use bevy::prelude::*;

const LEVEL: &str = include_str!("../assets/level1.txt");

const TILE: f32 = 32.0;


#[derive(Component)]
struct Wall;

#[derive(Component)]
struct Player;

#[derive(Component)]
struct LevelEntity;

fn draw_level() {
    let height = rows.len() as f32;
    let width = rows[0].len() as f32;
    let rows: Vec<&str> = LEVEL.lines().collect();

}