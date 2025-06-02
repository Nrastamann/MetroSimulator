use crate::ui::Slider;
use crate::{audio::Soundtrack, GameState};
use bevy::prelude::*;
use std::fs::File;
use std::io::{BufRead, BufReader, Read, Write};
pub struct SettingsPlugin;

impl Plugin for SettingsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ChangeSettingEvent>()
            .add_event::<SaveSettingEvent>();
        app.init_resource::<Settings>();
        app.add_systems(Startup, read_from_file);
        app.add_systems(
            Update,
            (change_settings, write_to_file).run_if(in_state(GameState::Settings)),
        );

        app.add_systems(
            Update,
            (change_settings, music_volume_change, turn_off_music)
                .run_if(in_state(GameState::InGame)),
        );
    }
}

pub enum SettingsType {
    TurnOfSFX = 0,
    TurnOfMetroSFX,
    MusicVolume,
    SFXVolume,
    SFXMetroVolume,
}

impl From<usize> for SettingsType {
    fn from(value: usize) -> Self {
        match value {
            _ if value == SettingsType::TurnOfSFX as usize => Self::TurnOfSFX,
            _ if value == SettingsType::TurnOfMetroSFX as usize => Self::TurnOfMetroSFX,
            _ if value == SettingsType::MusicVolume as usize => Self::MusicVolume,
            _ if value == SettingsType::SFXVolume as usize => Self::SFXVolume,
            _ if value == SettingsType::SFXMetroVolume as usize => Self::SFXMetroVolume,
            _ => panic!("WRONG VALUE"),
        }
    }
}

#[derive(Event)]
pub struct ChangeSettingEvent;

#[derive(Event)]
pub struct SaveSettingEvent;

#[derive(Resource)]
pub struct Settings {
    pub music_volume: f32,
    pub turn_on_metro_sfx: bool,
    pub turn_on_sfx: bool,
    pub sfx_volume: f32,
    pub metro_sfx_volume: f32,
}
impl Default for Settings {
    fn default() -> Self {
        Self {
            music_volume: 1.,
            turn_on_metro_sfx: true,
            turn_on_sfx: true,
            sfx_volume: 1.,
            metro_sfx_volume: 1.,
        }
    }
}

fn write_to_file(settings: Res<Settings>, mut save_settings: EventReader<SaveSettingEvent>) {
    for _ev in save_settings.read() {
        let mut file = File::create("Settings.txt").expect("Creation failed!");
        


        file.write(
            (settings.turn_on_sfx.to_string()
                + "\n"
                + &settings.turn_on_metro_sfx.to_string()
                + "\n"
                + &(settings.music_volume * 100.).to_string() 
                + "\n"
                + &(settings.sfx_volume * 100.).to_string()
                + "\n"
                + &(settings.metro_sfx_volume * 100.).to_string())
                .as_bytes(),
        )
        .expect("file error");
    }
}

fn read_from_file(mut settings: ResMut<Settings>) {
    let settings_file = File::open("Settings.txt").unwrap();

    let reader = BufReader::new(settings_file);
    let mut counter: usize = 0;
    for line in reader.lines() {
        match counter.into() {
            SettingsType::TurnOfSFX => {
                if line.unwrap() == "true" {
                    settings.turn_on_sfx = true;
                    counter += 1;
                    continue;
                }
                settings.turn_on_sfx = false;
            }
            SettingsType::MusicVolume => {
                settings.music_volume = line.unwrap().parse::<f32>().unwrap() / 100.;
            }
            SettingsType::SFXVolume => {
                settings.sfx_volume = line.unwrap().parse::<f32>().unwrap() / 100.;
            }
            SettingsType::SFXMetroVolume => {
                settings.metro_sfx_volume = line.unwrap().parse::<f32>().unwrap() / 100.;
            }
            SettingsType::TurnOfMetroSFX => {
                if line.unwrap() == "true" {
                    settings.turn_on_metro_sfx = true;
                    counter += 1;
                    continue;
                }
                settings.turn_on_metro_sfx = false;
            }
            _ => {
                panic!("bad file");
            }
        }
        counter += 1;
    }
    println!(
        "settings - {} {} {} {} {}",
        settings.turn_on_sfx,
        settings.turn_on_metro_sfx,
        settings.music_volume,
        settings.sfx_volume,
        settings.metro_sfx_volume
    );
}

fn change_settings(
    mut settings: ResMut<Settings>,
    slider_q: Query<&Slider>,
    mut change_settings_ev: EventReader<ChangeSettingEvent>,
    mut save_settings: EventWriter<SaveSettingEvent>,
    mut music_q: Query<&mut AudioSink, With<Soundtrack>>,
) {
    for _ev in change_settings_ev.read() {
        save_settings.send(SaveSettingEvent);
        for i in slider_q.iter() {
            match i.setting_type {
                SettingsType::MusicVolume => {
                    settings.music_volume = i.value;
                }
                SettingsType::SFXVolume => {
                    settings.sfx_volume = i.value;
                }
                _ => {
                    settings.metro_sfx_volume = i.value;
                }
            }
        }
        for mut volume in music_q.iter_mut() {
            volume.set_volume(settings.music_volume);
        }

    }
}
fn music_volume_change(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut settings: ResMut<Settings>,
    mut music_q: Query<&mut AudioSink, With<Soundtrack>>,
) {
    if keyboard.just_pressed(KeyCode::Equal) {
        if settings.music_volume < 1.0 {
            settings.music_volume += 0.1;
            for mut volume in music_q.iter_mut() {
                volume.set_volume(settings.music_volume);
            }
        }
    }
    if keyboard.just_pressed(KeyCode::Minus) {
        if settings.music_volume > 0. {
            settings.music_volume -= 0.1;
            for mut volume in music_q.iter_mut() {
                volume.set_volume(settings.music_volume);
            }
        }
    }
}

fn turn_off_music(keyboard: Res<ButtonInput<KeyCode>>, mut settings: ResMut<Settings>) {
    if keyboard.just_pressed(KeyCode::KeyK) {
        settings.turn_on_sfx = !settings.turn_on_sfx;
    }
    if keyboard.just_pressed(KeyCode::KeyL) {
        settings.turn_on_metro_sfx = !settings.turn_on_metro_sfx;
    }
}
