use bevy::prelude::*;

use crate::{metro::Metro, station::SpawnStationEvent, train::SpawnTrainEvent};

pub struct MapGenerationPlugin;

impl Plugin for MapGenerationPlugin{
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_map);
    }
}

fn spawn_map(
    mut metro: ResMut<Metro>,
    mut ev_spawn_station: EventWriter<SpawnStationEvent>,
    mut ev_spawn_train: EventWriter<SpawnTrainEvent>,
) {
    let line = metro.add_line(vec![]);

    let pos1 = (1 as i32 * 100, (1 as f32).powi(2).floor() as i32 * 20);
    let pos2 = (2 as i32 * 100, (2 as f32).powi(2).floor() as i32 * 20);
    line.push(pos1);
    line.push(pos2);
    ev_spawn_station.send(SpawnStationEvent { position: pos1, connection: pos2, color: line.color });
    ev_spawn_station.send(SpawnStationEvent { position: pos2, connection: pos1, color: line.color });

    ev_spawn_train.send(SpawnTrainEvent { line: 0, color: line.color });
}