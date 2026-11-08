use bevy::prelude::*;
use crate::definitions;

pub fn follow_player(
    player: Single<&Transform, (With<definitions::Player>, Without<Camera2d>)>,
    mut camera: Single<&mut Transform, (With<Camera2d>, Without<definitions::Player>)>,
) {
    camera.translation.x = player.translation.x;
    camera.translation.y = player.translation.y;
}