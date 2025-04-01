use std::string;

use bevy::prelude::*;
use rand::Rng;

use crate::{cursor::CursorPosition, metro::Metro, station_blueprint::SetBlueprintColorEvent, train::SpawnTrainEvent,GameState};

pub const STATION_NAMES: [&str; 10] = ["Достоевская","Обводный канал","Озерки","Парнас","Динамо","Автово","Сенная площадь","Купчино","Дыбенко","Звездная"];

pub struct StationPlugin;

impl Plugin for StationPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<StationBuilder>();
        app.add_event::<SpawnStationEvent>();
        app.add_systems(Update, (hover_select, check_building_position, build_new, spawn_station).run_if(in_state(GameState::InGame)));
    }
}

#[derive(Component, Clone, PartialEq)]
pub struct Station {
    pub id: (i32, i32),
    pub selected: bool,
    pub meshes: Vec<Handle<Mesh>>,
    pub materials: Vec<Handle<ColorMaterial>>,
    pub name: String,
}

#[derive(Event)]
pub struct SpawnStationEvent {
    pub position: (i32, i32),
    pub connection: (i32, i32),
    pub color: Color,
}

fn spawn_station(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut ev_spawn_station: EventReader<SpawnStationEvent>,
    mut metro: ResMut<Metro>,
) {
    for ev in ev_spawn_station.read() {
        let mut station = Station {
            name: STATION_NAMES[rand::rng().random_range(0..9)].to_string(),
            id: ev.position,
            selected: false,
            meshes: vec![],
            materials: vec![],
        };

        let mesh = meshes.add(Circle::new(25.));
        let material = materials.add(ev.color);

        station.meshes.push(mesh.clone());
        station.materials.push(material.clone());

        metro.stations.add(ev.connection, ev.position, station.clone());

        commands.spawn((
            Mesh2d(mesh),
            MeshMaterial2d(material),
            Transform::from_translation(Vec3::new(
                ev.position.0 as f32,
                ev.position.1 as f32, 0.0
            )),
            station
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
enum BuilderAction {
    Build,
    Connect { closest: (i32, i32) },
    #[default]
    Nothing
}

#[derive(Default, Resource)]
struct StationBuilder { // todo: приудмать, как это переделать, чтобы было не так убого
    is_building: bool,
    line_to_attach_to: usize,
    place: usize,
    connection: (i32, i32),
    action: BuilderAction 
}

fn build_new(
    stations: Query<&Station>,
    mut metro: ResMut<Metro>,
    cursor_position: Res<CursorPosition>,
    mouse: Res<ButtonInput<MouseButton>>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut builder: ResMut<StationBuilder>,
    mut ev_spawn_station: EventWriter<SpawnStationEvent>,
    mut ev_set_blueprint: EventWriter<SetBlueprintColorEvent>,
    mut ev_spawn_train: EventWriter<SpawnTrainEvent>,
) {
    if mouse.just_pressed(MouseButton::Left) { // начинаем строить, определяем, будет это продолжение старой ветки или создание новой
        for station in stations.iter() {
            if station.selected {
                for i in 0..metro.lines.len() {
                    if let Some(place) = metro.lines[i].points.iter().position(|s| *s == station.id) {
                        builder.line_to_attach_to = i;
                        builder.place = place;
                        builder.is_building = true;
                        builder.connection = station.id;
                        ev_set_blueprint.send( // todo: make color match the line 
                            SetBlueprintColorEvent(
                                Color::BLACK.with_alpha(0.5)
                            )
                        );
                        break;
                    }
                }
                break;
            }
        }
    }

    if mouse.just_released(MouseButton::Left)
    && builder.is_building { // строим
        match builder.action {
            BuilderAction::Build => {
                let color; 
                let id = (cursor_position.0.x.floor() as i32,
                                      cursor_position.0.y.floor() as i32);
                
                if keyboard.pressed(KeyCode::ShiftLeft) {
                    metro.add_line(vec![id, builder.connection]);
                    color = metro.lines[metro.lines.len()-1].color;
                    ev_spawn_train.send(SpawnTrainEvent { line: metro.lines.len()-1, color });
                }
                else {
                    let place = builder.place;
                    let line = &mut metro.lines[builder.line_to_attach_to];
                    if place == line.points.len() - 1 {
                        line.push(id);
                    }
                    else {
                       line.insert(place, id);
                    }
    
                    color = line.color;
                }
            
                ev_spawn_station.send(SpawnStationEvent {
                    position: cursor_position.as_tuple(),
                    color,
                    connection: builder.connection,
                });
            },
            BuilderAction::Connect { closest } => {
                let place = builder.place;
                let line = &mut metro.lines[builder.line_to_attach_to];
                if place == line.points.len() - 1 {
                    line.push(closest);
                }
                else {
                   line.insert(place, closest);
                }
            },
            _ => {}
        }

        builder.is_building = false;
        ev_set_blueprint.send(SetBlueprintColorEvent(Color::BLACK.with_alpha(0.0)));
    }
}

fn check_building_position(
    cursor_position: Res<CursorPosition>,
    q_stations: Query<(&Transform, &Station)>,
    mut builder: ResMut<StationBuilder>,
    mut ev_set_blueprint: EventWriter<SetBlueprintColorEvent>,
    metro: Res<Metro>,
) {
    if q_stations.iter().len() <= 0 {
        return;
    }

    let sorted: Vec<(&Transform, &Station)> = q_stations.iter()
        .sort_by::<&Transform>(|t1, t2| {
            t1.translation.distance(cursor_position.0.extend(0.0))
                .total_cmp(&t2.translation.distance(cursor_position.0.extend(0.0)))
        }).collect();

    let (closest_transform, closest_station) = sorted[0];

    if closest_transform.translation.distance(cursor_position.0.extend(0.0)) <= 100.0 {
        let color: Color;

        if metro.lines[builder.line_to_attach_to].points.contains(&closest_station.id) {
            builder.action = BuilderAction::Nothing;
            color = Color::srgba(1.0, 0.0, 0.0, 0.5);
        }
        else {
            builder.action = BuilderAction::Connect { closest: closest_station.id };
            color = Color::BLACK.with_alpha(0.5);
        }

        if builder.is_building {
            ev_set_blueprint.send(SetBlueprintColorEvent(color));
        }
    }
    else {
        builder.action = BuilderAction::Build;
        if builder.is_building {
            ev_set_blueprint.send(SetBlueprintColorEvent(Color::BLACK.with_alpha(0.5)));
        }
    }
}