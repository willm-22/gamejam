use bevy::prelude::*;
use crate::definitions;
use crate::collision;

const STARTING_PLAYER_VELOCITY: (f32, f32) = (0.0, 0.0);





pub fn spawn_player(starting_x: f32, starting_y: f32, commands: &mut Commands,  colour: Color) {
    // let shape: Handle<Mesh> = meshes.add(Rectangle::new(PLAYER_WIDTH, PLAYER_HEIGHT));
    // let mesh: Mesh2d = Mesh2d(shape);
    let velocity = definitions::Velocity::new(STARTING_PLAYER_VELOCITY.0, STARTING_PLAYER_VELOCITY.1);
    commands.spawn((
            Sprite::from_color(colour, Vec2::new(definitions::PLAYER_SIZE.width, definitions::PLAYER_SIZE.height)),
            Transform::from_xyz(
                starting_x,
                starting_y,
                0.0,
            ),
            definitions::Player,
            velocity,
            definitions::LevelEntity,
            definitions::PLAYER_SIZE
            ));
}

pub fn move_player(
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    player: Single<(&mut Transform, &mut definitions::Velocity, &definitions::Size), With<definitions::Player>>,
    collision_all_sides: Query<(&mut Transform, &definitions::Size, &definitions::Collision), Without<definitions::Player>>)
{

    let (mut transform, mut velocity, ssize) = player.into_inner();

    if input.pressed(KeyCode::KeyA) {
        velocity.x -= 10.0;
    }
    if input.pressed(KeyCode::KeyD) {
        velocity.x += 10.0;
    }
    if input.pressed(KeyCode::KeyW) {
        velocity.y += 10.0;
    }
    if input.pressed(KeyCode::KeyS) {
        velocity.y -= 10.0;
    }
    
    velocity.mult(0.95);

    // velocity.y -= 5.0;

    if velocity.x >= 100.0 { velocity.x = 100.0}
    if velocity.y >= 100.0 { velocity.x = 100.0}

    let delta_x = velocity.x * time.delta_secs();
    let delta_y = velocity.y * time.delta_secs();
    match collision::check_collision( transform.translation.x,  transform.translation.y, delta_x, delta_y, &mut *transform, ssize, collision_all_sides) {
        collision::Stop::None => {transform.translation.x += delta_x;
            transform.translation.y += delta_y;}
        collision::Stop::Verticle => {transform.translation.x += delta_x; 
            velocity.y = 0.0;}
        collision::Stop::Sideways => {transform.translation.y += delta_y;
        velocity.x = 0.0;}
         
    }


}
        