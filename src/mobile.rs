use bevy::prelude::*;
use crate::definitions;

fn move_rectange(mobileRectanges: Query<Collisions>) {
    for collision in &mobileRectanges {
        if collision.action == definitions::collisions::Move {
            
        }
    }
}