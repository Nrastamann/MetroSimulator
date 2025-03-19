use bevy::prelude::*;

use crate::metro::Metro;

pub struct TrainPlugin;

impl Plugin for TrainPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnTrainEvent>();
        app.add_systems(Update, (spawn_train, move_train, switch_train_direction));
    }
}

#[derive(Event)]
pub struct SpawnTrainEvent {
    pub line: usize
}

#[derive(PartialEq)]
enum TrainDirection {
    Forwards,
    Backwards
}

#[derive(Component)]
pub struct Train {
    line: usize,
    current: usize,
    direction: TrainDirection
}

impl Train {
    fn new(line: usize) -> Self {
        Self {
            line,
            current: 0,
            direction: TrainDirection::Forwards
        }
    } 
}

fn spawn_train(
    mut commands: Commands,
    mut ev_spawn: EventReader<SpawnTrainEvent>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    metro: Res<Metro>
) {
    for ev in ev_spawn.read() {
        let mesh = meshes.add(Rectangle::new(30., 15.));
        let material = materials.add(Color::BLACK);
        
        let position = metro.lines[ev.line].points[0];

        commands.spawn((
            Mesh2d(mesh),
            MeshMaterial2d(material),
            Transform::from_translation(Vec3::new(
                position.0 as f32,
                position.1 as f32, 2.0
            )),
            Train::new(ev.line)
        ));
    }
}

fn get_closest(positions: &Vec<Vec2>, target: &Vec2, direction: &TrainDirection) -> (Vec2, usize) {
    let mut sorted = positions.clone(); 

    sorted.sort_by(|pos1, pos2| {
        pos1.distance(*target).total_cmp(&pos2.distance(*target))
    });

    match direction {
        TrainDirection::Forwards => {
            let index = positions.iter().position(|p| *p == sorted[0]).unwrap();
            if index+1 >= positions.len() {
                return (positions[index], index)
            }
            return (positions[index+1], index+1);
        },
        TrainDirection::Backwards => {
            let index = positions.iter().position(|p| *p == sorted[0]).unwrap();
            if index <= 0 {
                return (positions[index], index);
            }
            return (positions[index-1], index-1);
        }
    }
}

fn get_current(positions: &Vec<Vec2>, target: &Vec2) -> usize {
    let mut sorted = positions.clone(); 
    sorted.sort_by(|pos1, pos2| {
        pos1.distance(*target).total_cmp(&pos2.distance(*target))
    });

    positions.iter().position(|p| *p == sorted[0]).unwrap()
}

fn move_train(
    mut q_train: Query<(&mut Transform, &mut Train)>,
    metro: Res<Metro>,
    time: Res<Time>,
) {
    for (mut train_transform, mut train) in q_train.iter_mut() {
        let line = &metro.lines[train.line];
        let Some(curve) = &line.curve else { return };
        let curve_positions: Vec<Vec2> = curve.iter_positions(32 * curve.segments().len()).collect();
        let (closest_point, closest_index) = get_closest(&curve_positions, &train_transform.translation.truncate(), &train.direction);

        train.current = closest_index;

        let diff = closest_point.extend(train_transform.translation.z) - train_transform.translation;
        let angle = diff.y.atan2(diff.x);
        train_transform.rotation = train_transform.rotation.lerp(Quat::from_rotation_z(angle), 12.0 * time.delta_secs());

        let direction = curve_positions[closest_index] - train_transform.translation.truncate();
        train_transform.translation += direction.normalize().extend(0.) * 100.0 * time.delta_secs();
    }
}

fn switch_train_direction(
    mut q_train: Query<(&Transform, &mut Train)>,
    metro: Res<Metro>,
) {
    for (train_transform, mut train) in q_train.iter_mut() {
        let line = &metro.lines[train.line];
        let Some(curve) = &line.curve else { return };
        let curve_positions: Vec<Vec2> = curve.iter_positions(32 * curve.segments().len()).collect();

        if train.current == 0 && train.direction == TrainDirection::Backwards {
            train.direction = TrainDirection::Forwards;
        }
        if train.current == curve_positions.len()-1 && train.direction == TrainDirection::Forwards {
            train.direction = TrainDirection::Backwards;
        }
    }
}