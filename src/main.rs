use bevy::prelude::*;

mod utils;

mod camera;
use camera::CameraPlugin;

mod map_generation;
use map_generation::MapGenerationPlugin;

mod station;
use station::StationPlugin;

mod station_blueprint;
use station_blueprint::StationBlueprintPlugin;

mod line;

mod metro;
use metro::MetroPlugin;

mod train;
use train::TrainPlugin;

mod cursor;
use cursor::CursorPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(CameraPlugin)
        .add_plugins(CursorPlugin)
        .add_plugins(MapGenerationPlugin)
        .add_plugins((StationPlugin, StationBlueprintPlugin))
        .add_plugins(MetroPlugin)
        .add_plugins(TrainPlugin)
        .run();
}