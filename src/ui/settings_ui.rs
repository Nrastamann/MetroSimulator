use crate::{audio::ChangeTrackEvent, GameState};
use bevy::prelude::*;
use bevy_lunex::*;

use super::TutorialSpawnEvent;
pub struct SettingsUIPlugin;

impl Plugin for SettingsUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Settings), Settings::spawn);
    }
}
#[derive(Component)]
pub struct Settings;
impl Settings {
    fn spawn(mut commands: Commands) {
        commands.spawn((UiLayoutRoot::new_2d(), UiFetchFromCamera::<0>, Settings)).with_children(|ui|{
            ui.spawn((
                Name::new("Settings"),
                UiLayoutTypeWindow::new().full().pack(),
                StateScoped(GameState::Settings),
                Sprite::new(),//add some background to settings, make it blur?
            )).with_children(|ui|{
                ui.spawn((
                    Name::new("Pick settings"),
                    UiLayoutTypeWindow::new().anchor_left().rl_size(100.,20.).pack(),
                )).with_children(|ui|{
                    ui.spawn((
                        Name::new("")
                    ));
                });
            });
        });
    }
}