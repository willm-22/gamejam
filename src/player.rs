use bevy::prelude::*;
use crate::definitions;

const STARTING_PLAYER_VELOCITY: (f32, f32) = (0.0, 0.0);


#[derive(Component)]
pub struct Velocity {x: f32, y : f32}
impl Velocity {
    fn init(&mut self, x: f32, y: f32) {
        self.x = x;
        self.y = y;
    }

    fn new(x: f32, y: f32) -> Velocity {
        Velocity {x, y}
    }

    fn mult(&mut self, num: f32) {
        self.x = self.x * num;
    }

}


pub fn spawn_player(starting_x: f32, starting_y: f32, commands: &mut Commands,  colour: Color) {
    // let shape: Handle<Mesh> = meshes.add(Rectangle::new(PLAYER_WIDTH, PLAYER_HEIGHT));
    // let mesh: Mesh2d = Mesh2d(shape);
    let velocity = Velocity::new(STARTING_PLAYER_VELOCITY.0, STARTING_PLAYER_VELOCITY.1);
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
    current_movement: Single<(&mut Transform, &mut Velocity), With<definitions::Player>>)
{

    let (mut transform, mut velocity) = current_movement.into_inner();

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

    velocity.y -= 5.0;

    if velocity.x >= 100.0 { velocity.x = 100.0}
    if velocity.y >= 100.0 { velocity.x = 100.0}

    let delta_x = velocity.x * time.delta_secs();
    let delta_y = velocity.y * time.delta_secs();
    transform.translation.x += delta_x;
    transform.translation.y += delta_y;

}
        