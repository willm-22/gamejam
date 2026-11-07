use bevy::prelude::*;

use crate::definitions;

enum side {LEFT, TOP, RIGHT, BOTTOM}

//very much based on https://bevy.org/examples/games/breakout/
pub fn check_collision(old_x: f32, old_y: f32, vel_x: f32, vel_y: f32,
    player: Single<(&mut Transform, definitions::Size), With<definitions::Player>>,
    collision_all_sides: Query<(&mut Transform, definitions::Size), With<definitions::Collision>>,
    collision_top: Query<(&mut Transform, definitions::Size), With<definitions::CollisionTop>>) {
        let (mut player_pos, player_size) = player.get_inner();
        let new_x = old_x + vel_x;
        let new_y = new_y + vel_y;
        let p_half_width = player_size.width / 2.0;
        let p_half_height = player_size.height / 2.0;

        let p_left_side = player_pos.x - p_half_width;
        let p_right_side = player_pos.x + p_half_width;
        let p_top_side = player_pos.y + p_half_height;
        let p_bottom_side = player_pos.y - p_half_height;

        for (collider_location, collider_size) in &collision_all_sides {
            let c_half_height = collider_size.height / 2.0;
            let c_half_width = collider_size.width / 2.0;

            let c_left_side = collider_location.x - c_half_width;
            let c_right_side = collider_location.x + c_half_width;
            let c_top_side = collider_location.y + c_half_height;
            let c_bottom_side = collider_location.y - c_half_height;

            let hit_right = p_right_side > c_left_side;
            let hit_left =  c_right_side > p_left_side;
            let hit_bottom = p_top_side > c_bottom_side;
            let hit_top = c_top_side > p_bottom_side;
            let bool_collision = (hit_right | hit_left) && ( hit_bottom | hit_top);
            if bool_collision {
                pass
                // in theory
                // 
            }
        }

    }
    
