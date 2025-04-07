use bevy::prelude::*;

use crate::{line::Line, station::Station, utils::graph::Graph};

pub struct MetroPlugin;

impl Plugin for MetroPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Metro>();        
        app.add_systems(Update, draw_curves);
    }
}

#[derive(Default, Resource)]
pub struct Metro {
    pub stations: Graph<Station>,
    pub lines: Vec<Line>,
}

impl Metro {
    pub fn add_line(&mut self, points: Vec<(i32,i32)>) -> &mut Line {
        let id = self.lines.len();
        let line = Line::new_from_points(id, points);
        self.lines.push(line);
        &mut self.lines[id]
    }
}

#[derive(Default, PartialEq, Copy, Clone)]
pub enum Direction {
    #[default]
    Forwards,
    Backwards
}

// todo: переписать, чтобы избавиться от Gizmos
fn draw_curves( // рисуем линии
    metro: Res<Metro>,
    mut gizmos: Gizmos
) {
    for line in metro.lines.iter() {
        let Some(ref curve) = line.curve else { continue };
        let resolution = 100 * curve.segments().len();
        gizmos.linestrip(
            curve.iter_positions(resolution).map(|pt| pt.extend(0.0)),
            line.color
        );
    }
}