use bevy::prelude::*;

use crate::{line::SpawnLineCurveEvent, metro::Metro, station::SpawnStationEvent, train::SpawnTrainEvent, GameState};

pub struct MapGenerationPlugin;

impl Plugin for MapGenerationPlugin{
    fn build(&self, app: &mut App) {
        app.add_systems(OnExit(GameState::MainMenu), spawn_map);//should i change it to startup back, so it generates events at startup?
    }
}

fn spawn_map(
    mut metro: ResMut<Metro>,
    mut ev_spawn_station: EventWriter<SpawnStationEvent>,
    mut ev_spawn_train: EventWriter<SpawnTrainEvent>,
    mut ev_spawn_line: EventWriter<SpawnLineCurveEvent>,
) {
    let line = metro.add_line(vec![]);

    let pos1 = (0, 0);
    let pos2 = (100, 0);
    line.push_back(pos1);
    line.push_back(pos2);

    ev_spawn_line.send(SpawnLineCurveEvent { line_id: 0 });

    ev_spawn_station.send(SpawnStationEvent { position: pos1, connection: pos2 });
    ev_spawn_station.send(SpawnStationEvent { position: pos2, connection: pos1 });

    ev_spawn_train.send(SpawnTrainEvent { line: 0, color: line.color });
}