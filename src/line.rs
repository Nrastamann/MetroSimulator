use bevy::prelude::*;
use rand::prelude::*;

#[derive(PartialEq)]
pub struct Line {
    pub points: Vec<(i32,i32)>,
    pub curve: Option<CubicCurve<Vec2>>,
    pub color: Color,
}

impl Line {
    fn update_curve(&mut self) { // обновляем точки, по которым строится кривая
        self.curve = CubicCardinalSpline::new_catmull_rom(self.points
            .iter().map(|&(x,y)| Vec2::new(x as f32, y as f32)).collect::<Vec<Vec2>>())
            .to_curve().ok();
    }

    pub fn new_from_points(new_points: Vec<(i32, i32)>) -> Self { // новая ветка из вектора станций
        let curve = CubicCardinalSpline::new_catmull_rom(new_points
            .iter().map(|&(x,y)| Vec2::new(x as f32, y as f32)).collect::<Vec<Vec2>>())
            .to_curve().ok();
        
        let mut rng = rand::rng();    

        Self {
            points: new_points,
            curve,
            color: Color::hsl(rng.random_range(0..=36) as f32 * 10., 0.5, 0.5)
        }
    }

    pub fn insert(&mut self, index: usize, point: (i32, i32)) {
        self.points.insert(index, point);
        self.update_curve();
    }

    pub fn push(&mut self, point: (i32, i32)) {
        self.points.push(point);
        self.update_curve();
    }
}

