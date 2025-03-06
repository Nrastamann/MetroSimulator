use bevy::prelude::*;

use crate::station::Station;

pub struct Line {
    pub stations: Vec<Station>,
    pub curve: Option<CubicCurve<Vec2>>,
}

impl Line {
    pub fn new_from_stations(new_stations: Vec<Station>) -> Self {
        let curve = CubicCardinalSpline::new_catmull_rom(new_stations
            .iter().map(|station| station.position).collect::<Vec<Vec2>>())
            .to_curve().ok(); 
        
        Self {
            stations: new_stations,
            curve
        }
    }

    pub fn add_station(&mut self, station: Station) {
        self.stations.push(station);
        
        self.curve = CubicCardinalSpline::new_catmull_rom(self.stations
            .iter().map(|station| station.position).collect::<Vec<Vec2>>())
            .to_curve().ok();
    }
}

