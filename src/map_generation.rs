use bevy::prelude::*;

use crate::{metro::Metro, station::{SpawnStationEvent, Station}};

pub struct MapGenerationPlugin;

impl Plugin for MapGenerationPlugin{
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_map);
    }
}

fn spawn_map(
    mut metro: ResMut<Metro>,
    mut ev_spawn_station: EventWriter<SpawnStationEvent>,
) {
    let line = metro.add_line(vec![]);

    for i in -1..2 {
        let position = Vec2::new(i as f32 * 100., (i as f32).powi(2) * 20.);
        let station = Station {
            position,
            selected: false
        };

        line.push((position.x.floor() as i32, position.y.floor() as i32));
        ev_spawn_station.send(SpawnStationEvent { position, station, color: line.color });
    }
}