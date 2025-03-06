use bevy::prelude::*;

use crate::{cursor::CursorPosition, metro::Metro};

pub struct StationPlugin;

impl Plugin for StationPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<StationBuilder>();
        app.add_event::<SpawnStationEvent>();
        app.add_systems(Update, (hover_select, build_new, spawn_station));
    }
}

#[derive(Component, Copy, Clone, PartialEq)]
pub struct Station {
    pub position: Vec2,
    pub selected: bool,
}

#[derive(Event)]
pub struct SpawnStationEvent {
    pub position: Vec2,
    pub station: Station,
}

fn spawn_station(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut ev_spawn_station: EventReader<SpawnStationEvent>,
) {
    for ev in ev_spawn_station.read() {
        commands.spawn((
            Mesh2d(meshes.add(Circle::new(25.))),
            MeshMaterial2d(materials.add(Color::hsl(20., 0.5, 0.5))),
            Transform::from_translation(ev.position.extend(0.0)),
            ev.station
        ));
    }
}

fn hover_select(
    mut stations: Query<(&mut Transform, &mut Station)>,
    cursor_position: Res<CursorPosition>,
) {
    for (mut station_transform, mut station) in stations.iter_mut() {
        if station_transform.translation.truncate().distance(cursor_position.0) < 25. {
            station_transform.scale = Vec3::splat(1.25);
            station.selected = true;
        }
        else {
            station_transform.scale = Vec3::splat(1.0);
            station.selected = false;
        }
    }
}

#[derive(Default, Resource)]
struct StationBuilder {
    has_selected: bool,
    attach_to_line: usize,
    place: usize
}

fn build_new(
    stations: Query<&Station>,
    mut metro: ResMut<Metro>,
    cursor_position: Res<CursorPosition>,
    mouse: Res<ButtonInput<MouseButton>>,
    mut builder: ResMut<StationBuilder>,
    mut ev_spawn_station: EventWriter<SpawnStationEvent>,
) {
    if mouse.just_pressed(MouseButton::Left) {
        for station in stations.iter() {
            if station.selected {
                for i in 0..metro.lines.len() {
                    if let Some(place) = metro.lines[i].stations.iter().position(|s| s.position == station.position) {
                        builder.attach_to_line = i;
                        builder.place = place;
                        builder.has_selected = true;
                        break;
                    }
                }
                break;
            }
        }
    }

    if mouse.just_released(MouseButton::Left) && builder.has_selected {
        let station = Station {
            position: cursor_position.0,
            selected: false
        };

        metro.lines[builder.attach_to_line].add_station(
            station,
            builder.place
        );

        ev_spawn_station.send(SpawnStationEvent {
            position: cursor_position.0,
            station
        });

        builder.has_selected = false;
    }
}