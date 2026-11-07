// since we are learning bevy from scratch, here are the resources used to learn from
// https://aibodh.com/posts/bevy-rust-game-development-chapter-1/ 1.
// any copied code will be marked with a start and end point


// copied from 1. for now
//main.rs
use bevy::prelude::*;
use crate::player::PlayerPlugin;

mod player;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::WHITE)) // We have updated the bg color to white
        .add_plugins(
            DefaultPlugins.set(AssetPlugin {
                // Our assets live in `src/assets` for this project
                file_path: "src/assets".into(),
                ..default()
            }),
        )
        .add_systems(Startup, setup_camera)
        .add_plugins(PlayerPlugin)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}