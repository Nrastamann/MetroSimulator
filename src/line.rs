use std::collections::LinkedList;

use bevy::prelude::*;
use rand::prelude::*;
pub const LINE_NAMES: [&str; 10] = ["Ветка1","Ветка2","Ветка3","Ветка4","Ветка5","Ветка6","Ветка7","Ветка8","Ветка9","Ветка10",]; 
use crate::station::Station;

#[derive(PartialEq)]
pub struct Line {
    pub name: String,
    pub id: usize,
    pub stations: LinkedList<Station>,
    pub points: Vec<(i32,i32)>,
    pub curve: Option<CubicCurve<Vec2>>,
    pub color: Color,
}

impl Line {
    fn update_curve(&mut self) { // обновляем точки, по которым строится кривая
        self.curve = CubicCardinalSpline::new_catmull_rom(self.stations
            .iter().map(|station| Vec2::new(station.position.0 as f32, station.position.1 as f32)).collect::<Vec<Vec2>>())
            .to_curve().ok();
    }

    pub fn new_from_points(id: usize, new_points: Vec<(i32, i32)>) -> Self { // новая ветка из вектора станций
        let curve = CubicCardinalSpline::new_catmull_rom(new_points
            .iter().map(|&(x,y)| Vec2::new(x as f32, y as f32)).collect::<Vec<Vec2>>())
            .to_curve().ok();
        
        let mut rng = rand::rng();    

        Self {
            name: LINE_NAMES[rand::rng().random_range(0..9)].to_string(),
            id,
            stations: LinkedList::new(),
            points: new_points,
            curve,
            color: Color::hsl(rng.random_range(0..=12) as f32 * 30., 0.5, 0.5)
        }
    }

    pub fn push_back(&mut self, point: (i32, i32)) {
        self.stations.push_back(Station::new(point));
        self.update_curve();
    }

    pub fn push_front(&mut self, point: (i32, i32)) {
        self.stations.push_front(Station::new(point));
        self.update_curve();
    }
}

