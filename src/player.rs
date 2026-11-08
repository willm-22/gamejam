use bevy::prelude::*;
use crate::definitions;
use crate::clock::GameClock;
use std::time::Duration;
use crate::collision;

const STARTING_PLAYER_VELOCITY: (f32, f32) = (0.0, 0.0);

#[derive(Component)]
pub struct AnimationTimer(pub Timer);

#[derive(Component)]
pub struct AnimationIndices {
    pub first: usize,
    pub last: usize,
}




pub fn spawn_player(starting_x: f32, starting_y: f32, commands: &mut Commands, asset_server: &AssetServer, layouts: &mut Assets<TextureAtlasLayout>) {
    // let shape: Handle<Mesh> = meshes.add(Rectangle::new(PLAYER_WIDTH, PLAYER_HEIGHT));
    // let mesh: Mesh2d = Mesh2d(shape);
    let velocity = definitions::Velocity::new(STARTING_PLAYER_VELOCITY.0, STARTING_PLAYER_VELOCITY.1);
    let player = asset_server.load("player.png");
    let layout = layouts.add(TextureAtlasLayout::from_grid(UVec2::new(32, 32), 4, 1, None, None));
    let indices = AnimationIndices { first: 0, last: 3 };

    let mut sprite = Sprite::from_atlas_image(
        player,
        TextureAtlas { layout, index: indices.first },
    );

    sprite.custom_size = Some(Vec2::new(definitions::PLAYER_SIZE.height, definitions::PLAYER_SIZE.width));

    commands.spawn((
            sprite,
            Transform::from_xyz(
                starting_x,
                starting_y,
                0.0,
            ),
            definitions::Player {on_ground: false},
            velocity,
            indices,
            AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
            definitions::LevelEntity,
            definitions::PLAYER_SIZE
            ));
}

pub fn move_player(
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    player: Single<(&mut Transform, &mut definitions::Velocity, &definitions::Size, &mut definitions::Player), With<definitions::Player>>,
    collision_all_sides: Query<(&mut Transform, &definitions::Size, &definitions::Collision), Without<definitions::Player>>)
{
    let (mut transform, mut velocity, ssize, mut player) = player.into_inner();
    let delta_time = time.delta_secs();
    const JUMP: f32 = 300.0;
    if input.pressed(KeyCode::KeyA) {velocity.x -= 50.0;}
    if input.pressed(KeyCode::KeyD) {velocity.x += 50.0;}

    if input.pressed(KeyCode::KeyW) && player.on_ground {
        velocity.y = JUMP;
    } 
    if !player.on_ground {
        velocity.y -= 30.0;
    }

    if input.pressed(KeyCode::KeyS) { velocity.y -= 10.0; }
    if velocity.x > 300.0 {velocity.x = 300.0}
    if velocity.y > 300.0 {velocity.y = 300.0}
    if velocity.x < -300.0 {velocity.x = -300.0}
    if velocity.y < -300.0 {velocity.y = -300.0}
    let delta_x = velocity.x * delta_time;
    let delta_y = velocity.y * delta_time;

    println!("vx = {:.0}, grounded = {}", velocity.x, player.on_ground);

    match collision::check_collision(transform.translation.x, transform.translation.y, delta_x, delta_y, &mut *transform, ssize, &mut *player, collision_all_sides) {
        0 => {
            transform.translation.x += delta_x;
            transform.translation.y += delta_y;
        }
        1 => {
            transform.translation.x += delta_x;
            velocity.y = 0.0;
        }
        2 => {
            transform.translation.y += delta_y;
            velocity.x = 0.0;
        }
        3 => {
            velocity.x = 0.0;
            velocity.y = 0.0;
        }
        _ => {;}
    }
}


pub fn animate_player(
    time: Res<Time>,
    clock: Res<GameClock>,
    mut query: Query<(&AnimationIndices, &mut AnimationTimer, &mut Sprite), With<definitions::Player>>,
) {
    for (indices, mut timer, mut sprite) in &mut query {
        timer.0.tick(Duration::from_secs_f32(time.delta_secs() * clock.scale));
        if timer.0.just_finished() {
            if let Some(atlas) = &mut sprite.texture_atlas {
                atlas.index = if atlas.index >= indices.last {
                    indices.first
                } else {
                    atlas.index + 1
                };
            }
        }
    }
}