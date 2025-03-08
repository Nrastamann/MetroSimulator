use bevy::prelude::*;
use rand::prelude::*;

use crate::station::Station;

#[derive(PartialEq)]
pub struct Line {
    pub stations: Vec<Station>,
    pub curve: Option<CubicCurve<Vec2>>,
    pub color: Color,
}

impl Line {
    fn update_curve(&mut self) {
        self.curve = CubicCardinalSpline::new_catmull_rom(self.stations
            .iter().map(|station| station.position).collect::<Vec<Vec2>>())
            .to_curve().ok();
    }

    pub fn new_from_stations(new_stations: Vec<Station>) -> Self {
        let curve = CubicCardinalSpline::new_catmull_rom(new_stations
            .iter().map(|station| station.position).collect::<Vec<Vec2>>())
            .to_curve().ok(); 
        
        let mut rng = rand::rng();    

        Self {
            stations: new_stations,
            curve,
            color: Color::hsl(rng.random_range(0..=36) as f32 * 10., 0.5, 0.5)
        }
    }

    pub fn push_front(&mut self, station: Station) {
        self.stations.insert(0, station);
        self.update_curve();
    }

    pub fn push_back(&mut self, station: Station) {
        self.stations.push(station);
        self.update_curve();
    }
}

