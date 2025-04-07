use bevy::prelude::*;
use rand::Rng;

use crate::{
    cursor::CursorPosition,
    metro::{Direction, Metro},
    station_blueprint::{Direction, SetBlueprintColorEvent, StationBlueprint},
    train::SpawnTrainEvent,
    GameState,
};

pub const STATION_NAMES: [&str; 10] = [
    "Достоевская",
    "Обводный канал",
    "Озерки",
    "Парнас",
    "Динамо",
    "Автово",
    "Сенная площадь",
    "Купчино",
    "Дыбенко",
    "Звездная",
];

pub struct StationPlugin;

impl Plugin for StationPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnStationEvent>();
        app.add_systems(
            Update,
            (
                hover_select,
                check_building_position,
                build_new,
                spawn_station,
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
        Self {
            position,
        }
    }
}

#[derive(Default, Component)]
pub struct StationButton {
    pub selected: bool,
    pub passenger_ids: Vec<usize>,
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

        let inner_circle = commands.spawn((
            Mesh2d(meshes.add(Circle::new(20.))),
            MeshMaterial2d(materials.add(Color::WHITE)),
            Transform::from_translation(Vec3::new(
                0.0,0.0, 2.0
            )),
        ))
        .id();

        let button = StationButton::default();
        // for _ in 0..rand::random_range(0..5) {
        //     let mut destination_pool = vec![];
        //     for line in metro.lines.iter() {
        //         for station in line.stations.iter() {
        //             if rand::random_bool(0.5) {
        //                 destination_pool.push(*station);
        //             }
        //         }
        //     }
        //     button.passengers.push(Passenger {
        //         destination_pool,
        //         ..default()
        //     });
        // }

        metro.stations.add(ev.connection, ev.position, station.clone());
        commands.spawn((
            Mesh2d(mesh),
            MeshMaterial2d(material),
            Transform::from_translation(Vec3::new(
                ev.position.0 as f32,
                ev.position.1 as f32, 0.0
            )),
            StationButton::default(),
            station,
            render_data
        ));
    }
}

fn hover_select( // просто выделение при наведении на станцию
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

#[derive(Default)]
enum BuilderAction {
    Build,
    Connect {
        closest: (i32, i32),
    },
    #[default]
    Nothing,
}
#[derive(Event)]
pub struct StartBuildingEvent {
    pub connection: (i32, i32),
    pub direction: Direction,
    pub line_to_attach: usize,
}

#[derive(Event)]
pub struct BuildStationEvent {
    pub place: (i32, i32),
    pub connection: (i32, i32),
    pub direction: Direction,
    pub line_to_attach: usize,
}

fn build_new(
    stations: Query<&Station>,
    metro: Res<Metro>,
    mouse: Res<ButtonInput<MouseButton>>,
    mut ev_start_build: EventWriter<StartBuildingEvent>,
    mut ev_set_blueprint: EventWriter<SetBlueprintColorEvent>,
) {
    if mouse.just_pressed(MouseButton::Left) {
        // начинаем строить, определяем, будет это продолжение старой ветки или создание новой
        for station in stations.iter() {
            if station.selected {
                for i in 0..metro.lines.len() {
                    if let Some(place) = metro.lines[i].points.iter().position(|s| *s == station.id)
                    {
                        let mut direction: Direction = Direction::Forward;

                        // if keyboard.pressed(KeyCode::ShiftLeft) {
                        //     line = -1;
                        // }

                        if place == 0 {
                            direction = Direction::Backwards;
                        }

                        ev_start_build.send(StartBuildingEvent {
                            connection: station.id,
                            direction: direction,
                            line_to_attach: i,
                        });
                        ev_set_blueprint.send(
                            // todo: make color match the line
                            SetBlueprintColorEvent(Color::BLACK.with_alpha(0.5)),
                        );
                        break;
                    }
                }
                break;
            }
        }
    }
}

fn build_station(){
    
}

fn detect_left_release(
    cursor_position: Res<CursorPosition>,
    mouse: Res<ButtonInput<MouseButton>>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut ev_build_station: EventWriter<BuildStationEvent>,
    mut ev_set_blueprint: EventWriter<SetBlueprintColorEvent>,
    mut blueprint_q: Query<(&mut StationBlueprint, &mut Visibility)>,
) {
    if mouse.just_released(MouseButton::Left) {
        // строим
        let Ok((mut blueprint, mut vision)) = blueprint_q.get_single_mut() else {
            panic!("NO BLUEPRINT");
        };
        
        if !blueprint.can_build || *vision == Visibility::Hidden{
            *vision = Visibility::Hidden;
            return;
        }

        let position = (
            cursor_position.0.x.floor() as i32,
            cursor_position.0.y.floor() as i32,
        );

        if keyboard.pressed(KeyCode::ShiftLeft) {
            blueprint.line_to_attach = usize::MAX;
        }

        *vision = Visibility::Hidden;

        ev_build_station.send(BuildStationEvent {
            place: position,
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

        if metro.lines[blueprint.line_to_attach]
            .points
            .contains(&closest_station.id)
        {
            blueprint.can_build = false;
            color = Color::srgba(1.0, 0.0, 0.0, 0.5);
        } else {
            blueprint.can_build = true;
            color = Color::BLACK.with_alpha(0.5);
        }

        ev_set_blueprint.send(SetBlueprintColorEvent(color));
    } else {
        ev_set_blueprint.send(SetBlueprintColorEvent(Color::BLACK.with_alpha(0.5)));
    }
}
