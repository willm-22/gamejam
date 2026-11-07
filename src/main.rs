// since we are learning bevy from scratch, here are the resources used to learn from
// https://aibodh.com/posts/bevy-rust-game-development-chapter-1/ 1.
// https://bevy.org/examples/2d-rendering/2d-shapes/ 2.
// any copied code will be marked with a start and end point


use bevy::prelude::*;
mod level1_placer;
mod player;

#[derive(Component)]
struct Collision;


fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, startup)
        .add_systems(Startup, test)
        .add_systems(Update, player::move_player)
        // .add_systems(Update, bob)
        .run();

}
fn test(mut commands: Commands, 
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>) {
    // add code to run for testing here
    player::spawn_player(commands, meshes, materials);
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