use std::{collections::VecDeque, time::Duration};

use bevy::{prelude::*, time::common_conditions::on_timer, utils::HashMap};

use crate::{
    district::{DistrictMap, DistrictType},
    metro::Metro,
    station::{Station, StationButton},
    GameState, DISTRICT_CELL_SIZE,
};

pub struct PassengerPlugin;

impl Plugin for PassengerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PassengerDatabase>();
        app.add_event::<AddPassengerEvent>();
        app.add_systems(
            Update,
            (
                add_passengers,
                decide_where_to_go
                    .run_if(on_timer(Duration::from_secs(5))),
                start_moving,
                fill_passenger_pool
                    // не слишком часто делаем проверки на заполненный пул мест пассажира
                    .run_if(on_timer(Duration::from_millis(100))),
                stop_moving,
            )
                .run_if(in_state(GameState::InGame)),
        );
        app.add_systems(Update, monitor_first_guy.run_if(on_timer(Duration::from_secs(5))));
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum PassengerDesire {
    Home,
    Work,
    Entertainment,
}

impl std::fmt::Display for PassengerDesire {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let word = match self {
            PassengerDesire::Home => "home",
            PassengerDesire::Work => "work",
            PassengerDesire::Entertainment => "fun",
        };

        write!(f, "{word}")
    }
}

#[derive(Clone, PartialEq)]
pub struct Passenger {
    pub current_desire: PassengerDesire,
    pub last_visited_district: usize,
    pub district_ids: [usize; 3],
    pub route: Vec<Station>,
}

#[derive(Resource, Default)]
pub struct PassengerDatabase(pub HashMap<usize, Passenger>);

#[derive(Event)]
pub struct AddPassengerEvent {
    pub(crate) district_id: usize,
}

fn add_passengers(
    mut ev_add_passenger: EventReader<AddPassengerEvent>,
    mut database: ResMut<PassengerDatabase>,
    mut district_map: ResMut<DistrictMap>,
) {
    for ev in ev_add_passenger.read() {
        let passenger = Passenger {
            current_desire: PassengerDesire::Home,
            last_visited_district: ev.district_id,
            district_ids: [ev.district_id, 0, 0], // домашний район - район, в котором он создался
            route: vec![],
        };
        let passenger_id = database.0.len();
        district_map.districts[ev.district_id]
            .passenger_ids
            .push(passenger_id.clone());
        database.0.insert(passenger_id, passenger);
    }
}

fn fill_passenger_pool(mut database: ResMut<PassengerDatabase>, district_map: Res<DistrictMap>) {
    for (_, passenger) in database.0.iter_mut() {
        if passenger.district_ids[1] == 0 {
            let work_districts: Vec<usize> = district_map
                .districts
                .iter()
                .filter(|dist| dist.district_type == DistrictType::Work)
                .map(|dist| dist.id)
                .collect();

            if work_districts.len() <= 0 {
                continue;
            }

            let district_id = work_districts[rand::random_range(0..work_districts.len())];
            passenger.district_ids[1] = district_id;
        }

        if passenger.district_ids[2] == 0 {
            let entertainment_districts: Vec<usize> = district_map
                .districts
                .iter()
                .filter(|dist| dist.district_type == DistrictType::Entertainment)
                .map(|dist| dist.id)
                .collect();

            if entertainment_districts.len() <= 0 {
                continue;
            }

            let district_id =
                entertainment_districts[rand::random_range(0..entertainment_districts.len())];
            passenger.district_ids[2] = district_id;
        }
    }
}

fn decide_where_to_go(
    mut database: ResMut<PassengerDatabase>,
    district_map: Res<DistrictMap>,
    metro: Res<Metro>,
) {
    for (_, passenger) in database.0.iter_mut() {
        // println!("{:?}", passenger.district_ids);

        if passenger.route.len() != 0
            || passenger.district_ids[1] == 0
            || passenger.district_ids[2] == 0
        {
            continue;
        }

        let random_desire: PassengerDesire;
        match passenger.current_desire {
            PassengerDesire::Entertainment => {
                random_desire = match rand::random_bool(0.5) {
                    false => PassengerDesire::Home,
                    true => PassengerDesire::Work,
                };
            }
            PassengerDesire::Home => {
                random_desire = match rand::random_bool(0.5) {
                    false => PassengerDesire::Entertainment,
                    true => PassengerDesire::Work,
                };
            }
            PassengerDesire::Work => {
                random_desire = match rand::random_bool(0.5) {
                    false => PassengerDesire::Home,
                    true => PassengerDesire::Entertainment,
                };
            }
        }
        passenger.current_desire = random_desire;

        let destination_district_id = passenger.district_ids[random_desire as usize];
        let district = &district_map.districts[destination_district_id];

        'line_loop: for line in metro.lines.iter() {
            for station in line.stations.iter() {
                for cell in district.cells.iter() {
                    let cell_position =
                        Vec2::new(cell.0 as f32, cell.1 as f32) * DISTRICT_CELL_SIZE;
                    let station_position =
                        Vec2::new(station.position.0 as f32, station.position.1 as f32);
                    let distance = cell_position.distance(station_position);

                    if distance > DISTRICT_CELL_SIZE / 2. {
                        continue;
                    }

                    passenger.route.push(*station);
                    break 'line_loop;
                }
            }
        }
    }
}

fn start_moving(
    database: Res<PassengerDatabase>,
    mut district_map: ResMut<DistrictMap>,
    metro: ResMut<Metro>,
    mut q_station_button: Query<(&mut StationButton, &Station)>,
) {
    for district in district_map
        .districts
        .iter_mut()
        .filter(|dist| dist.passenger_ids.len() > 0)
    {
        for id in district.passenger_ids.clone().iter() {
            let passenger = database.0.get(id).unwrap();
            if passenger.route.len() <= 0 {
                continue;
            }

            let mut starting_line = None;
            // let mut starting_station = None;
            let destination_station = database.0.get(id).unwrap().route[0];

            // WTF IS THIS SHIT
            let mut destination_line = None;
            for line in metro.lines.iter() {
                if line.stations.contains(&destination_station) {
                    destination_line = Some(line.id);
                }
            }
            if destination_line.is_none() {
                continue;
            }

            'line_loop: for line in metro.lines.iter() {
                for station in line.stations.iter() {
                    for cell in district.cells.iter() {
                        let cell_position =
                            Vec2::new(cell.0 as f32, cell.1 as f32) * DISTRICT_CELL_SIZE;
                        let station_position =
                            Vec2::new(station.position.0 as f32, station.position.1 as f32);
                        let distance = cell_position.distance(station_position);

                        if distance > DISTRICT_CELL_SIZE / 2. {
                            continue;
                        }

                        let Some((mut station_button, _)) = q_station_button
                            .iter_mut()
                            .filter(|(_, &st)| station.position == st.position)
                            .next()
                        else {
                            continue;
                        };

                        if station_button.passenger_ids.len() >= 12 {
                            continue;
                        }

                        station_button.passenger_ids.push(*id);

                        let Some(remove_index) = district
                            .passenger_ids
                            .iter()
                            .position(|pass_id| *pass_id == *id)
                        else {
                            continue;
                        };

                        district.passenger_ids.remove(remove_index);

                        starting_line = Some(line.id);
                        // starting_station = Some(station);

                        break 'line_loop;
                    }
                }
            }

            if starting_line.is_none() {
                continue;
            }

            // let mut queue = VecDeque::new();
            // queue.push_back(starting_line.unwrap());

            let mut visited = HashMap::new();
            let mut last_station = starting_station.unwrap();

            let mut visited_lines: Vec<usize> = vec![];

            // let mut visited_lines: Vec<usize> = vec![];

            // while let Some(line_id) = queue.pop_front() {
            //     if line_id == destination_line.unwrap() {
            //         let mut path = vec![destination_station];
            //         let mut current = *starting_station.unwrap();
            //         while let Some(&Some(prev)) = visited.get(&current.position) {
            //             path.push(prev);
            //             current = prev.clone();
            //         }
            //         path.reverse();
            //         database.0.get_mut(id).unwrap().route = path;
            //         break; // success
            //     }

            //     // WHAT THE FUCK
            //     // WHO WROTE THIS SHIT
            //     // OH IT WAS ME
            //     // NEVERMIND
            //     // I HATE MY PAST SELF

                for line in metro
                    .lines
                    .iter()
                    .filter(|&current| *current != metro.lines[line_id])
                {
                    if visited_lines.contains(&line.id) {
                        continue;
                    }

                    for station in line.stations.iter() {
                        if metro.lines[line_id].stations.contains(station) {
                            if visited.contains_key(&station.position) {
                                continue;
                            }

                            queue.push_back(line.id);
                            visited.insert(station.position, Some(*last_station));
                            visited_lines.push(line.id);
                            last_station = station;
                            break;
                        }
                    }
                }
            }
        }
    }
}

fn stop_moving(
    database: Res<PassengerDatabase>,
    mut district_map: ResMut<DistrictMap>,
    metro: Res<Metro>,
    mut q_station_button: Query<(&mut StationButton, &Station)>,
) {
    for line in metro.lines.iter() {
        for station in line.stations.iter() {
            let Some((mut station_button, _)) = q_station_button
                .iter_mut()
                .filter(|(_, &st)| station.position == st.position)
                .next()
            else {
                continue;
            };

            for passenger_id in station_button.passenger_ids.clone().iter() {
                let Some(passenger) = database.0.get(passenger_id) else {
                    continue;
                };

                if passenger.route.len() != 0 {
                    continue;
                }

                let district = &mut district_map.districts
                    [passenger.district_ids[passenger.current_desire as usize]];

                district.passenger_ids.push(*passenger_id);

                let remove_index = station_button
                    .passenger_ids
                    .iter()
                    .position(|id| id == passenger_id)
                    .unwrap();

                station_button.passenger_ids.remove(remove_index);
            }
        }
    }
}

fn monitor_first_guy(database: Res<PassengerDatabase>) {
    let Some(passenger) = database.0.get(&0) else {
        return;
    };

    info!("want to go to: {}\t route length: {}\n", passenger.current_desire, passenger.route.len());
}
