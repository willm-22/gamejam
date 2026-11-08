use bevy::prelude::*;
use crate::player;
use crate::definitions;

pub fn draw_level(
    commands: &mut Commands,
    asset_server: &AssetServer,
    layouts: &mut Assets<TextureAtlasLayout>,
    level: &str,
) {
    let rows: Vec<&str> = level.lines().collect();
    let height = rows.len() as f32;
    let width = rows.iter().map(|r| r.chars().count()).max().unwrap_or(0) as f32;

    let origin = Vec2::new(-width * definitions::TILE / 2.0, height * definitions::TILE / 2.0);

    for (y, row) in rows.iter().enumerate() {
        for (x, ch) in row.chars().enumerate() {
            let pos = Vec3::new(
                origin.x + x as f32 * definitions::TILE + definitions::TILE / 2.0,
                origin.y - y as f32 * definitions::TILE - definitions::TILE / 2.0,
                0.0,
            );

            match ch {
                '#' => draw_tile(&mut commands, pos, Color::srgb(0.3, 0.3, 0.4), definitions::Collision {action: definitions::CollisionAction::Wall}),
                'P' => player::spawn_player(pos.x, pos.y, &mut commands, &asset_server, &mut layouts),
                'G' => draw_tile(&mut commands, pos, Color::srgb(0.3, 0.3, 0.4), definitions::Collision {action: definitions::CollisionAction::Goal}),
                'X' => draw_tile(&mut commands, pos, Color::srgb(0.3, 0.3, 0.4), definitions::Collision {action: definitions::CollisionAction::Death}),
                'T' => draw_tile(&mut commands, pos, Color::srgb(0.8, 0.8, 0.2), definitions::Collision {action: definitions::CollisionAction::Top}),
                _ => {}
            }
        }
    }
}

fn draw_tile(commands: &mut Commands, pos: Vec3, color: Color, marker: impl Component) {
    commands.spawn((
        marker,
        definitions::LevelEntity,
        Sprite::from_color(color, Vec2::splat(definitions::TILE)),
        Transform::from_translation(pos),
        definitions::Size { width: 32.0, height: 32.0 },
    ));
}

pub fn make_level(
    commands: &mut Commands,
    asset_server: &AssetServer,
    layouts: &mut Assets<TextureAtlasLayout>,
    level: i32,
) {
    let level_path = format!("assets/levels/level{}.txt", level);
    let level_string = read_string(&level_path);
    draw_level(commands, asset_server, layouts, &level_string);
}

fn read_string(level_path: &str) -> String {
    std::fs::read_to_string(level_path)
        .unwrap_or_else(|e| panic!("couldn't read level file {}: {}", level_path, e))
}