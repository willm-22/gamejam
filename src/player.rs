use bevy::prelude::*;

const PLAYER_COLOUR: Color = Color::hsl(300.0, 100.0, 100.0);
const STARTING_PLAYER_POSITION: (f32, f32) = (0.0, 0.0);
const STARTING_PLAYER_VELOCITY: (f32, f32) = (0.0, 0.0);
const PLAYER_WIDTH: f32 = 30.0;
const PLAYER_HEIGHT: f32 = 60.0;

#[derive(Component)]
struct Position {x: f32, y: f32}
impl Position {
    fn init(&mut self, x: f32, y: f32) {
        self.x = x;
        self.y = y;
    }

    fn new(x: f32, y: f32) -> Position {
        Position {x, y}
    }
}
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
#[derive(Component)]
pub struct Player;


pub fn spawn_player(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<ColorMaterial>>) {
    let shape: Handle<Mesh> = meshes.add(Rectangle::new(PLAYER_WIDTH, PLAYER_HEIGHT));
    let mesh: Mesh2d = Mesh2d(shape);
    let position = Position::new(STARTING_PLAYER_POSITION.0, STARTING_PLAYER_POSITION.1);
    let velocity = Velocity::new(STARTING_PLAYER_VELOCITY.0, STARTING_PLAYER_VELOCITY.1);
    commands.spawn((
            mesh, 
            MeshMaterial2d(materials.add(PLAYER_COLOUR)),
            Transform::from_xyz(
                STARTING_PLAYER_POSITION.0,
                STARTING_PLAYER_POSITION.1,
                0.0,
            ),
            Player,
            velocity
            ));
}

pub fn move_player(
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut transform: Single<&mut Transform, With<Player>>)
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
    if input.pressed(KeyCode::KeyD) {
        direction.y -= 1.0;
    }

    if direction != Vec2::ZERO {
        let delta = direction.normalize() * (1000.0) * time.delta_secs();
        transform.translation.x += delta.x;
}
}
        