use bevy::prelude::*;

mod utils;

use bevy_lunex::UiLunexPlugins;

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

mod district;
use district::DistrictPlugin;

const DISTRICT_CELL_SIZE: f32 =  25.;
const MAX_DISTRICT_SIZE: usize = 60;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum GameState {
    #[default]
    MainMenu,
    InGame,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(UiLunexPlugins)
        .init_state::<GameState>()
        .add_plugins(CameraPlugin)
        .add_plugins(CursorPlugin)
        .add_plugins(MapGenerationPlugin)
        .add_plugins((StationPlugin, StationBlueprintPlugin))
        .add_plugins(MetroPlugin)
        .add_plugins(TrainPlugin)
        .add_plugins((MainMenuPlugin, StationUIPlugin))
        .add_plugins(DistrictPlugin)
        .run();
}
