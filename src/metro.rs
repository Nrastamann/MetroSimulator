use bevy::prelude::*;

use crate::{line::Line, station::Station};

pub struct MetroPlugin;

impl Plugin for MetroPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Metro>();        
        app.add_systems(Update, draw_curves);
    }
}

#[derive(Default, Resource)]
pub struct Metro {
    pub lines: Vec<Line>,
}

impl Metro {
    pub fn add_line(&mut self, stations: Vec<Station>) {
        self.lines.push(Line::new_from_stations(stations));
    }
}

fn draw_curves(
    metro: Res<Metro>,
    mut gizmos: Gizmos
) {
    for line in metro.lines.iter() {
        let Some(ref curve) = line.curve else { continue };
        let resolution = 100 * curve.segments().len();
        gizmos.linestrip(
            curve.iter_positions(resolution).map(|pt| pt.extend(0.0)),
            Color::hsl(20., 0.5, 0.5)
        );
    }
}