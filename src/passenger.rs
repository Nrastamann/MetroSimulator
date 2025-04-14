use std::time::Duration;

use bevy::{prelude::*, time::common_conditions::on_timer, utils::HashMap};

use crate::{district::{DistrictMap, DistrictType}, station::Station, GameState};

pub struct PassengerPlugin;

impl Plugin for PassengerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PassengerDatabase>();
        app.add_event::<AddPassengerEvent>();
        app.add_systems(Update, (
            add_passengers,
            fill_passenger_pool
                // не слишком часто делаем проверки на заполненный пул мест пассажира
                .run_if(on_timer(Duration::from_millis(1000))) 
        ).run_if(
                in_state(GameState::InGame)
            )
        );
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum PassengerDesire {
    Home,
    Work,
    Entertainment
}

#[derive(Clone, PartialEq)]
pub struct Passenger {
    pub current_desire: PassengerDesire,
    pub district_ids: [usize; 3],
    pub destination_station: Option<Station>,
}

#[derive(Resource, Default)]
pub struct PassengerDatabase(pub HashMap<usize, Passenger>);

#[derive(Event)]
pub struct AddPassengerEvent {
    pub(crate) district_id: usize
}

fn add_passengers(
    mut ev_add_passenger: EventReader<AddPassengerEvent>,
    mut database: ResMut<PassengerDatabase>,
    mut district_map: ResMut<DistrictMap>,
) {
    for ev in ev_add_passenger.read() {
        let passenger = Passenger {
            current_desire: PassengerDesire::Home,
            district_ids: [ev.district_id, 0, 0], // домашний район - район, в котором он создался
            destination_station: None
        };
        let passenger_id = database.0.len();
        district_map.districts[ev.district_id].passenger_ids.push(passenger_id.clone());
        database.0.insert(passenger_id, passenger);
    }
}

fn fill_passenger_pool(
    mut database: ResMut<PassengerDatabase>,
    district_map: Res<DistrictMap>,
) {
    for (_, passenger) in database.0.iter_mut() {
        if passenger.district_ids[1] == 0 {
            let work_districts: Vec<usize> =
                district_map.districts.iter()
                .filter(|dist| dist.district_type == DistrictType::Work)
                .map(|dist| dist.id).collect();

            if work_districts.len() <= 0 {
                return;
            }

            passenger.district_ids[1] = work_districts[rand::random_range(0..work_districts.len())]; 
        }

        if passenger.district_ids[2] == 0 {
            let entertainment_districts: Vec<usize> =
                district_map.districts.iter()
                .filter(|dist| dist.district_type == DistrictType::Entertainment)
                .map(|dist| dist.id).collect();


            if entertainment_districts.len() <= 0 {
                return;
            }

            passenger.district_ids[1] = entertainment_districts[rand::random_range(0..entertainment_districts.len())]; 
        }
    }
}