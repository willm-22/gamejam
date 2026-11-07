use bevy::prelude::*;
use crate::player;
use crate::definitions;

pub fn draw_level(mut commands: Commands) {
    let rows: Vec<&str> = definitions::LEVEL.lines().collect();
    let height = rows.len() as f32;
    let width = rows[0].len() as f32;


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
                'P' => player::spawn_player(pos.x, pos.y, &mut commands, Color::srgb(0.3, 0.8, 0.4)),
                'G' => draw_tile(&mut commands, pos, Color::srgb(0.3, 0.3, 0.4), definitions::Collision {action: definitions::CollisionAction::Goal}),
                'X' => draw_tile(&mut commands, pos, Color::srgb(0.3, 0.3, 0.4), definitions::Collision {action: definitions::CollisionAction::Death}),
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
    ));
}