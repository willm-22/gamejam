use bevy::prelude::*;

use crate::definitions;

pub enum Stop {Sideways, Verticle, None}
//very much based on https://bevy.org/examples/games/breakout/
pub fn check_collision(old_x: f32, old_y: f32, vel_x: f32, vel_y: f32,
    player_transformer: &mut Transform, player_size: &definitions::Size, player: &mut definitions::Player,
    collision_all_sides: &Query<(&Transform, &definitions::Size, &definitions::Collision), Without<definitions::Player>>,
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
        
        let pn_left_side = po_left_side + vel_x;
        let pn_right_side = po_right_side + vel_x;
        let pn_top_side = po_top_side + vel_y;
        let pn_bottom_side = po_bottom_side + vel_y;

        for (collider_location, collider_size, collider) in collision_all_sides {
            let c_half_height = collider_size.height / 2.0;
            let c_half_width = collider_size.width / 2.0;

            let c_left_side = collider_location.translation.x - c_half_width;
            let c_right_side = collider_location.translation.x + c_half_width;
            let c_top_side = collider_location.translation.y + c_half_height;
            let c_bottom_side = collider_location.translation.y - c_half_height;

            let collide_bottom: bool = !check_between(c_bottom_side, c_top_side, po_top_side) && check_between(c_bottom_side, c_top_side, pn_top_side) && (check_between(c_left_side, c_right_side, pn_right_side) | check_between(c_left_side, c_right_side, pn_left_side));
            let collide_top: bool = !check_between(c_bottom_side, c_top_side, po_bottom_side+20.0) && check_between(c_bottom_side, c_top_side, pn_bottom_side + 20.0) && (check_between(c_left_side, c_right_side, pn_right_side) | check_between(c_left_side, c_right_side, pn_left_side));
            
            let collide_left: bool = !check_between(c_left_side, c_right_side, po_right_side) && (check_between(c_left_side, c_right_side, pn_right_side) && check_between(c_bottom_side, c_top_side, pn_top_side) | check_between(c_bottom_side, c_top_side, pn_bottom_side)) & !(collide_top | collide_bottom);
            let collide_right: bool = !check_between(c_left_side, c_right_side, po_left_side) && (check_between(c_left_side, c_right_side, pn_left_side) && check_between(c_bottom_side, c_top_side, pn_top_side) | check_between(c_bottom_side, c_top_side, pn_bottom_side)) & !(collide_top | collide_bottom);
            player.on_ground = collide_top;
            if collide_left | collide_right | collide_top | collide_bottom {
                match collider.action {
                    definitions::CollisionAction::Move => {},
                    definitions::CollisionAction::Death => {println!("Ow");},
                    definitions::CollisionAction::Goal => {println!("YAY")},
                    definitions::CollisionAction::Wall => {
                        if collide_top && vel_y < 0.0 {
                           
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
                    _ => {}
            
                }
            
            }
        }
        return Stop::None;

    }
    

    fn check_between(lower: f32, higher: f32, value: f32) -> bool {
        return (lower < value && value < higher);
    }