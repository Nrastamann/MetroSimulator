use bevy::prelude::*;

use crate::{metro::Metro, station::Station};

pub struct MapGenerationPlugin;

impl Plugin for MapGenerationPlugin{
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_map);
    }
}

fn spawn_map(
    mut commands: Commands, 
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut metro: ResMut<Metro>,
) {
    let mut stations: Vec<Station> = vec![];

    for i in 0..5 {
        let station = Station { position: Vec2::new(i as f32 * 100., (i as f32).powi(2) * 20.) };

        commands.spawn((
            Mesh2d(meshes.add(Circle::new(25.))),
            MeshMaterial2d(materials.add(Color::hsl(20., 0.5, 0.5))),
            Transform::from_xyz(i as f32 * 100., (i as f32).powi(2) * 20., 0.),
            station.clone()
        ));

        stations.push(station);
    }

    metro.add_line(stations);
}