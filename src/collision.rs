/* use bevy::prelude::*;

use crate::definitions;

pub enum Stop {Sideways, Verticle, None}
//very much based on https://bevy.org/examples/games/breakout/
pub fn check_collision(old_x: f32, old_y: f32, vel_x: f32, vel_y: f32,
    player_transformer: &mut Transform, player_size: &definitions::Size, player: &mut definitions::Player,
    collision_all_sides: Query<(&mut Transform, &definitions::Size, &definitions::Collision), Without<definitions::Player>>,
    ) -> Stop {
        // let (mut player_transformer, velocity, player_size) = player.into_inner();

        let new_x = old_x + vel_x;
        let new_y = old_y + vel_y;
        let p_half_width = player_size.width / 2.0;
        let p_half_height = player_size.height / 2.0;

        let po_left_side = player_transformer.translation.x - p_half_width;
        let po_right_side = player_transformer.translation.x + p_half_width;
        let po_top_side = player_transformer.translation.y + p_half_height;
        let po_bottom_side = player_transformer.translation.y - p_half_height;
        
        let pn_left_side = po_left_side + vel_x - p_half_width;
        let pn_right_side = po_right_side + vel_x + p_half_width;
        let pn_top_side = po_top_side + vel_y + p_half_height;
        let pn_bottom_side = po_bottom_side + vel_y - p_half_height;

        for (collider_location, collider_size, collider) in &collision_all_sides {
            let c_half_height = collider_size.height / 2.0;
            let c_half_width = collider_size.width / 2.0;

            let c_left_side = collider_location.translation.x - c_half_width;
            let c_right_side = collider_location.translation.x + c_half_width;
            let c_top_side = collider_location.translation.y + c_half_height;
            let c_bottom_side = collider_location.translation.y - c_half_height;

            let collide_bottom: bool = !check_between(c_bottom_side, c_top_side, po_top_side) && check_between(c_bottom_side, c_top_side, pn_top_side) && (check_between(c_left_side, c_right_side, pn_right_side) | check_between(c_left_side, c_right_side, pn_left_side));
            let collide_top: bool = !check_between(c_bottom_side, c_top_side, po_bottom_side) && check_between(c_bottom_side, c_top_side, pn_bottom_side) && (check_between(c_left_side, c_right_side, pn_right_side) | check_between(c_left_side, c_right_side, pn_left_side));
            
            let collide_left: bool = !check_between(c_left_side, c_right_side, po_right_side) && (check_between(c_left_side, c_right_side, pn_right_side) && check_between(c_bottom_side, c_top_side, pn_top_side) | check_between(c_bottom_side, c_top_side, pn_bottom_side)) & !(collide_top | collide_bottom);
            let collide_right: bool = !check_between(c_left_side, c_right_side, po_left_side) && (check_between(c_left_side, c_right_side, pn_left_side) && check_between(c_bottom_side, c_top_side, pn_top_side) | check_between(c_bottom_side, c_top_side, pn_bottom_side)) & !(collide_top | collide_bottom);
            player.on_ground = collide_top;
            if collide_left | collide_right | collide_top | collide_bottom {
                match collider.action {
                    definitions::CollisionAction::Move => {},
                    definitions::CollisionAction::Death => {println!("Ow");},
                    definitions::CollisionAction::Goal => {println!("YAY")},
                    definitions::CollisionAction::Wall => {
                        if (collide_top && vel_y < 0.0) {
                            println!("{}", vel_y);
                            player_transformer.translation.y = c_top_side + p_half_height;
                            return Stop::Verticle;
                        }
                        if collide_bottom && vel_y > 0.0 {
                            return Stop::Verticle;
                        }
                        if collide_right && vel_x < 0.0 {
                            return Stop::Sideways;
                        }
                        if collide_left && vel_x > 0.0 {
                            return Stop::Sideways;
                        }
                    },
                    definitions::CollisionAction::Top | definitions::CollisionAction::Move => {
                        if collide_top && vel_y < 0.0 {
                            player_transformer.translation.y = c_top_side + p_half_height;
                            return Stop::Verticle;
                        } else {
                            return Stop::None;
                        }
                    }
                    _ => {}
            
                }
            
            }
        }
        return Stop::None;

    }
    

    fn check_between(lower: f32, higher: f32, value: f32) -> bool {
        return (lower < value && value < higher);
    }

    */
use bevy::prelude::*;

use crate::definitions;

/// What the player ran into during one call to `move_and_collide`.
#[derive(Default)]
pub struct Hit {
    pub x: bool,      // blocked sideways by a wall
    pub y: bool,      // blocked vertically (floor or ceiling)
    pub ground: bool, // landed on top of a wall
    pub died: bool,   // touching a Death tile
    pub goal: bool,   // touching a Goal tile
}

// Tiny tolerance so floating point error never turns "touching" into "overlapping".
const EPS: f32 = 0.01;

fn overlaps(
    a_pos: Vec3,
    a: &definitions::Size,
    b_pos: Vec3,
    b: &definitions::Size,
    eps: f32,
) -> bool {
    (a_pos.x - b_pos.x).abs() < (a.width + b.width) / 2.0 - eps
        && (a_pos.y - b_pos.y).abs() < (a.height + b.height) / 2.0 - eps
}

fn is_wall(c: &definitions::Collision) -> bool {
    matches!(c.action, definitions::CollisionAction::Wall)
}

/// Moves the player by `step`, one axis at a time, pushing it out of any wall it ends up inside.
/// Call this several times per frame with small steps to avoid tunnelling at high speed.
pub fn move_and_collide(
    transform: &mut Transform,
    size: &definitions::Size,
    step: Vec2,
    colliders: &Query<
        (&Transform, &definitions::Size, &definitions::Collision),
        Without<definitions::Player>,
    >,
) -> Hit {
    let mut hit = Hit::default();

    // ---- X axis ----
    transform.translation.x += step.x;
    for (c_t, c_size, c) in colliders.iter() {
        if is_wall(c) && overlaps(transform.translation, size, c_t.translation, c_size, EPS) {
            let gap = (size.width + c_size.width) / 2.0;
            if step.x > 0.0 {
                transform.translation.x = c_t.translation.x - gap; // hit the wall's left side
                hit.x = true;
            } else if step.x < 0.0 {
                transform.translation.x = c_t.translation.x + gap; // hit the wall's right side
                hit.x = true;
            }
        }
    }

    // ---- Y axis ----
    transform.translation.y += step.y;
    for (c_t, c_size, c) in colliders.iter() {
        if is_wall(c) && overlaps(transform.translation, size, c_t.translation, c_size, EPS) {
            let gap = (size.height + c_size.height) / 2.0;
            if step.y < 0.0 {
                transform.translation.y = c_t.translation.y + gap; // landed on top
                hit.y = true;
                hit.ground = true;
            } else if step.y > 0.0 {
                transform.translation.y = c_t.translation.y - gap; // bumped head
                hit.y = true;
            }
        }
    }

    // ---- Non-solid tiles (nothing is pushed out of these) ----
    for (c_t, c_size, c) in colliders.iter() {
        if overlaps(transform.translation, size, c_t.translation, c_size, 0.0) {
            match c.action {
                definitions::CollisionAction::Death => hit.died = true,
                definitions::CollisionAction::Goal => hit.goal = true,
                _ => {}
            }
        }
    }

    hit
}
 