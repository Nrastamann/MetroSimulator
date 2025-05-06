use bevy::{audio::Volume, prelude::*};

use crate::GameState;
pub struct AudioPlugin;

pub const MUSIC_NAMES: [&str; 5] = ["", "", "", "", ""];
pub const SFX_NAMES: [&str; 5] = ["", "", "", "", ""];
pub const SFX_METRO_NAMES: [&str; 5] = ["", "", "", "", ""];

pub const FADE_TIME: f32 = 2.0;

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {}
}

#[derive(Resource)]
pub struct MusicPlayer {
    track_list: Vec<Handle<AudioSource>>,
    sfx_list: Vec<Handle<AudioSource>>,
    sfx_metro_list: Vec<Handle<AudioSource>>,
}

impl MusicPlayer {
    fn new(
        track_list: Vec<Handle<AudioSource>>,
        sfx_list: Vec<Handle<AudioSource>>,
        sfx_metro_list: Vec<Handle<AudioSource>>,
    ) -> Self {
        Self {
            track_list,
            sfx_list,
            sfx_metro_list,
        }
    }
}

#[derive(Component)]
struct SFX;

#[derive(Component)]
struct Soundtrack;

#[derive(Component)]
struct MetroSounds;

#[derive(Component)]
struct FadeIn;

#[derive(Component)]
struct FadeOut;

#[derive(Event)]
struct ChangeTrackEvent;

fn setup(asset_server: Res<AssetServer>, mut commands: Commands) {
    let mut track_list: Vec<Handle<AudioSource>> = vec![];
    let mut sfx_list: Vec<Handle<AudioSource>> = vec![];
    let mut sfx_metro_list: Vec<Handle<AudioSource>> = vec![];

    for i in MUSIC_NAMES {
        track_list.push(asset_server.load::<AudioSource>("music/".to_owned() + i));
    }
    for i in SFX_METRO_NAMES {
        sfx_metro_list.push(asset_server.load::<AudioSource>("music/".to_owned() + i));
    }
    for i in SFX_NAMES {
        sfx_list.push(asset_server.load::<AudioSource>("music/".to_owned() + i));
    }

    commands.insert_resource(MusicPlayer::new(track_list, sfx_list, sfx_metro_list));
}

fn fade_out(
    mut commands: Commands,
    mut audio_sink: Query<(&mut AudioSink, Entity), With<FadeOut>>,
    time: Res<Time>,
) {
    for (mut audio, entity) in audio_sin1k.iter_mut() {
        let current_volume = audio.volume();
        audio.set_volume(current_volume - Volume::Linear(time.delta_secs() / FADE_TIME));
        if audio.volume().to_linear() <= 0.0 {
            commands.entity(entity).despawn();
        }
    }
}

fn fade_in(
    mut commands: Commands,
    mut audio_sink: Query<(&mut AudioSink, Entity), With<FadeIn>>,
    time: Res<Time>,
) {
    for (mut audio, entity) in audio_sink.iter_mut() {
        let current_volume = audio.volume();
        audio.set_volume(current_volume + Volume::Linear(time.delta_secs() / FADE_TIME));
        if audio.volume().to_linear() >= 1.0 {
            audio.set_volume(Volume::Linear(1.0));
            commands.entity(entity).remove::<FadeIn>();
        }
    }
}

fn change_track(
    mut commands: Commands,
    soundtrack: Query<Entity, With<AudioSink>>,
    mut change_track_ev: EventReader<ChangeTrackEvent>,
    game_state: State<GameState>,
) {
    for ev in change_track_ev.read() {
        match game_state.get() {
            GameState::InGame => {
                //просто ставим след трек в ingame, надо добавить в ивент, как мы поменяли звук, он кончился/игрок поменял
            }
            _ => {
                //короче, если в меню или в настройках, то надо бы накинуть - громкость? хз
            }
        }
    }
}

fn volume(keyboard: Res<ButtonInput<KeyCode>>) {}
//прикрутить изменение громкости
//сделать плеер, как-нибудь, посмотреть, можно ли сделать слайдер, хз