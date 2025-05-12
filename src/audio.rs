use crate::GameState;
use bevy::{audio::Volume, prelude::*};
pub struct AudioPlugin;

pub const MUSIC_NAMES: [&str; 8] = [
    "1.ogg", "2.ogg", "3.ogg", "4.ogg", "5.ogg", "6.ogg", "7.ogg", "8.ogg",
];
pub const SFX_NAMES: [&str; 5] = ["", "", "", "", ""];
pub const SFX_METRO_NAMES: [&str; 6] = ["1.ogg", "2.ogg", "3.ogg", "4.ogg", "5.ogg", "6.ogg"];

pub const FADE_TIME: f32 = 2.0;

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ChangeTrackEvent>()
            .add_systems(OnEnter(GameState::MainMenu), setup)
            .add_systems(Update, (change_track, empty_track).after(setup));
    }
}

#[derive(PartialEq)]
enum PlayerState {
    Stopped,
    Ended,
    Playing,
}

enum PlayerMode {
    Straight,
    Shuffle,
    SingleRepeat,
}
#[derive(Resource)]
pub struct MusicPlayer {
    track_list: Vec<Handle<AudioSource>>,
    sfx_list: Vec<Handle<AudioSource>>,
    sfx_metro_list: Vec<Handle<AudioSource>>,
    current_state: PlayerState,
    player_mode: PlayerMode,
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
            current_state: PlayerState::Playing,
            player_mode: PlayerMode::Straight,
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

#[derive(Component)]
struct Music;

#[derive(Event)]
struct ChangeTrackEvent;

fn empty_track(
    soundtrack: Query<Entity, With<Soundtrack>>,
    mut player: ResMut<MusicPlayer>,
    mut change_track: EventWriter<ChangeTrackEvent>,
) {
    if player.current_state == PlayerState::Playing && soundtrack.iter().len() == 0 {
        print!("why?");
        change_track.send(ChangeTrackEvent);
        player.current_state = PlayerState::Ended;
    }else{
        println!("amount of tracks - {}", soundtrack.iter().len());
    }
}

fn setup(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut change_track: EventWriter<ChangeTrackEvent>,
) {
    let mut track_list: Vec<Handle<AudioSource>> = vec![];
    let mut sfx_list: Vec<Handle<AudioSource>> = vec![];
    let mut sfx_metro_list: Vec<Handle<AudioSource>> = vec![];

    for i in MUSIC_NAMES {
        track_list.push(asset_server.load::<AudioSource>("music/".to_owned() + i));
    }
    for i in SFX_METRO_NAMES {
        sfx_metro_list.push(asset_server.load::<AudioSource>("sfx/".to_owned() + i));
    }
    for i in SFX_NAMES {
        sfx_list.push(asset_server.load::<AudioSource>("metro_sfx/".to_owned() + i));
    }

    commands.spawn((
        AudioPlayer::new(track_list[0].clone()),
        Soundtrack,
        PlaybackSettings {
            mode: bevy::audio::PlaybackMode::Despawn,
            volume: Volume::new(0.8),
            ..default()
        },
    ));

    commands.insert_resource(MusicPlayer::new(track_list, sfx_list, sfx_metro_list));
}

fn fade_out(
    mut commands: Commands,
    mut audio_sink: Query<(&mut AudioSink, Entity), With<FadeOut>>,
    time: Res<Time>,
) {
    for (audio, entity) in audio_sink.iter_mut() {
        let current_volume = audio.volume();
        audio.set_volume(current_volume - Volume::new(time.delta_secs() / FADE_TIME).get());
        if audio.volume() <= 0.0 {
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
        audio.set_volume(current_volume + Volume::new(time.delta_secs() / FADE_TIME).get());
        if audio.volume() >= 1.0 {
            audio.set_volume(1.0);
            commands.entity(entity).remove::<FadeIn>();
        }
    }
}

fn change_track(
    mut commands: Commands,
    soundtrack: Query<Entity, (With<Music>, With<AudioSink>)>,
    mut change_track_ev: EventReader<ChangeTrackEvent>,
    game_state: Res<State<GameState>>,
    global_vol: Res<GlobalVolume>, //kinda useless?
    mut music_player: ResMut<MusicPlayer>,
) {
    for ev in change_track_ev.read() {
        for music in soundtrack.iter() {
            commands.entity(music).insert(FadeOut);
            println!("fade");
        }
        match game_state.get() {
            GameState::InGame => {

                //просто ставим след трек в ingame, надо добавить в ивент, как мы поменяли звук, он кончился/игрок поменял
            }
            _ => {
                println!("new track");
                commands.spawn((
                    AudioPlayer::new(music_player.track_list[0].clone()),
                    Soundtrack,
                    PlaybackSettings {
                        mode: bevy::audio::PlaybackMode::Despawn,
                        volume: Volume::new(0.8),
                        ..default()
                    },
                ));
                //короче, если в меню или в настройках, то надо бы накинуть - громкость? хз
            }
        }
        music_player.current_state = PlayerState::Playing;
    }
}

fn volume(keyboard: Res<ButtonInput<KeyCode>>) {}
//прикрутить изменение громкости
//сделать плеер, как-нибудь, посмотреть, можно ли сделать слайдер, хз
