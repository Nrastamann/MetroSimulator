use bevy::prelude::*;

mod utils;

use bevy_2d_line::LineRenderingPlugin;
use bevy_lunex::{UiLunexDebugPlugin, UiLunexPlugins};

mod camera;
use camera::CameraPlugin;

mod map_generation;
use line::MetroLinePlugin;
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

mod passenger;
use passenger::PassengerPlugin;

const DISTRICT_CELL_SIZE: f32 =  50.;
const MAX_DISTRICT_SIZE: usize = 24;

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
        .add_plugins(CursorPlugin)
        .add_plugins(MapGenerationPlugin)
        .init_state::<GameState>()
        .add_plugins(LineRenderingPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins((StationPlugin, StationBlueprintPlugin))
        .add_plugins(MetroPlugin)
        .add_plugins(MetroLinePlugin)
        .add_plugins(TrainPlugin)
        .add_plugins((MainMenuPlugin, StationUIPlugin,UiLunexDebugPlugin::<0, 0>))
        .add_plugins(DistrictPlugin)
        .add_plugins(PassengerPlugin)
        .run();
}
