use std::{f32::consts::PI, usize};

use bevy::prelude::*;
use rand::Rng;

use crate::{
    cursor::CursorPosition,
    line::{SpawnLineCurveEvent, UpdateLineRendererEvent},
    metro::{Direction, Metro},
    money::Money,
    station_blueprint::{SetBlueprintColorEvent, StationBlueprint},
    train::SpawnTrainEvent,
    GameState,
};

pub const STATION_NAMES: [&str; 10] = [
    "Достоевская",
    "Обводный канал",
    "Озерки",
    "Парнас",
    "Зенит",
    "Автово",
    "Сенная площадь",
    "Купчино",
    "Дыбенко",
    "Звездная",
];

const STATION_COST: u32 = 100;
pub const STATION_MAX_PASSENGERS: u32 = 12;

pub struct StationPlugin;

impl Plugin for StationPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnStationEvent>()
            .add_event::<BuildStationEvent>()
            .add_event::<StartBuildingEvent>();
        app.add_systems(
            Update,
            (
                hover_select,
                check_building_position,
                build_new,
                spawn_station,
                build_station,
                debug_draw_passengers,
                detect_left_release,
            )
                .run_if(in_state(GameState::InGame)),
        );
    }
}

#[derive(Component, Clone, Copy, PartialEq)]
pub struct Station {
    pub position: (i32, i32),
}

impl Station {
    pub fn new(position: (i32, i32)) -> Self {
        Self { position }
    }
}

#[derive(Default, Component)]
pub struct StationButton {
    pub selected: bool,
    pub passenger_ids: Vec<usize>,
    pub name: String,
}

#[derive(Event)]
pub struct StartBuildingEvent {
    pub connection: (i32, i32),
    pub direction: Direction,
    pub line_to_attach: usize,
    pub from_menu: bool,
}

#[derive(Event)]
pub struct BuildStationEvent {
    pub position: (i32, i32),
    pub connection: (i32, i32),
    pub direction: Direction,
    pub line_to_attach: usize,
}
#[derive(Event)]
pub struct SpawnStationEvent {
    pub position: (i32, i32),
    pub connection: (i32, i32),
}

fn spawn_station(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut ev_spawn_station: EventReader<SpawnStationEvent>,
    mut metro: ResMut<Metro>,
) {
    for ev in ev_spawn_station.read() {
        let station = Station {
            position: ev.position,
        };

        let mesh = meshes.add(Circle::new(25.));
        let material = materials.add(Color::BLACK);

        let inner_circle = commands
            .spawn((
                Mesh2d(meshes.add(Circle::new(20.))),
                MeshMaterial2d(materials.add(Color::WHITE)),
                Transform::from_translation(Vec3::new(0.0, 0.0, 2.0)),
            ))
            .id();

        let mut button = StationButton::default();
        button.name = STATION_NAMES[rand::rng().random_range(0..10)].to_string();
        // println!("name - {}", button.name);

        metro
            .stations
            .add(ev.connection, ev.position, station.clone());
        commands
            .spawn((
                Mesh2d(mesh),
                MeshMaterial2d(material),
                Transform::from_translation(Vec3::new(
                    ev.position.0 as f32,
                    ev.position.1 as f32,
                    1.0,
                )),
                button,
                station,
            ))
            .add_child(inner_circle);
    }
}

fn debug_draw_passengers(q_station: Query<(&Transform, &StationButton)>, mut gizmos: Gizmos) {
    for (transform, station) in q_station.iter() {
        for i in 0..station.passenger_ids.len() {
            let position = transform.translation.truncate()
                + 40. * Vec2::from_angle((i as f32) * (2. * PI / (STATION_MAX_PASSENGERS as f32)));
            gizmos.circle_2d(Isometry2d::from_translation(position), 5., Color::BLACK);
        }
    }
}

fn hover_select(
    // просто выделение при наведении на станцию
    mut stations: Query<(&mut Transform, &mut StationButton)>,
    cursor_position: Res<CursorPosition>,
) {
    for (mut station_transform, mut station) in stations.iter_mut() {
        if station_transform
            .translation
            .truncate()
            .distance(cursor_position.0)
            < 25.
        {
            station_transform.scale = Vec3::splat(1.25);
            station.selected = true;
        } else {
            station_transform.scale = Vec3::splat(1.0);
            station.selected = false;
        }
    }
}

fn build_new(
    q_station: Query<(&Station, &StationButton)>,
    metro: Res<Metro>,
    mouse: Res<ButtonInput<MouseButton>>,
    mut ev_start_build: EventWriter<StartBuildingEvent>,
    mut ev_set_blueprint: EventWriter<SetBlueprintColorEvent>,
) {
    if mouse.just_pressed(MouseButton::Left) {
        let Some((selected_station, _)) = q_station.iter().filter(|(_, btn)| btn.selected).next()
        else {
            println!("a?");
            return;
        };

        // начинаем строить, определяем, будет это продолжение старой ветки или создание новой
        for line in metro.lines.iter() {
            if line.stations.contains(&selected_station) {
                let mut direction: Direction = Direction::Forwards;
                let mut line_id = line.id;
                // if keyboard.pressed(KeyCode::ShiftLeft) {
                //     line = -1;
                // }

                if line.stations.front().unwrap() == selected_station {
                    direction = Direction::Backwards;
                } else if line.stations.back().unwrap() != selected_station {
                    println!("Line is not front & isn't back");
                    line_id = usize::MAX;
                }
                ev_start_build.send(StartBuildingEvent {
                    connection: selected_station.position,
                    direction: direction,
                    line_to_attach: line_id,
                    from_menu: false,
                });
                ev_set_blueprint.send(
                    SetBlueprintColorEvent(Color::BLACK.with_alpha(0.5)),
                );
            }
        }
    }
}

fn build_station(
    mut ev_build_station: EventReader<BuildStationEvent>,
    mut metro: ResMut<Metro>,
    mut ev_spawn_station: EventWriter<SpawnStationEvent>,
    mut ev_update_line_renderer: EventWriter<UpdateLineRendererEvent>,
    mut ev_spawn_line: EventWriter<SpawnLineCurveEvent>,
    mut ev_spawn_train: EventWriter<SpawnTrainEvent>,
    mut money: ResMut<Money>,
) {
    for ev in ev_build_station.read() {
        if money.0 < STATION_COST {
            continue;
        }

        money.0 -= STATION_COST;

        match ev.line_to_attach {
            usize::MAX => {
                let line = metro.add_line(vec![ev.position, ev.connection]);
                ev_spawn_train.send(SpawnTrainEvent {
                    line: line.id,
                    station: ev.position,
                });

                ev_spawn_line.send(SpawnLineCurveEvent { line_id: line.id });

                ev_spawn_station.send(SpawnStationEvent {
                    position: ev.position,
                    connection: ev.connection,
                });
            }
            _ => {
                let line = &mut metro.lines[ev.line_to_attach];
                match ev.direction {
                    Direction::Forwards => line.push_back(ev.position),
                    Direction::Backwards => line.push_front(ev.position),
                }

                ev_update_line_renderer.send(UpdateLineRendererEvent { line_id: line.id });

                ev_spawn_station.send(SpawnStationEvent {
                    position: ev.position,
                    connection: ev.connection,
                });
            }
        }
    }
}

fn detect_left_release(
    mouse: Res<ButtonInput<MouseButton>>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut ev_build_station: EventWriter<BuildStationEvent>,
    mut ev_set_blueprint: EventWriter<SetBlueprintColorEvent>,
    mut blueprint_q: Query<(&mut StationBlueprint, &mut Visibility, &Transform)>,
) {
    if mouse.just_released(MouseButton::Left) {
        // строим
        let Ok((mut blueprint, mut vision, position)) = blueprint_q.get_single_mut() else {
            panic!("NO BLUEPRINT");
        };
        if blueprint.menu_flag {
            blueprint.menu_flag = false;
            return;
        }
        if !blueprint.can_build || *vision == Visibility::Hidden {
            println!("Drops there");
            *vision = Visibility::Hidden;
            return;
        }

        let position = position.translation.truncate();

        if keyboard.pressed(KeyCode::ShiftLeft) {
            blueprint.line_to_attach = usize::MAX;
        }

        *vision = Visibility::Hidden;
        ev_build_station.send(BuildStationEvent {
            position: (position.x.round() as i32, position.y.round() as i32),
            connection: blueprint.connection,
            direction: blueprint.direction,
            line_to_attach: blueprint.line_to_attach,
        });

        ev_set_blueprint.send(SetBlueprintColorEvent(Color::BLACK.with_alpha(0.0)));
    }
}

fn check_building_position(
    cursor_position: Res<CursorPosition>,
    q_stations: Query<(&Transform, &Station)>,
    mut blueprint_q: Query<&mut StationBlueprint>,
    mut ev_set_blueprint: EventWriter<SetBlueprintColorEvent>,
    metro: Res<Metro>,
) {
    if q_stations.iter().len() <= 0 {
        return;
    }
    let Ok(mut blueprint) = blueprint_q.get_single_mut() else {
        panic!("NO BLUEPRINT!");
    };

    if blueprint.line_to_attach == usize::MAX {
        return;
    }

    let sorted: Vec<(&Transform, &Station)> = q_stations
        .iter()
        .sort_by::<&Transform>(|t1, t2| {
            t1.translation
                .distance(cursor_position.0.extend(0.0))
                .total_cmp(&t2.translation.distance(cursor_position.0.extend(0.0)))
        })
        .collect();

    let (closest_transform, closest_station) = sorted[0];

    if closest_transform
        .translation
        .distance(cursor_position.0.extend(0.0))
        <= 100.0
    {
        let color: Color;
        if metro.lines[blueprint.line_to_attach] //??????????????
            .stations
            .contains(&closest_station)
        {
            blueprint.can_build = false;
            color = Color::srgba(1.0, 0.0, 0.0, 0.5);
        } else {
            blueprint.can_build = true;
            color = Color::BLACK.with_alpha(0.5);
        }

        ev_set_blueprint.send(SetBlueprintColorEvent(color));
    } else {
        blueprint.can_build = true;
        ev_set_blueprint.send(SetBlueprintColorEvent(Color::BLACK.with_alpha(0.5)));
    }
}
