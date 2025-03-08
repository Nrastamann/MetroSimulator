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
    let mut stations: Vec<Station> = vec![];

    for i in 0..5 {
        let position = Vec2::new(i as f32 * 100., (i as f32).powi(2) * 20.);
        let station = Station {
            position,
            selected: false
        };

        let color = Color::hsl(50., 0.5, 0.5);

        ev_spawn_station.send(SpawnStationEvent { position, station, color });
        stations.push(station);
    }

    metro.add_line(stations);
}