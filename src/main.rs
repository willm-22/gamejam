// since we are learning bevy from scratch, here are the resources used to learn from
// https://aibodh.com/posts/bevy-rust-game-development-chapter-1/ 1.
// https://bevy.org/examples/2d-rendering/2d-shapes/ 2.
// any copied code will be marked with a start and end point

mod level_placer;
mod player;
mod definitions;
mod collision;
mod music;
mod clock;
use level_placer::draw_level;

use bevy::prelude::*;


fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Robert the Roofer".into(),
                resizable: true,
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, startup)
        .add_systems(Startup, test)
        .add_systems(Startup, music::setup_music)
        .add_systems(Update, player::animate_player)
        .add_systems(Startup, clock::setup_clock)
        .add_systems(Update, (clock::update_clock, clock::update_clock_text))
        .add_systems(Update, player::move_player)
        // .add_systems(Update, bob)
        .run();

}
fn test(mut commands: Commands, 
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>) {
    // add code to run for testing here
    // player::spawn_player(commands, meshes, materials);
}

fn startup(
    mut commands: Commands, 
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>) 
{
    commands.spawn(Camera2d);
    let shape = meshes.add(Rectangle::new(200.0, 100.0));
    let mesh = Mesh2d(shape);
    let color = Color::hsl(0.0, 0.95, 0.7); // red hsl(0, 100%, 50%)

    commands.spawn(
               ( 
                mesh,
                MeshMaterial2d(materials.add(color)), // 2.
                Transform::from_xyz(
                // Distribute shapes from -X_EXTENT/2 to +X_EXTENT/2.
                100.0,
                100.0,
                0.0,
                )
            )
            
            );
}

// fn move_the_rect(
//     input: Res<ButtonInput<KeyCode>>,
//     time: Res<Time>,

// )
fn bob() {
   // bob is a placeholder bob does not do anything
   println!("bob");
}