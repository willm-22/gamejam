use bevy::prelude::*;
use crate::definitions;

const ZERO: Transform = Transform::from_translation(Vec3::new(0.0, 0.0, 0.0));

pub fn grapple(
    input: Res<ButtonInput<KeyCode>>,
    player: Single<(&Transform, &mut definitions::Velocity), With<definitions::Player>>,
    rectange: Query<(&definitions::Collision, &Transform) , Without<definitions::Player>>) {
    if !input.pressed(KeyCode::KeyE) { return;}

    let mut closest_pos: &Transform = &ZERO;
    let mut closest: f32 = f32::MAX;
    let (player_trans, mut velocity) = player.into_inner();
    let p_x = player_trans.translation.x;
    let p_y = player_trans.translation.y;
    // decide closest
    for (collide, trans) in &rectange {
        let distance = calc_dis_square(p_x, p_y, trans.translation.x, trans.translation.y);
        if distance < closest && distance < 700.0 {closest = distance; closest_pos = trans}
    }

    if closest_pos == &ZERO {return;}

    let x_diff = p_x - closest_pos.translation.x;
    let y_diff = p_y - closest_pos.translation.y;
    velocity.x -= x_diff * 0.5;
    velocity.y -= y_diff * 0.5;
}

fn calc_dis_square(x1: f32, y1:f32, x2: f32, y2: f32) -> f32 {
    let x_dif = x1 - x2;
    let y_dif = y1 - y2;
    return x_dif * x_dif + y_dif * y_dif;
}