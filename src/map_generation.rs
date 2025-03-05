use bevy::prelude::*;

use crate::station::Station;

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
) {
    for i in 0..5 {
        commands.spawn((
            Mesh2d(meshes.add(Circle::new(25.))),
            MeshMaterial2d(materials.add(Color::hsl(20., 0.5, 0.5))),
            Transform::from_xyz(i as f32 * 100., (i as f32).powi(2) * 20., 0.),
            Station
        ));
    }
}