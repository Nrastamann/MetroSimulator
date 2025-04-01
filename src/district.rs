use std::{collections::HashMap, time::Duration};

use bevy::{prelude::*, time::common_conditions::on_timer};

use crate::{DISTRICT_CELL_SIZE, MAX_DISTRICT_SIZE};

pub struct DistrictPlugin;

impl Plugin for DistrictPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<DistrictMap>();
        app.add_systems(Startup, (
            test_gen_district,
        ));
        app.add_systems(Update, (
            grow_districts
                .run_if(on_timer(Duration::from_millis(100))),
            test_draw_district));
    }
}

#[derive(Clone, PartialEq)]
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
    is_spawned_complete: bool,
    cell_keys: Vec<(i32, i32)>
}

impl Default for District {
    fn default() -> Self {
        Self {
            district_type: DistrictType::Living,
            is_completed: false,
            is_spawned_complete: false,
            cell_keys: vec![]
        }
    }
}

#[derive(Resource, Default)]
pub struct DistrictMap {
    districts: Vec<District>,
    cells: HashMap<(i32, i32), DistrictCell>
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

fn grow_districts(
    mut district_map: ResMut<DistrictMap>,
) {
    for district in district_map.districts.clone().iter().filter(|&dist| !dist.is_completed) {
        if district.cell_keys.len() >= MAX_DISTRICT_SIZE {
            let index = district_map.districts.iter().position(|dist| *dist == *district).unwrap();
            district_map.districts[index].is_completed=true;

            let random_type = match rand::random_range(0..3) {
                0 => DistrictType::Living,
                1 => DistrictType::Work,
                2 => DistrictType::Entertainment,
                _ => DistrictType::Living
            };

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

            let new_district = District {
                district_type: random_type,
                cell_keys: vec![border_points[rand::random_range(0..border_points.len())]],
                ..default()
            };

            district_map.cells.insert(new_district.cell_keys[0], DistrictCell { population: 0, max_population: 5 });
            district_map.districts.push(new_district);

            return;
        }

        let mut new_district = district.clone();
        for cell_key in district.cell_keys.iter() {
            if let Some(cell) = district_map.cells.get_mut(cell_key) {
                if cell.population < cell.max_population {
                    cell.population+=1;
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
        for point in border_points.iter() {
            gizmos.rect_2d(Isometry2d::from_xy(point.x, point.y), Vec2::splat(2.), district.district_type.color());
        }
    }
}