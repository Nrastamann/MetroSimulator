use std::collections::LinkedList;

use bevy::prelude::*;
use rand::prelude::*;

use crate::{metro::Metro, station::Station};

pub struct MetroLinePlugin;

impl Plugin for MetroLinePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnLineCurveEvent>();
        app.add_event::<UpdateLineRendererEvent>();
        app.add_systems(Update, (spawn_line_curve, update_line_renderer));
    }
}

#[derive(PartialEq)]
pub struct MetroLine {
    pub name: String,
    pub id: usize,
    pub stations: LinkedList<Station>,
    pub curve: Option<CubicCurve<Vec2>>,
    pub color: Color,
}

impl MetroLine {
    fn update_curve(&mut self) { // обновляем точки, по которым строится кривая
        self.curve = CubicCardinalSpline::new_catmull_rom(self.stations
            .iter().map(|station| Vec2::new(station.position.0 as f32, station.position.1 as f32)).collect::<Vec<Vec2>>())
            .to_curve().ok();
    }

    pub fn new_from_points(id: usize, new_points: Vec<(i32, i32)>) -> Self { // новая ветка из вектора станций
        let curve = CubicCardinalSpline::new_catmull_rom(new_points
            .iter().map(|&(x,y)| Vec2::new(x as f32, y as f32)).collect::<Vec<Vec2>>())
            .to_curve().ok();
        
        let mut rng = rand::rng();    

        let mut stations = LinkedList::new();
        for point in new_points.iter() {
            stations.push_back(Station::new(*point));
        }

        Self {
            name: LINE_NAMES[rand::rng().random_range(0..9)].to_string(),
            id,
            stations: LinkedList::new(),
            points: new_points,
            id,
            stations,
            curve,
            color: Color::hsl(rng.random_range(0..=12) as f32 * 30., 0.5, 0.5)
        }
    }

    pub fn push_back(&mut self, point: (i32, i32)) {
        self.stations.push_back(Station::new(point));
        self.update_curve();
    }

    pub fn push_front(&mut self, point: (i32, i32)) {
        self.stations.push_front(Station::new(point));
        self.update_curve();
    }
}

#[derive(Component)]
struct LineRenderer {
    line_id: usize,
}

#[derive(Event)]
pub struct SpawnLineCurveEvent {
    pub line_id: usize,
}

fn spawn_line_curve(
    mut commands: Commands,
    metro: Res<Metro>,
    mut ev_spawn_line: EventReader<SpawnLineCurveEvent>,
) {
    for ev in ev_spawn_line.read() {
        let line = &metro.lines[ev.line_id];
        let Some(ref curve) = line.curve else { continue };
        let resolution = 100 * curve.segments().len();
        let points = curve.iter_positions(resolution).collect::<Vec<Vec2>>();

        let mut colors = vec![];
        for _ in 0..points.len() {
            colors.push(line.color.into());
        }
        commands.spawn((
            bevy_2d_line::Line {
                points,
                colors,
                thickness: 5.0
            },
            LineRenderer { line_id: line.id }
        ));        
    }
}

#[derive(Event)]
pub struct UpdateLineRendererEvent{
    pub line_id: usize
}

fn update_line_renderer(
    metro: Res<Metro>,
    mut ev_update_line: EventReader<UpdateLineRendererEvent>,
    mut q_line_renderer: Query<(&mut bevy_2d_line::Line, &LineRenderer)>,
) {
    for ev in ev_update_line.read() {
        let (mut line, _) = q_line_renderer.iter_mut().filter(|(_, renderer)| renderer.line_id == ev.line_id).next().unwrap();
        let line_data = &metro.lines[ev.line_id];
        let Some(ref curve) = line_data.curve else { continue };
        let resolution = 100 * curve.segments().len();

        line.points = curve.iter_positions(resolution).collect::<Vec<Vec2>>();
        while line.colors.len() < line.points.len() {
            line.colors.push(line_data.color.into());
        }
    }
}