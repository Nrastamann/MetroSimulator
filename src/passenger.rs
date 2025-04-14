use bevy::prelude::*;

use crate::station::Station;

pub struct PassengerPlugin;

impl Plugin for PassengerPlugin {
    fn build(&self, app: &mut App) {
        
    }
}

#[derive(Default, Clone, Copy, PartialEq)]
pub enum PassengerDesire {
    #[default]
    Home,
    Work,
    Entertainment
}

#[derive(Default, Clone, PartialEq)]
pub struct Passenger {
    pub current_desire: PassengerDesire,
    pub destination_pool: Vec<Station>,
}

