use crate::{ui::ChangeSongNameEvent, GameState};
use bevy::{audio::Volume, prelude::*};
use bevy_lunex::cosmic_text::Change;
use rand::{rng, Rng};
use std::{time::Duration, usize};
pub struct AudioPlugin;

pub const MUSIC_NAMES: [&str; 8] = [
    "1.ogg", "2.ogg", "3.ogg", "4.ogg", "5.ogg", "6.ogg", "7.ogg", "8.ogg",
];
pub const SFX_NAMES: [&str; 2] = ["click.ogg", "wrong.ogg"];
pub const SFX_METRO_NAMES: [&str; 6] = ["1.ogg", "2.ogg", "3.ogg", "4.ogg", "5.ogg", "6.ogg"];

pub const FADE_TIME: f32 = 2.0;

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MetroTimer>()
            .add_event::<PlaySFXEvent>()
            .add_event::<ChangeOrderOfPlaying>()
            .add_event::<PlayMetroSFXEvent>()
            .add_event::<ChangeTrackEvent>()
            .add_event::<PlayMetroSFXEvent>()
            .add_systems(Startup, setup)
            .add_systems(
                Update,
                (
                    change_track,
                    play_sfx,
                    click,
                    change_order,
                    fade_out_sfx,
                    empty_track,
                    fade_out,
                    fade_in,
                ),
            )
            .add_systems(
                Update,
                (tick_timer, play_metro_sfx).run_if(in_state(GameState::InGame)),
            );
    }
}

#[derive(PartialEq)]
enum PlayerState {
    Stopped,
    Ended,
    Playing,
}

pub enum PlayerMode {
    Straight,
    Shuffle,
    SingleRepeat,
}
#[derive(Resource)]
pub struct MusicPlayer {
    pub track_list: Vec<Handle<AudioSource>>,
    pub sfx_list: Vec<Handle<AudioSource>>,
    pub sfx_metro_list: Vec<Handle<AudioSource>>,
    current_state: PlayerState,
    pub player_mode: PlayerMode,
    pub current_composition: usize,
    pub order: Vec<usize>,
    pub sfx_playing: bool,
}

impl MusicPlayer {
    fn new(
        track_list: Vec<Handle<AudioSource>>,
        sfx_list: Vec<Handle<AudioSource>>,
        sfx_metro_list: Vec<Handle<AudioSource>>,
    ) -> Self {
        let mut order: Vec<usize> = vec![];

        for i in 0..track_list.len() {
            order.push(i);
        }

        Self {
            track_list,
            sfx_list,
            sfx_metro_list,
            current_state: PlayerState::Playing,
            player_mode: PlayerMode::Straight,
            current_composition: 0,
            order,
            sfx_playing: false,
        }
    }
}
#[derive(Resource)]
struct MetroTimer {
    time_to_play: Timer,
}

impl Default for MetroTimer {
    fn default() -> Self {
        Self {
            time_to_play: Timer::new(Duration::from_secs(18), TimerMode::Repeating),
        }
    }
}

#[derive(Component)]
struct SFX;

#[derive(Component)]
pub struct Soundtrack;

#[derive(Component)]
struct MetroSounds;

#[derive(Component)]
struct FadeIn(pub f32);

#[derive(Component)]
struct FadeOut(pub f32);

#[derive(Event)]
pub struct ChangeTrackEvent {
    pub track: Option<usize>,
}

#[derive(Event)]
struct PlayMetroSFXEvent;

pub enum SFXType {
    ClickLKMSound,
    ErrorSound,
}

#[derive(Event)]
pub struct PlaySFXEvent {
    pub sfx_to_play: SFXType,
}

#[derive(Event)]
pub struct ChangeOrderOfPlaying;

fn change_order(
    mut change_order: EventReader<ChangeOrderOfPlaying>,
    mut music_player: ResMut<MusicPlayer>,
) {
    for _ev in change_order.read() {
        print!("Before: ");
        for i in music_player.order.iter() {
            print!("{} ", i);
        }
        println!("");

        match music_player.player_mode {
            PlayerMode::Straight => {
                for i in 0..music_player.track_list.len() {
                    if MUSIC_NAMES[music_player.current_composition] == MUSIC_NAMES[i] {
                        music_player.current_composition = i;
                        break;
                    }
                }

                for i in 0..music_player.track_list.len() {
                    music_player.order[i] = i;
                }
            }

            PlayerMode::SingleRepeat => {
                print!("How'd you get there?");
                return;
            }

            PlayerMode::Shuffle => {
                let mut len = music_player.order.len();
                for i in music_player.current_composition + 1..music_player.order.len() {
                    music_player.order.swap(i, rand::rng().random_range(i..len));
                }
                len = music_player.current_composition;
                for i in 0..music_player.current_composition {
                    music_player.order.swap(i, rand::rng().random_range(i..len));
                }
            }
        }
        print!("After: ");
        for i in music_player.order.iter() {
            print!("{} ", i);
        }
        println!("");
    }
}

fn tick_timer(
    time: Res<Time>,
    mut time_to_sfx: ResMut<MetroTimer>,
    mut play_sfx: EventWriter<PlayMetroSFXEvent>,
    metro_sfx_q: Query<Entity, With<MetroSounds>>,
    mut player: ResMut<MusicPlayer>,
) {
    if metro_sfx_q.iter().len() != 0 {
        return;
    }
    time_to_sfx.time_to_play.tick(time.delta());

    if time_to_sfx.time_to_play.just_finished() {
        play_sfx.send(PlayMetroSFXEvent);
        time_to_sfx
            .time_to_play
            .set_duration(Duration::from_secs(rand::rng().random_range(14..60)));
        time_to_sfx.time_to_play.reset();
        player.sfx_playing = true;
        println!("Timer is over")
    }
}

fn fade_out_sfx(
    sfx_q: Query<Entity, With<MetroSounds>>,
    mut player: ResMut<MusicPlayer>,
    music_q: Query<Entity, With<Soundtrack>>,
    mut commands: Commands,
) {
    if sfx_q.iter().len() == 0 && player.sfx_playing {
        player.sfx_playing = false;
        println!("sfx is over");

        for music_e in music_q.iter() {
            commands.entity(music_e).insert(FadeIn(1.0));
        }
    }
}

fn click(mut sfx_sound_ev: EventWriter<PlaySFXEvent>, mouse: Res<ButtonInput<MouseButton>>) {
    if mouse.just_pressed(MouseButton::Left) {
        sfx_sound_ev.send(PlaySFXEvent {
            sfx_to_play: SFXType::ClickLKMSound,
        });
    }
}

fn play_sfx(
    mut commands: Commands,
    music_player: Res<MusicPlayer>,
    mut play_sfx_ev: EventReader<PlaySFXEvent>,
) {
    for ev in play_sfx_ev.read() {
        let sfx_num;
        match ev.sfx_to_play {
            SFXType::ClickLKMSound => {
                sfx_num = 0;
            }
            SFXType::ErrorSound => {
                sfx_num = 1;
            }
        }
        commands.spawn((
            AudioPlayer::new(music_player.sfx_list[sfx_num].clone()),
            SFX,
            PlaybackSettings {
                mode: bevy::audio::PlaybackMode::Despawn,
                volume: Volume::new(0.5),
                ..default()
            },
        ));
    }
}

fn play_metro_sfx(
    mut commands: Commands,
    mut play_metro_sfx_event: EventReader<PlayMetroSFXEvent>,
    music_q: Query<Entity, With<Soundtrack>>,
    music_player: Res<MusicPlayer>,
) {
    for _ev in play_metro_sfx_event.read() {
        for music_e in music_q.iter() {
            commands.entity(music_e).insert(FadeOut(0.5));
        }
        commands.spawn((
            AudioPlayer::new(
                music_player.sfx_metro_list
                    [rand::rng().random_range(0..music_player.sfx_metro_list.len())]
                .clone(),
            ),
            MetroSounds,
            PlaybackSettings {
                mode: bevy::audio::PlaybackMode::Despawn,
                ..default()
            },
        ));
    }
}

fn empty_track(
    soundtrack: Query<Entity, With<Soundtrack>>,
    mut player: ResMut<MusicPlayer>,
    mut change_track: EventWriter<ChangeTrackEvent>,
) {
    if player.current_state == PlayerState::Playing && soundtrack.iter().len() == 0 {
        print!("why?");
        change_track.send(ChangeTrackEvent { track: None });
        player.current_state = PlayerState::Ended;
    }
}

fn setup(asset_server: Res<AssetServer>, mut commands: Commands) {
    let mut track_list: Vec<Handle<AudioSource>> = vec![];
    let mut sfx_list: Vec<Handle<AudioSource>> = vec![];
    let mut sfx_metro_list: Vec<Handle<AudioSource>> = vec![];

    for i in MUSIC_NAMES {
        track_list.push(asset_server.load::<AudioSource>("music/".to_owned() + i));
    }
    for i in SFX_METRO_NAMES {
        sfx_metro_list.push(asset_server.load::<AudioSource>("metro_sfx/".to_owned() + i));
    }
    for i in SFX_NAMES {
        sfx_list.push(asset_server.load::<AudioSource>("sfx/".to_owned() + i));
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
    mut audio_sink: Query<(&mut AudioSink, Entity, &FadeOut)>,
    time: Res<Time>,
) {
    for (audio, entity, fade_amount) in audio_sink.iter_mut() {
        let current_volume = audio.volume();
        audio.set_volume(current_volume - Volume::new(time.delta_secs() / FADE_TIME).get());
        if audio.volume() <= 0.0 {
            commands.entity(entity).despawn();
            println!("Done fading, removing track");
        }
        if fade_amount.0 >= audio.volume() {
            audio.set_volume(fade_amount.0);
            commands.entity(entity).remove::<FadeOut>();
            println!("Done fading");
        }
    }
}

fn fade_in(
    mut commands: Commands,
    mut audio_sink: Query<(&mut AudioSink, Entity, &FadeIn)>,
    time: Res<Time>,
) {
    for (audio, entity, fade) in audio_sink.iter_mut() {
        let current_volume = audio.volume();
        audio.set_volume(current_volume + Volume::new(time.delta_secs() / FADE_TIME).get());
        if audio.volume() >= fade.0 {
            audio.set_volume(fade.0);
            commands.entity(entity).remove::<FadeIn>();
        }
    }
}

fn change_track(
    mut commands: Commands,
    soundtrack: Query<Entity, (With<Soundtrack>, With<AudioSink>)>,
    mut change_track_ev: EventReader<ChangeTrackEvent>,
    game_state: Res<State<GameState>>,
    mut change_song_name: EventWriter<ChangeSongNameEvent>,
    mut music_player: ResMut<MusicPlayer>,
) {
    for ev in change_track_ev.read() {        
        if ev.track.is_some(){
            for music in soundtrack.iter(){
                commands.entity(music).despawn();
            }
            if ev.track.unwrap() >= MUSIC_NAMES.len(){
                for i in 0..music_player.order.len(){
                    if music_player.order[i] == music_player.current_composition{
                        match ev.track.unwrap(){
                            usize::MAX =>{
                                if i == 0{
                                    music_player.current_composition = music_player.order[music_player.order.len() - 1];
                                    break;
                                }
                                music_player.current_composition = music_player.order[i - 1];
                                break;        
                            }
                            _ =>{
                                if i >= MUSIC_NAMES.len() - 1{
                                    music_player.current_composition = music_player.order[0];
                                    break;
                                }
                                music_player.current_composition = music_player.order[i + 1];
                                break;
        
                            }
                        }
                    }
            }
        }else{
            music_player.current_composition = ev.track.unwrap();
        }
            change_song_name.send(ChangeSongNameEvent);
            commands.spawn((
                AudioPlayer::new(
                    music_player.track_list[music_player.order[music_player.current_composition]].clone(),
                ),
                Soundtrack,
                PlaybackSettings {
                    mode: bevy::audio::PlaybackMode::Despawn,
                    volume: Volume::new(1.),
                    ..default()
                },
            ));
            return;
        } 
        
        for music in soundtrack.iter() {
            commands.entity(music).insert(FadeOut(0.0));
            println!("fade");
        }

        match game_state.get() {
            GameState::InGame => {
                println!("game track");
                let track_num;

                match music_player.player_mode {
                    PlayerMode::SingleRepeat => {
                        track_num = music_player.current_composition;
                    }
                    _ => {
                        music_player.current_composition =
                            (music_player.current_composition + 1) % music_player.order.len();

                        track_num = music_player.current_composition;
                    }
                }
                commands.spawn((
                    AudioPlayer::new(
                        music_player.track_list[music_player.order[track_num]].clone(),
                    ),
                    Soundtrack,
                    PlaybackSettings {
                        mode: bevy::audio::PlaybackMode::Despawn,
                        volume: Volume::new(1.),
                        ..default()
                    },
                ));
            }

            _ => {
                println!("menu track");
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
        change_song_name.send(ChangeSongNameEvent);
        music_player.current_state = PlayerState::Playing;
    }
}

fn volume(keyboard: Res<ButtonInput<KeyCode>>) {}
//прикрутить изменение громкости
//сделать плеер, как-нибудь, посмотреть, можно ли сделать слайдер, хз
