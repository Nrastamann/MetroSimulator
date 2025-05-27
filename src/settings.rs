use bevy::prelude::*;

pub struct SettingsPlugin;

impl Plugin for SettingsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Settings>();
    }
}

#[derive(Resource)]
pub struct Settings{
    music_volume: f32,
    turn_on_metro_sfx: bool,
    sfx_volume: f32,
    metro_sfx_volume: f32,
}