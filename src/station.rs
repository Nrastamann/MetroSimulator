use bevy::prelude::*;

use crate::{cursor::CursorPosition, metro::Metro, station_blueprint::SetBlueprintColorEvent};

pub struct StationPlugin;

impl Plugin for StationPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<StationBuilder>();
        app.add_event::<SpawnStationEvent>();
        app.add_systems(Update, (hover_select, check_building_position, build_new, spawn_station));
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
    pub color: Color,
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
            MeshMaterial2d(materials.add(ev.color)),
            Transform::from_translation(ev.position.extend(0.0)),
            ev.station
        ));
    }
}

fn hover_select( // просто выделение при наведении на станцию
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

#[derive(Default)]
enum BuildingMode {
    #[default]
    Prolong,
    NewLine,
}

#[derive(Default, Resource)]
struct StationBuilder { // todo: приудмать, как это переделать, чтобы было не так убого
    is_building: bool,
    building_mode: BuildingMode,
    line_to_attach_to: usize,
    place: usize,
    parent_station: Option<Station>,
    is_position_allowed: bool,
}

fn build_new(
    stations: Query<&Station>,
    mut metro: ResMut<Metro>,
    cursor_position: Res<CursorPosition>,
    mouse: Res<ButtonInput<MouseButton>>,
    mut builder: ResMut<StationBuilder>,
    mut ev_spawn_station: EventWriter<SpawnStationEvent>,
    mut ev_set_blueprint: EventWriter<SetBlueprintColorEvent>,
) {
    if mouse.just_pressed(MouseButton::Left) { // начинаем строить, определяем, будет это продолжение старой ветки или создание новой
        for station in stations.iter() {
            if station.selected {
                for i in 0..metro.lines.len() {
                    if let Some(place) = metro.lines[i].stations.iter().position(|s| s.position == station.position) {
                        builder.line_to_attach_to = i;
                        builder.place = place;
                        builder.is_building = true;
                        builder.parent_station = Some(*station);
                        if place == 0 || place == metro.lines[i].stations.len()- 1 {
                            builder.building_mode = BuildingMode::Prolong;
                        }
                        else {
                            builder.building_mode = BuildingMode::NewLine;
                        }
                        ev_set_blueprint.send(SetBlueprintColorEvent(Color::BLACK.with_alpha(0.5))); // todo: make color match the line 
                        break;
                    }
                }
                break;
            }
        }
    }

    if mouse.just_released(MouseButton::Left)
    && builder.is_building { // строим
        if builder.is_position_allowed {
            let station = Station {
                position: cursor_position.0,
                selected: false
            };
    
            let color; 
    
            match builder.building_mode {
                BuildingMode::NewLine => {
                    metro.add_line(vec![builder.parent_station.unwrap(), station]);
                    color = metro.lines[metro.lines.len()-1].color;
                },
                BuildingMode::Prolong => {
                    let place = builder.place;
                    let line = &mut metro.lines[builder.line_to_attach_to];
                    if place == line.stations.len() - 1 {
                        line.push_back(station);
                    }
                    else if place == 0 {
                        line.push_front(station);
                    }
    
                    color = line.color;
                }
            }
    
            ev_spawn_station.send(SpawnStationEvent {
                position: cursor_position.0,
                station,
                color
            });
        }

        builder.is_building = false;
        builder.parent_station = None;
        ev_set_blueprint.send(SetBlueprintColorEvent(Color::BLACK.with_alpha(0.0)));
    }
}

fn check_building_position(
    cursor_position: Res<CursorPosition>,
    q_stations: Query<(&Transform, &Station)>,
    mut builder: ResMut<StationBuilder>,
    mut ev_set_blueprint: EventWriter<SetBlueprintColorEvent>,
) {
    if q_stations.iter().len() <= 0 {
        return;
    }

    let sorted: Vec<(&Transform, &Station)> = q_stations.iter()
        .sort_by::<&Transform>(|t1, t2| {
            t1.translation.distance(cursor_position.0.extend(0.0))
                .total_cmp(&t2.translation.distance(cursor_position.0.extend(0.0)))
        }).collect();

    let (closest, _) = sorted[0];

    if closest.translation.distance(cursor_position.0.extend(0.0)) <= 100.0 {
        builder.is_position_allowed = false;
        if builder.is_building {
            ev_set_blueprint.send(SetBlueprintColorEvent(Color::srgba(1.0, 0.0, 0.0, 0.5)));
        }
    }
    else {
        builder.is_position_allowed = true;
        if builder.is_building {
            ev_set_blueprint.send(SetBlueprintColorEvent(Color::BLACK.with_alpha(0.5)));
        }
    }
}