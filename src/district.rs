use std::time::Duration;

use bevy::{prelude::*, time::common_conditions::on_timer};

use crate::{passenger::AddPassengerEvent, GameState, DISTRICT_CELL_SIZE, MAX_DISTRICT_SIZE};

pub struct DistrictPlugin;

impl Plugin for DistrictPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<DistrictMap>();
        app.add_systems(OnEnter(GameState::InGame), (
            test_gen_district,
        ));
        app.add_systems(Update, (
            grow_districts
                .run_if(on_timer(Duration::from_millis(500))),
            start_new_districts
                .run_if(on_timer(Duration::from_millis(1000))),
            draw_district_cells
        )
        .run_if(in_state(GameState::InGame)));
    }
}

#[derive(Copy, Clone, PartialEq)]
pub enum DistrictType {
    Home,
    Work,
    Entertainment,
}

impl DistrictType {
    fn color(&self) -> Color {
        match self {
            Self::Home => Color::srgb(0.8, 0.4, 0.1),
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
pub (crate) struct District {
    is_completed: bool,
    derivatives_amount: u32,
    is_fertile: bool,
    max_size: usize,

    pub district_type: DistrictType,
    pub id: usize,
    pub(crate) passenger_ids: Vec<usize>,
    pub cells: Vec<(i32, i32)>,
}

impl Default for District {
    fn default() -> Self {
        Self {
            district_type: DistrictType::Home,
            is_completed: false,
            derivatives_amount: 0,
            is_fertile: true,
            cells: vec![],
            max_size: MAX_DISTRICT_SIZE,
            id: 0,
            passenger_ids: vec![]
        }
    }
}

#[derive(Resource, Default)]
pub struct DistrictMap {
    pub(crate) districts: Vec<District>,
    cells: Vec<(i32, i32)>,
}

fn test_gen_district(
    mut district_map: ResMut<DistrictMap>,
) {
    let mut district = District {
        id: district_map.districts.len(),
        district_type: DistrictType::Home,
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
                    false => DistrictType::Home,
                    true => DistrictType::Work,
                };
            },
            DistrictType::Home => {
                random_type = match rand::random_bool(0.5) {
                    false => DistrictType::Entertainment,
                    true => DistrictType::Work,
                };
            },
            DistrictType::Work => {
                random_type = match rand::random_bool(0.5) {
                    false => DistrictType::Home,
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
            id: district_map.districts.len(),
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
    mut ev_add_passenger: EventWriter<AddPassengerEvent>, 
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

        // каждую вторую клетку добавляем пассажира в район (т.е. на 24 клетки района должно прийтись 12 пассажиров)
        if new_district.cells.len() % 4 == 0
        && new_district.district_type == DistrictType::Home {
            ev_add_passenger.send(AddPassengerEvent {
                district_id: new_district.id
            });
        }

        district_map.districts[district.id] = new_district;
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