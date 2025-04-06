use bevy::prelude::*;

mod utils;

use bevy_lunex::{UiLunexDebugPlugin, UiLunexPlugins};

mod camera;
use camera::CameraPlugin;

mod map_generation;
use map_generation::MapGenerationPlugin;

mod station;
use station::StationPlugin;

mod station_blueprint;
use station_blueprint::StationBlueprintPlugin;

mod ui;
use ui::{MainMenuPlugin, StationUIPlugin};

mod line;

mod metro;
use metro::MetroPlugin;

mod train;
use train::TrainPlugin;

mod cursor;
use cursor::CursorPlugin;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum GameState {
    #[default]
    MainMenu,
    InGame,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_state::<GameState>()
        .add_plugins(CameraPlugin)
        .add_plugins(CursorPlugin)
        .add_plugins(MapGenerationPlugin)
        .add_plugins((StationPlugin, StationBlueprintPlugin))
        .add_plugins(MetroPlugin)
        .add_plugins(TrainPlugin)
        .add_plugins(UiLunexPlugins)
        .add_plugins((MainMenuPlugin, StationUIPlugin, //UiLunexDebugPlugin::<0, 0>
            ))
        .run();
}
