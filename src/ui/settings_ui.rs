use crate::{audio::ChangeTrackEvent, GameState};
use bevy::prelude::*;
use bevy_lunex::*;

const SETTINGS_NAME: [&str; 3] = ["Включить", "Б", "С"];
const SETTING_SIZE: f32 = 11.;
const SETTINGS_OFFSET: f32 = 15.;

use super::{
    TutorialSpawnEvent, UIStyles, METRO_LIGHT_BLUE_COLOR, OPACITY_LEVEL_BLUR, OPACITY_LEVEL_HIGHEST, OPACITY_LEVEL_MAIN
};
pub struct SettingsUIPlugin;

impl Plugin for SettingsUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Settings), Settings::spawn);
    }
}
#[derive(Component)]
pub struct Settings;
impl Settings {
    fn spawn(mut commands: Commands, asset_server: Res<AssetServer>) {
        commands
            .spawn((UiLayoutRoot::new_2d(), UiFetchFromCamera::<0>, Settings))
            .with_children(|ui| {
                ui.spawn((
                    Name::new("Settings BG"),
                    UiLayoutTypeWindow::new().full().pack(),
                    StateScoped(GameState::Settings),
                    Sprite {
                        image: asset_server.load("Settings.png"),
                        ..default()
                    }, //add some background to settings, make it blur?
                ))
                .with_children(|ui| {
                    ui.spawn((
                        Name::new("Settings"),
                        UiLayoutTypeWindow::new().full().pack(),
                        Sprite::default(),
                        UiColor::from(METRO_LIGHT_BLUE_COLOR.with_alpha(OPACITY_LEVEL_HIGHEST)),
                    ))
                    .with_children(|ui| {
                        ui.spawn((
                            Name::new("Settings background"),
                            UiLayoutTypeWindow::new()
                                .anchor_left()
                                .rl_pos(10., 10.)
                                .rl_size(80., 80.)
                                .pack(),
                            Sprite::default(),
                            UiColor::from(Color::WHITE.with_alpha(OPACITY_LEVEL_MAIN)),
                        ))
                        .with_children(|ui| {
                            ui.spawn((
                                Name::new("Text part"),
                                UiLayoutTypeWindow::new()
                                    .anchor_left()
                                    .rl_size(80., 100.)
                                    .pack(),
                            ))
                            .with_children(|ui| {
                                let mut current_offset = 0.;
                                for i in SETTINGS_NAME {
                                    
                                }
                            });
                            ui.spawn((
                                Name::new("Settings set part"),
                                UiLayoutTypeWindow::new()
                                    .anchor_left()
                                    .x(Rl(80.))
                                    .rl_size(20., 100.)
                                    .pack(),
                            ))
                            .with_children(|ui| {

                            });
                        });
                    });
                });
            });
    }
}
