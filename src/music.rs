use bevy::prelude::*;

pub fn setup_music(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        AudioPlayer::new(asset_server.load("main_tune.wav")),
        PlaybackSettings::LOOP,
        MusicTrack,
    ));
}

#[derive(Component)]
pub struct MusicTrack;