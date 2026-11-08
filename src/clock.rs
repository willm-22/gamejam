use bevy::prelude::*;
use crate::definitions;

#[derive(Resource)]
pub struct GameClock {
    pub remaining: f32,
    pub scale: f32,
}

#[derive(Component)]
pub struct TimerText;

pub fn setup_clock(mut commands: Commands) {
    commands.insert_resource(GameClock { remaining: 60.0, scale: 1.0 });
    commands.spawn((Text::new("60.0"), TimerText));
}

pub fn update_clock(
    time: Res<Time>,
    mut clock: ResMut<GameClock>,
    player: Single<&definitions::Velocity, With<definitions::Player>>,
) {
    clock.scale = 1.0 - (player.speed() / 100.0).min(1.0) * 0.9;
    clock.remaining = (clock.remaining - time.delta_secs() * clock.scale).max(0.0);
}

pub fn update_clock_text(clock: Res<GameClock>, mut text: Single<&mut Text, With<TimerText>>) {
    text.0 = format!("{:.1}", clock.remaining);
}