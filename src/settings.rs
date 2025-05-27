use bevy::prelude::*;

use crate::{audio::Soundtrack, GameState};

pub struct SettingsPlugin;

impl Plugin for SettingsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Settings>();
        app.add_systems(Update, (music_volume_change,turn_off_music).run_if(in_state(GameState::InGame)));
    }
}

#[derive(Resource)]
pub struct Settings{
    pub music_volume: f32,
    pub turn_on_metro_sfx: bool,
    pub turn_on_sfx: bool,
    pub turn_on_music: bool,
    pub sfx_volume: f32,
    pub metro_sfx_volume: f32,
}
impl Default for Settings{
    fn default() -> Self {
        Self{
            music_volume: 1.,
            turn_on_metro_sfx: true,
            turn_on_sfx: true,
            turn_on_music: true,
            sfx_volume: 1.,
            metro_sfx_volume: 1.,
        }
    }
}
fn music_volume_change(keyboard: Res<ButtonInput<KeyCode>>, mut settings: ResMut<Settings>, mut music_q: Query<&mut AudioSink, With<Soundtrack>>){
    if keyboard.just_pressed(KeyCode::Equal){
        if settings.music_volume < 1.0{
            settings.music_volume += 0.1; 
            for mut volume in music_q.iter_mut(){
                volume.set_volume(settings.music_volume);
            }
        }
    }
    if keyboard.just_pressed(KeyCode::Minus){
        if settings.music_volume > 0.{
            settings.music_volume -= 0.1; 
            for mut volume in music_q.iter_mut(){
                volume.set_volume(settings.music_volume);
            }
        }
    }
}

fn turn_off_music(keyboard: Res<ButtonInput<KeyCode>>, mut settings: ResMut<Settings>,){
    if keyboard.just_pressed(KeyCode::KeyM){
        settings.turn_on_music = !settings.turn_on_music;
    }
    if keyboard.just_pressed(KeyCode::KeyK){
        settings.turn_on_sfx = !settings.turn_on_sfx;
    }
    if keyboard.just_pressed(KeyCode::KeyL){
        settings.turn_on_metro_sfx = !settings.turn_on_metro_sfx;
    }
}