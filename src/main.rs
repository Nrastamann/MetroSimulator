use bevy::prelude::*;

mod camera;
use camera::CameraPlugin;

mod map_generation;
use map_generation::MapGenerationPlugin;

mod station;
use station::StationPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(CameraPlugin)
        .add_plugins(MapGenerationPlugin)
        .add_plugins(StationPlugin)
        .run();
}