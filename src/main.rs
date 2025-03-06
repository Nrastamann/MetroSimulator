use bevy::prelude::*;

mod camera;
use camera::CameraPlugin;

mod map_generation;
use map_generation::MapGenerationPlugin;

mod station;
use station::StationPlugin;

mod line;

mod metro;
use metro::MetroPlugin;

mod cursor;
use cursor::CursorPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(CameraPlugin)
        .add_plugins(CursorPlugin)
        .add_plugins(MapGenerationPlugin)
        .add_plugins(StationPlugin)
        .add_plugins(MetroPlugin)
        .run();
}