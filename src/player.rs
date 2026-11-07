use bevy::prelude::*;
use crate::definitions;

const STARTING_PLAYER_VELOCITY: (f32, f32) = (0.0, 0.0);


#[derive(Component)]
struct Velocity {x: f32, y : f32}
impl Velocity {
    fn init(&mut self, x: f32, y: f32) {
        self.x = x;
        self.y = y;
    }

    fn new(x: f32, y: f32) -> Velocity {
        Velocity {x, y}
    }
}


pub fn spawn_player(starting_x: f32, starting_y: f32, commands: &mut Commands,  colour: Color) {
    // let shape: Handle<Mesh> = meshes.add(Rectangle::new(PLAYER_WIDTH, PLAYER_HEIGHT));
    // let mesh: Mesh2d = Mesh2d(shape);
    let velocity = Velocity::new(STARTING_PLAYER_VELOCITY.0, STARTING_PLAYER_VELOCITY.1);
    commands.spawn((
            Sprite::from_color(colour, Vec2::new(definitions::PLAYER_WIDTH, definitions::PLAYER_HEIGHT)),
            Transform::from_xyz(
                starting_x,
                starting_y,
                0.0,
            ),
            definitions::Player,
            velocity
            ));
}

pub fn move_player(
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut transform: Single<&mut Transform, With<definitions::Player>>)
{

    let mut direction = Vec2::ZERO;
    if input.pressed(KeyCode::KeyA) {
        direction.x -= 1.0;
    }
    if input.pressed(KeyCode::KeyD) {
        direction.x += 1.0;
    }
    if input.pressed(KeyCode::KeyW) {
        direction.y += 1.0;
    }
    if input.pressed(KeyCode::KeyS) {
        direction.y -= 1.0;
    }

    if direction != Vec2::ZERO {
        let delta = direction.normalize() * (1000.0) * time.delta_secs();
        transform.translation.x += delta.x;
        transform.translation.y += delta.y;
}
}
        