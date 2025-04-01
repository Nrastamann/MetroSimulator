use std::{cmp::Ordering, collections::HashMap, f32::consts::PI, time::Duration};

use bevy::{prelude::*, time::common_conditions::on_timer};

use crate::{GameState, DISTRICT_CELL_SIZE, MAX_DISTRICT_SIZE};

pub struct DistrictPlugin;

impl Plugin for DistrictPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<DistrictMap>();
        app.add_systems(OnEnter(GameState::InGame), (
            test_gen_district,
        ));
        app.add_systems(Update, (
            grow_districts
                .run_if(on_timer(Duration::from_millis(100))),
            start_new_districts
                .run_if(on_timer(Duration::from_millis(1000))),
            test_draw_district
        )
        .run_if(in_state(GameState::InGame)));
    }
}

#[derive(Copy, Clone, PartialEq)]
enum DistrictType {
    Living,
    Work,
    Entertainment,
}

impl DistrictType {
    fn color(&self) -> Color {
        match self {
            Self::Living => Color::srgb(0.8, 0.4, 0.1),
            Self::Entertainment => Color::srgb(0.1, 0.8, 0.1),
            Self::Work => Color::srgb(0.1, 0.1, 0.8),
        }
    }
}

pub struct DistrictCell {
    population: u32,
    max_population: u32,
}

#[derive(Clone, PartialEq)]
struct District {
    district_type: DistrictType,
    is_completed: bool,
    derivatives_amount: u32,
    is_fertile: bool,
    cell_keys: Vec<(i32, i32)>,
    max_size: usize,
}

impl Default for District {
    fn default() -> Self {
        Self {
            district_type: DistrictType::Living,
            is_completed: false,
            derivatives_amount: 0,
            is_fertile: true,
            cell_keys: vec![],
            max_size: rand::random_range(0..40) + MAX_DISTRICT_SIZE
        }
    }
}

#[derive(Resource, Default)]
pub struct DistrictMap {
    districts: Vec<District>,
    cells: HashMap<(i32, i32), DistrictCell>,
}

fn test_gen_district(
    mut district_map: ResMut<DistrictMap>,
) {
    let mut district = District {
        district_type: DistrictType::Living,
        ..default()
    };


    district.cell_keys.push((0,0));
    district_map.cells.insert((0,0), DistrictCell { population: 0, max_population: 5 });

    district_map.districts.push(district);
}

fn start_new_districts(
    mut district_map: ResMut<DistrictMap>,
) {
    for district in district_map.districts.clone().iter()
        .filter(|&dist| dist.is_completed && dist.derivatives_amount < 4 && dist.is_fertile) {
        
        let index = district_map.districts.iter().position(|dist| *dist == *district).unwrap();
        district_map.districts[index].derivatives_amount+=1;

        let random_type: DistrictType;
        match district.district_type {
            DistrictType::Entertainment => {
                random_type = match rand::random_bool(0.5) {
                    false => DistrictType::Living,
                    true => DistrictType::Work,
                };
            },
            DistrictType::Living => {
                random_type = match rand::random_bool(0.5) {
                    false => DistrictType::Entertainment,
                    true => DistrictType::Work,
                };
            },
            DistrictType::Work => {
                random_type = match rand::random_bool(0.5) {
                    false => DistrictType::Living,
                    true => DistrictType::Entertainment,
                };
            }
        }

        let mut border_points: Vec<(i32,i32)> = vec![]; 
        for cell_key in district.cell_keys.iter() {
            if let Some(_cell) = district_map.cells.get(cell_key) {
                if !district_map.cells.contains_key(&(cell_key.0+1, cell_key.1)) {
                    border_points.push((cell_key.0+1, cell_key.1));
                    continue;
                }
                if !district_map.cells.contains_key(&(cell_key.0-1, cell_key.1)) {
                    border_points.push((cell_key.0-1, cell_key.1));
                    continue;
                }
                if !district_map.cells.contains_key(&(cell_key.0, cell_key.1+1)) {
                    border_points.push((cell_key.0, cell_key.1+1));
                    continue;
                }
                if !district_map.cells.contains_key(&(cell_key.0, cell_key.1-1)) {
                    border_points.push((cell_key.0, cell_key.1-1));
                    continue;
                }
            }
        }

        if border_points.len() <= 0 {
            return;
        }

        let new_district = District {
            district_type: random_type,
            cell_keys: vec![border_points[rand::random_range(0..border_points.len())]],
            is_fertile: rand::random_bool(0.7),
            ..default()
        };

        district_map.cells.insert(new_district.cell_keys[0], DistrictCell { population: 0, max_population: 5 });
        district_map.districts.push(new_district);
    }
}

fn grow_districts(
    mut district_map: ResMut<DistrictMap>,
) {
    for district in district_map.districts.clone().iter().filter(|&dist| !dist.is_completed) {
        if district.cell_keys.len() >= district.max_size {
            let index = district_map.districts.iter().position(|dist| *dist == *district).unwrap();
            district_map.districts[index].is_completed=true;
            return;
        }

        let mut new_district = district.clone();
        for cell_key in district.cell_keys.iter() {
            if let Some(cell) = district_map.cells.get_mut(cell_key) {
                if cell.population < cell.max_population {
                    cell.population+=rand::random_range(0..3);
                    continue;
                }

                if !district_map.cells.contains_key(&(cell_key.0+1, cell_key.1)) {
                    district_map.cells.insert((cell_key.0+1, cell_key.1), DistrictCell { population: 0, max_population: 5 });
                    new_district.cell_keys.push((cell_key.0+1, cell_key.1));
                    break;
                }
                if !district_map.cells.contains_key(&(cell_key.0-1, cell_key.1)) {
                    district_map.cells.insert((cell_key.0-1, cell_key.1), DistrictCell { population: 0, max_population: 5 });
                    new_district.cell_keys.push((cell_key.0-1, cell_key.1));
                    break;
                }
                if !district_map.cells.contains_key(&(cell_key.0, cell_key.1+1)) {
                    district_map.cells.insert((cell_key.0, cell_key.1+1), DistrictCell { population: 0, max_population: 5 });
                    new_district.cell_keys.push((cell_key.0, cell_key.1+1));
                    break;
                }
                if !district_map.cells.contains_key(&(cell_key.0, cell_key.1-1)) {
                    district_map.cells.insert((cell_key.0, cell_key.1-1), DistrictCell { population: 0, max_population: 5 });
                    new_district.cell_keys.push((cell_key.0, cell_key.1-1));
                    break;
                }
            }            
        }

        let index = district_map.districts.iter().position(|dist| *dist == *district).unwrap();
        district_map.districts[index] = new_district;
    }
}

fn get_angle(center: Vec2, point: Vec2) -> f32 {
    let diff = point-center;
    let mut angle = diff.y.atan2(diff.x);

    if angle <= 0.0 {
        angle += 2.*PI;
    }

    angle
}

fn get_distance(center: Vec2, point: Vec2) -> f32 {
    let diff = point-center;
    (diff.x.powi(2) + diff.y.powi(2)).sqrt()
}

fn sort_border_points(
    border_points: &mut Vec<Vec2>,
) {
    let centroid = Vec2::new(
        border_points.iter().map(|&point| point.x).sum::<f32>() / border_points.len() as f32,
        border_points.iter().map(|&point| point.y).sum::<f32>() / border_points.len() as f32
    );

    border_points.sort_by(|&a, &b| {        
        let angle1 = get_angle(centroid, a);
        let angle2 = get_angle(centroid, b);
        
        let hypotenouse1 = get_distance(centroid, a);
        let hypotenouse2 = get_distance(centroid, b);

        if angle1 < angle2 {
            return Ordering::Less;
        }
 
        if angle1 == angle2 && hypotenouse1 < hypotenouse2 {
            return Ordering::Less;
        }

        return Ordering::Greater;

        // (angle1, hypotenouse1).partial_cmp(&(angle2, hypotenouse2)).unwrap()
    });
}

fn test_draw_district(
    district_map: Res<DistrictMap>,
    mut gizmos: Gizmos, 
) {
    for district in district_map.districts.clone().iter() {
        let mut border_points: Vec<Vec2> = vec![]; 
        for cell_key in district.cell_keys.iter() {
            if let Some(_cell) = district_map.cells.get(cell_key) {
                if district.cell_keys.contains(&(cell_key.0+1, cell_key.1)) &&
                   district.cell_keys.contains(&(cell_key.0-1, cell_key.1)) &&
                   district.cell_keys.contains(&(cell_key.0, cell_key.1+1)) &&
                   district.cell_keys.contains(&(cell_key.0, cell_key.1-1)) {
                    continue;
                }
                border_points.push(Vec2::new(cell_key.0 as f32, cell_key.1 as f32) * DISTRICT_CELL_SIZE);
            }
        }
        sort_border_points(&mut border_points);

        let Ok(curve) = CubicCardinalSpline::new_catmull_rom(border_points.clone()).to_curve_cyclic() else { return };

        let resolution = 100 * curve.segments().len();
        gizmos.linestrip(
            curve.iter_positions(resolution).map(|pt| pt.extend(0.0)),
            district.district_type.color()
        );

        for point in district.cell_keys.iter() {
            gizmos.rect_2d(Isometry2d::from_xy(point.0 as f32 * DISTRICT_CELL_SIZE, point.1 as f32 * DISTRICT_CELL_SIZE), Vec2::splat(5.), district.district_type.color());
        }
    }
}