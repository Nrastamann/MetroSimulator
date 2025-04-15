use bevy::prelude::*;

use crate::{line::MetroLine, station::Station, utils::graph::Graph};

pub struct MetroPlugin;

impl Plugin for MetroPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Metro>();
        app.init_gizmo_group::<MetroLineGizmos>();
        app.add_systems(Startup, config_gizmos);
    }
}

#[derive(Default, Resource)]
pub struct Metro {
    pub stations: Graph<Station>,
    pub lines: Vec<MetroLine>,
}

impl Metro {
    pub fn add_line(&mut self, points: Vec<(i32, i32)>) -> &mut MetroLine {
        let id = self.lines.len();
        let line = MetroLine::new_from_points(id, points);
        self.lines.push(line);
        &mut self.lines[id]
    }
    pub fn find_line_by_station(&mut self, station_id: (i32, i32)) -> Option<&MetroLine> {
        self.lines
            .iter()
            .filter(|line| {
                line.stations
                    .iter()
                    .filter(|station| station.position == station_id)
                    .next()
                    .is_some()
            })
            .next()
    }
    pub fn find_station(&mut self, station_id: (i32, i32)) -> Option<&Station> {
        self.find_line_by_station(station_id).unwrap().stations.iter().filter(|station| station.position == station_id).next()
    }
}

#[derive(Default, PartialEq, Copy, Clone)]
pub enum Direction {
    #[default]
    Forwards,
    Backwards,
}

#[derive(Default, Reflect, GizmoConfigGroup)]
struct MetroLineGizmos {}

fn config_gizmos(mut config_store: ResMut<GizmoConfigStore>) {
    let (config, _) = config_store.config_mut::<MetroLineGizmos>();
    config.line_width = 5.;
}



#[derive(Default, Reflect, GizmoConfigGroup)]
struct MetroLineGizmos {}

fn config_gizmos(
    mut config_store: ResMut<GizmoConfigStore>,
) {
    let (config, _) = config_store.config_mut::<MetroLineGizmos>();
    config.line_width = 5.;
}
