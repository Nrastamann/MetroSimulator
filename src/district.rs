use std::{cmp::Ordering, f32::consts::PI, time::Duration};

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
                .run_if(on_timer(Duration::from_millis(1000))),
            start_new_districts
                .run_if(on_timer(Duration::from_millis(1000))),
            draw_district_cells
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

#[derive(Component, PartialEq, Copy, Clone)]
pub struct DistrictCell {
    position: (i32, i32)
}

#[derive(Clone, PartialEq)]
struct District {
    district_type: DistrictType,
    is_completed: bool,
    derivatives_amount: u32,
    is_fertile: bool,
    cells: Vec<(i32, i32)>,
    max_size: usize,
}

impl Default for District {
    fn default() -> Self {
        Self {
            district_type: DistrictType::Living,
            is_completed: false,
            derivatives_amount: 0,
            is_fertile: true,
            cells: vec![],
            max_size: rand::random_range(0..40) + MAX_DISTRICT_SIZE
        }
    }
}

#[derive(Resource, Default)]
pub struct DistrictMap {
    districts: Vec<District>,
    cells: Vec<(i32, i32)>,
}

fn test_gen_district(
    mut district_map: ResMut<DistrictMap>,
) {
    let mut district = District {
        district_type: DistrictType::Living,
        ..default()
    };


    district.cells.push((0,0));
    district_map.cells.push((0,0));

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
        for cell in district.cells.iter() {
            if !district_map.cells.contains(&(cell.0+1, cell.1)) {
                border_points.push((cell.0+1, cell.1));
                continue;
            }
            if !district_map.cells.contains(&(cell.0-1, cell.1)) {
                border_points.push((cell.0-1, cell.1));
                continue;
            }
            if !district_map.cells.contains(&(cell.0, cell.1+1)) {
                border_points.push((cell.0, cell.1+1));
                continue;
            }
            if !district_map.cells.contains(&(cell.0, cell.1-1)) {
                border_points.push((cell.0, cell.1-1));
                continue;
            }
        }

        if border_points.len() <= 0 {
            return;
        }

        let new_district = District {
            district_type: random_type,
            cells: vec![border_points[rand::random_range(0..border_points.len())]],
            is_fertile: rand::random_bool(0.7),
            ..default()
        };

        district_map.cells.push(new_district.cells[0]);
        district_map.districts.push(new_district);
    }
}

fn grow_districts(
    mut district_map: ResMut<DistrictMap>,
) {
    for district in district_map.districts.clone().iter().filter(|&dist| !dist.is_completed) {
        if district.cells.len() >= district.max_size {
            let index = district_map.districts.iter().position(|dist| *dist == *district).unwrap();
            district_map.districts[index].is_completed=true;
            return;
        }

        let mut new_district = district.clone();
        for cell in district.cells.iter() {
            if !district_map.cells.contains(&(cell.0+1, cell.1)) {
                district_map.cells.push((cell.0+1, cell.1));
                new_district.cells.push((cell.0+1, cell.1));
                break;
            }
            if !district_map.cells.contains(&(cell.0-1, cell.1)) {
                district_map.cells.push((cell.0-1, cell.1));
                new_district.cells.push((cell.0-1, cell.1));
                break;
            }
            if !district_map.cells.contains(&(cell.0, cell.1+1)) {
                district_map.cells.push((cell.0, cell.1+1));
                new_district.cells.push((cell.0, cell.1+1));
                break;
            }
            if !district_map.cells.contains(&(cell.0, cell.1-1)) {
                district_map.cells.push((cell.0, cell.1-1));
                new_district.cells.push((cell.0, cell.1-1));
                break;
            }
        }

        let index = district_map.districts.iter().position(|dist| *dist == *district).unwrap();
        district_map.districts[index] = new_district;
    }
}

fn draw_district_cells (
    mut commands: Commands,
    q_cell: Query<&DistrictCell>,
    district_map: Res<DistrictMap>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let spawned_cells: Vec<(i32,i32)> = q_cell.iter().map(|cell| cell.position).collect();

    // фильтруем клетки так, чтобы не спавнить повторно те, что уже заспавнены
    for district in district_map.districts.iter() {
        for cell in district.cells.iter()
        .filter(|&cell| !spawned_cells.contains(cell)) {
            let mesh = meshes.add(Rectangle::new(DISTRICT_CELL_SIZE, DISTRICT_CELL_SIZE));
            let material = materials.add(district.district_type.color().with_alpha(0.5));

            commands.spawn((
                Mesh2d(mesh),
                MeshMaterial2d(material),
                DistrictCell { position: *cell },
                Transform::from_xyz(
                    cell.0 as f32 * DISTRICT_CELL_SIZE,
                    cell.1 as f32 * DISTRICT_CELL_SIZE, -5.0)
            ));
        }
    }
}