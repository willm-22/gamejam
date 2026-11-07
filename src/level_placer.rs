use bevy::prelude::*;

const LEVEL: &str = include_str!("../assets/level1.txt");

const TILE: f32 = 32.0;

#[derive(Component)]
struct Wall;

#[derive(Component)]
struct Player;

#[derive(Component)]
struct LevelEntity;

pub fn draw_level(mut commands: Commands) {
    let rows: Vec<&str> = LEVEL.lines().collect();

    let height = rows.len() as f32;
    let width = rows[0].len() as f32;


    let origin = Vec2::new(-width * TILE / 2.0, height * TILE / 2.0);

    for (y, row) in rows.iter().enumerate() {
        for (x, ch) in row.chars().enumerate() {
            let pos = Vec3::new(
                origin.x + x as f32 * TILE + TILE / 2.0,
                origin.y - y as f32 * TILE - TILE / 2.0,
                0.0,
            );

            match ch {
                '#' => draw_tile(&mut commands, pos, Color::srgb(0.3, 0.3, 0.4), Wall),
                'P' => draw_tile(&mut commands, pos.with_z(1.0), Color::srgb(0.3, 0.8, 0.4), Player),
                _ => {}
            }
        }
    }
}

fn draw_tile(commands: &mut Commands, pos: Vec3, color: Color, marker: impl Component) {
    commands.spawn((
        marker,
        LevelEntity,
        Sprite::from_color(color, Vec2::splat(TILE)),
        Transform::from_translation(pos),
    ));
}