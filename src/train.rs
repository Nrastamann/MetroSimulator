use std::time::Duration;

use bevy::prelude::*;

use crate::{
    metro::{Direction, Metro},
    passenger::PassengerDatabase,
    station::{Station, StationButton},
};

#[allow(unused)]
const TRAIN_STOP_TIME_SECS: f32 = 1.0;
const TRAIN_SPEED: f32 = 100.0;

pub struct TrainPlugin;

impl Plugin for TrainPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnTrainEvent>();
        app.add_systems(
            Update,
            (
                spawn_train,
                move_train,
                stop_train,
                switch_train_direction,
                update_train_text,
            ),
        );
    }
}

#[derive(Event)]
pub struct SpawnTrainEvent {
    pub line: usize,
    pub color: Color,
}

#[derive(Component)]
pub struct Train {
    line: usize,
    current: usize,
    passenger_ids: Vec<usize>,
    direction: Direction,
    last_stop_time: Duration,
}

#[derive(Component)]
struct TrainStop {
    timer: Timer,
}

impl Train {
    fn new(line: usize) -> Self {
        Self {
            line,
            current: 0,
            passenger_ids: vec![],
            direction: Direction::Forwards,
            last_stop_time: Duration::from_millis(0),
        }
    }
}

fn spawn_train(
    mut commands: Commands,
    mut ev_spawn: EventReader<SpawnTrainEvent>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    metro: Res<Metro>,
) {
    for ev in ev_spawn.read() {
        let mesh = meshes.add(Rectangle::new(36., 16.));
        let material = materials.add(ev.color);

        let Some(station) = metro.lines[ev.line].stations.front() else {
            return;
        };
        let position = station.position;

        commands
            .spawn((
                Mesh2d(mesh),
                MeshMaterial2d(material),
                Transform::from_translation(Vec3::new(position.0 as f32, position.1 as f32, 1.0)),
                Train::new(ev.line),
            ))
            .with_child((Text2d::new("0"),));
    }
}

// код говна
fn get_closest(positions: &Vec<Vec2>, target: &Vec2, direction: &Direction) -> (Vec2, usize) {
    let mut sorted = positions.clone();

    sorted.sort_by(|pos1, pos2| pos1.distance(*target).total_cmp(&pos2.distance(*target)));

    match direction {
        Direction::Forwards => {
            let index = positions.iter().position(|p| *p == sorted[0]).unwrap();
            if index + 1 >= positions.len() {
                return (positions[index], index);
            }
            return (positions[index + 1], index + 1);
        }
        Direction::Backwards => {
            let index = positions.iter().position(|p| *p == sorted[0]).unwrap();
            if index <= 0 {
                return (positions[index], index);
            }
            return (positions[index - 1], index - 1);
        }
    }
}

fn offload_passengers(
    station_button: &mut StationButton,
    station: &Station,
    train: &mut Train,
    passenger_database: &mut ResMut<PassengerDatabase>,
) -> Vec<usize> {
    let mut offloading_passengers = vec![];
    for id in train.passenger_ids.iter() {
        let passenger = passenger_database.0.get_mut(id).unwrap();
        if passenger
            .destination_station
            .is_some_and(|st| st.position == station.position)
        {
            offloading_passengers.push(*id);
        }
        passenger.destination_station = None;
    }

    train.passenger_ids = train
        .passenger_ids
        .iter()
        .filter(|&pass| !offloading_passengers.contains(pass))
        .map(|pass| pass.clone())
        .collect();

    let mut offloaded_passengers = vec![];
    while station_button.passenger_ids.len() < 12 && offloading_passengers.len() > 0 {
        let offloading_passenger = offloading_passengers.pop().unwrap();
        offloaded_passengers.push(offloading_passenger);
    }

    offloaded_passengers
}

fn load_passengers(
    station_button: &mut StationButton,
    train: &mut Train,
    offloaded_passengers: &mut Vec<usize>,
) {
    while train.passenger_ids.len() < 6 && station_button.passenger_ids.len() > 0 {
        let loading_passenger = station_button.passenger_ids.pop().unwrap();
        train.passenger_ids.push(loading_passenger);
    }

    station_button.passenger_ids.append(offloaded_passengers);
}

fn offload_passengers(
    station_button: &mut StationButton,
    station: &Station,
    train: &mut Train,
    passenger_database: &mut ResMut<PassengerDatabase>,
) -> Vec<usize> {
    let mut offloading_passengers = vec![];
    for id in train.passenger_ids.iter() {
        let passenger = passenger_database.0.get_mut(id).unwrap();
        if passenger.destination_station.is_some_and(|st| st.position == station.position) {
            offloading_passengers.push(*id);
        }
        passenger.destination_station = None;
    }

    train.passenger_ids =
        train.passenger_ids.iter()
        .filter(|&pass| !offloading_passengers.contains(pass))
        .map(|pass| pass.clone())
        .collect();

    let mut offloaded_passengers = vec![];
    while station_button.passenger_ids.len() < 12
    && offloading_passengers.len() > 0 {
        let offloading_passenger = offloading_passengers.pop().unwrap();
        offloaded_passengers.push(offloading_passenger);
    }

    offloaded_passengers
}

fn load_passengers(
    station_button: &mut StationButton,
    train: &mut Train,
    offloaded_passengers: &mut Vec<usize>,
) {
    while train.passenger_ids.len() < 6
    && station_button.passenger_ids.len() > 0 {
        let loading_passenger = station_button.passenger_ids.pop().unwrap();
        train.passenger_ids.push(loading_passenger);
    }

    station_button.passenger_ids.append(offloaded_passengers);
}

fn move_train(
    mut commands: Commands,
    mut q_train: Query<(Entity, &mut Transform, &mut Train), Without<TrainStop>>,
    mut q_station_button: Query<(&mut StationButton, &Station)>,
    metro: Res<Metro>,
    time: Res<Time>,
    mut passenger_database: ResMut<PassengerDatabase>,
) {
    for (e_train, mut train_transform, mut train) in q_train.iter_mut() {
        let line = &metro.lines[train.line];
        let Some(curve) = &line.curve else { return };
        let curve_positions: Vec<Vec2> =
            curve.iter_positions(32 * curve.segments().len()).collect();

        // получаем ближайшую точку пути с учётом направления поезда (скорее всего ошибка тут, потому что код говна)
        let (closest_point, closest_index) = get_closest(
            &curve_positions,
            &train_transform.translation.truncate(),
            &train.direction,
        );

        let closest_point_tuple = (
            closest_point.x.floor() as i32,
            closest_point.y.floor() as i32,
        );

        // проверяем, если текущая точка пути совпадает с позицией станции и нужно сделать остановку
        if line
            .stations
            .iter()
            .map(|station| station.position)
            .collect::<Vec<(i32, i32)>>()
            .contains(&closest_point_tuple)
            && (time.elapsed() - train.last_stop_time).as_secs_f32() >= TRAIN_STOP_TIME_SECS * 1.1
        // todo: get rid of magic number
        {
            let (mut btn, station) = q_station_button
                .iter_mut()
                .filter(|(_, station)| station.position == closest_point_tuple)
                .next()
                .unwrap();

            let mut offloaded_passengers =
                offload_passengers(&mut btn, &station, &mut train, &mut passenger_database);
            load_passengers(&mut btn, &mut train, &mut offloaded_passengers);

            train.last_stop_time = time.elapsed();

            commands.entity(e_train).insert(TrainStop {
                timer: Timer::from_seconds(TRAIN_STOP_TIME_SECS, TimerMode::Once),
            });
        }

        train.current = closest_index;

        let closest_point_tuple = (
            closest_point.x.floor() as i32,
            closest_point.y.floor() as i32,
        );

        // проверяем, если текущая точка пути совпадает с позицией станции и нужно сделать остановку
        if line.stations.iter()
            .map(|station| station.position)
            .collect::<Vec<(i32, i32)>>()
            .contains(&closest_point_tuple)
        {
            let (mut btn, station) =
                q_station_button.iter_mut()
                .filter(|(_, station)| station.position == closest_point_tuple)
                .next().unwrap();


            let mut offloading_passengers = vec![];
            for passenger in train.passengers.iter() {
                if passenger.destination_pool.contains(&station) {
                    offloading_passengers.push(passenger.clone());
                }
            }

            train.passengers =
                train.passengers.iter()
                .filter(|&pass| !offloading_passengers.contains(pass))
                .map(|pass| pass.clone())
                .collect();

            let mut offloaded_passengers = vec![];
            while btn.passengers.len() < 12
            && offloading_passengers.len() > 0 {
                let offloading_passenger = offloading_passengers.pop().unwrap();
                offloaded_passengers.push(offloading_passenger);
            }

            while train.passengers.len() < 6
            && btn.passengers.len() > 0 {
                let loading_passenger = btn.passengers.pop().unwrap();
                train.passengers.push(loading_passenger);
            }

            btn.passengers.append(&mut offloaded_passengers);

            // commands.entity(e_train).insert(TrainStop { timer: Timer::from_seconds(TRAIN_STOP_TIME_SECS, TimerMode::Once) });
        }
        
        train.current = closest_index;

        let diff = closest_point.extend(train_transform.translation.z) - train_transform.translation;
        let angle = diff.y.atan2(diff.x);
        train_transform.rotation = train_transform
            .rotation
            .lerp(Quat::from_rotation_z(angle), 12.0 * time.delta_secs());

        let direction = curve_positions[train.current] - train_transform.translation.truncate();
        train_transform.translation +=
            direction.normalize().extend(0.) * TRAIN_SPEED * time.delta_secs();
    }
}

fn stop_train(
    mut commands: Commands,
    mut q_train: Query<(Entity, &mut TrainStop)>,
    time: Res<Time>,
) {
    for (e_train, mut train_stop) in q_train.iter_mut() {
        train_stop.timer.tick(time.delta());

        if train_stop.timer.just_finished() {
            commands.entity(e_train).remove::<TrainStop>();
        }
    }
}

fn switch_train_direction(mut q_train: Query<&mut Train>, metro: Res<Metro>) {
    for mut train in q_train.iter_mut() {
        let line = &metro.lines[train.line];
        let Some(curve) = &line.curve else { return };
        let curve_positions: Vec<Vec2> =
            curve.iter_positions(32 * curve.segments().len()).collect();

        if train.current == 0 && train.direction == Direction::Backwards {
            train.direction = Direction::Forwards;
        }
        if train.current == curve_positions.len() - 1 && train.direction == Direction::Forwards {
            train.direction = Direction::Backwards;
        }
    }
}

fn update_train_text(
    mut q_train_text: Query<(&mut Text2d, &Parent)>,
    q_train: Query<(&Train, Entity)>,
) {
    for (train, e_train) in q_train.iter() {
        for (mut text, parent) in q_train_text.iter_mut() {
            if e_train != parent.get() {
                continue;
            }

            text.0 = train.passenger_ids.len().to_string();
        }
    }
}
