use std::f32::consts::PI;

use bevy::prelude::*;

use crate::{
    cursor::CursorPosition, line::{SpawnLineCurveEvent, UpdateLineRendererEvent}, metro::{
        Direction,
        Metro
    }, station_blueprint::SetBlueprintColorEvent, train::SpawnTrainEvent, GameState
};


pub struct StationPlugin;

impl Plugin for StationPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<StationBuilder>();
        app.add_event::<SpawnStationEvent>();
        app.add_systems(Update, (
            hover_select, check_building_position, build_new, spawn_station, debug_draw_passengers
        ).run_if(in_state(GameState::InGame)));
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
                ev.position.1 as f32, 2.0
            )),
            button,
            station,
        )).add_child(inner_circle);

    }
}

fn debug_draw_passengers(
    q_station: Query<(&Transform, &StationButton)>,
    mut gizmos: Gizmos
) {
    for (transform, station) in q_station.iter() {
        for i in 0..station.passenger_ids.len() {
            let position = transform.translation.truncate() + 40. * Vec2::from_angle((i as f32)*(PI/6.));
            gizmos.circle_2d(Isometry2d::from_translation(position), 5., Color::BLACK);
        }
    }
}

fn hover_select( // просто выделение при наведении на станцию
    mut stations: Query<(&mut Transform, &mut StationButton)>,
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
    Prolong,
    NewLine,
    Connect { closest: (i32, i32) },
    #[default]
    Nothing
}

#[derive(Default, Resource)]
struct StationBuilder { // todo: приудмать, как это переделать, чтобы было не так убого
    is_building: bool,
    line_to_attach_to: usize,
    connection: (i32, i32),
    action: BuilderAction, 
    direction: Direction
}

fn build_new(
    q_station: Query<(&Station, &StationButton)>,
    mut metro: ResMut<Metro>,
    cursor_position: Res<CursorPosition>,
    mouse: Res<ButtonInput<MouseButton>>,
    mut builder: ResMut<StationBuilder>,
    mut ev_spawn_station: EventWriter<SpawnStationEvent>,
    mut ev_set_blueprint: EventWriter<SetBlueprintColorEvent>,
    mut ev_spawn_train: EventWriter<SpawnTrainEvent>,
    mut ev_update_line_renderer: EventWriter<UpdateLineRendererEvent>,
    mut ev_spawn_line: EventWriter<SpawnLineCurveEvent>,
) {
    if mouse.just_pressed(MouseButton::Left) { // определяем, будет это продолжение старой ветки или создание новой
        let Some((selected_station, _)) =
            q_station.iter()
            .filter(
                |(_, btn)| btn.selected
            )
            .next()
        else { return }; 
        
        for line in metro.lines.iter() {
            if line.stations.contains(&selected_station) {
                builder.line_to_attach_to = line.id;
                if line.stations.front().unwrap() == selected_station {
                    builder.direction = Direction::Backwards;
                    builder.action = BuilderAction::Prolong;
                }
                else if line.stations.back().unwrap() == selected_station {
                    builder.direction = Direction::Forwards;
                    builder.action = BuilderAction::Prolong;
                }
                else {
                    builder.action = BuilderAction::NewLine;
                }
                builder.is_building = true;
                builder.connection = selected_station.position;
                ev_set_blueprint.send(
                    SetBlueprintColorEvent(
                        Color::BLACK.with_alpha(0.5)
                    )
                );
                break;
            }
        }
    }

    if mouse.just_released(MouseButton::Left)
    && builder.is_building { // строим
        let position = (
            cursor_position.0.x.floor() as i32,
            cursor_position.0.y.floor() as i32
        );

        match builder.action {
            BuilderAction::Prolong => {
                let line = &mut metro.lines[builder.line_to_attach_to];
                match builder.direction {
                    Direction::Forwards => line.push_back(position),
                    Direction::Backwards => line.push_front(position)
                }

                ev_update_line_renderer.send(UpdateLineRendererEvent {
                    line_id: line.id
                });

                ev_spawn_station.send(SpawnStationEvent {
                    position: cursor_position.as_tuple(),
                    connection: builder.connection,
                });
            },
            BuilderAction::NewLine => {
                let line = metro.add_line(vec![position, builder.connection]);
                let color = line.color;
                ev_spawn_train.send(SpawnTrainEvent { line: line.id, color });

                ev_spawn_line.send(SpawnLineCurveEvent { line_id: line.id });

                ev_spawn_station.send(SpawnStationEvent {
                    position: cursor_position.as_tuple(),
                    connection: builder.connection,
                });
            }
            // BuilderAction::Connect { closest } => {
            //     let place = builder.place;
            //     let line = &mut metro.lines[builder.line_to_attach_to];
            //     if place == line.points.len() - 1 {
            //         line.push_back(closest);
            //     }
            //     // else {
            //     //    line.insert(place, closest);
            //     // }
            // },
            _ => {}
        }

        builder.is_building = false;
        ev_set_blueprint.send(SetBlueprintColorEvent(Color::BLACK.with_alpha(0.0)));
    }
}

fn check_building_position(
    cursor_position: Res<CursorPosition>,
    q_stations: Query<(&Transform, &Station)>,
    builder: Res<StationBuilder>,
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

        if metro.lines[builder.line_to_attach_to].stations.contains(&closest_station) {
            // builder.action = BuilderAction::Nothing;
            color = Color::srgba(1.0, 0.0, 0.0, 0.5);
        }
        else {
            // builder.action = BuilderAction::Connect { closest: closest_station.position };
            color = Color::BLACK.with_alpha(0.5);
        }

        if builder.is_building {
            ev_set_blueprint.send(SetBlueprintColorEvent(color));
        }
    }
    else {
        // builder.action = BuilderAction::Prolong;
        if builder.is_building {
            ev_set_blueprint.send(SetBlueprintColorEvent(Color::BLACK.with_alpha(0.5)));
        }
    }
}