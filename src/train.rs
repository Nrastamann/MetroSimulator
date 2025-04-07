use bevy::prelude::*;

use crate::metro::{Direction, Metro};

const TRAIN_STOP_TIME_SECS: f32 = 1.0;
const TRAIN_SPEED: f32 = 100.0;

pub struct TrainPlugin;

impl Plugin for TrainPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnTrainEvent>();
        app.add_systems(Update, (spawn_train, move_train, stop_train, switch_train_direction));
    }
}

#[derive(Event)]
pub struct SpawnTrainEvent {
    pub line: usize,
    pub color: Color,
}

#[derive(Component)]
pub struct Train {
    line: usize,
    current: usize,
    direction: Direction
}

#[derive(Component)]
struct TrainStop {
    timer: Timer,
}

impl Train {
    fn new(line: usize) -> Self {
        Self {
            line,
            current: 0,
            direction: Direction::Forwards
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
        let material = materials.add(ev.color);
        
        let Some(station) = metro.lines[ev.line].stations.front() else { return };
        let position = station.position;

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

fn get_closest(positions: &Vec<Vec2>, target: &Vec2, direction: &Direction) -> (Vec2, usize) {
    let mut sorted = positions.clone(); 

    sorted.sort_by(|pos1, pos2| {
        pos1.distance(*target).total_cmp(&pos2.distance(*target))
    });

    match direction {
        Direction::Forwards => {
            let index = positions.iter().position(|p| *p == sorted[0]).unwrap();
            if index+1 >= positions.len() {
                return (positions[index], index)
            }
            return (positions[index+1], index+1);
        },
        Direction::Backwards => {
            let index = positions.iter().position(|p| *p == sorted[0]).unwrap();
            if index <= 0 {
                return (positions[index], index);
            }
            return (positions[index-1], index-1);
        }
    }
}

fn move_train(
    mut commands: Commands,
    mut q_train: Query<(Entity, &mut Transform, &mut Train), Without<TrainStop>>,
    metro: Res<Metro>,
    time: Res<Time>,
) {
    for (e_train, mut train_transform, mut train) in q_train.iter_mut() {
        let line = &metro.lines[train.line];
        let Some(curve) = &line.curve else { return };
        let curve_positions: Vec<Vec2> = curve.iter_positions(32 * curve.segments().len()).collect();
        let (closest_point, closest_index) = get_closest(&curve_positions, &train_transform.translation.truncate(), &train.direction);

        train.current = closest_index;

        let closest_point_tuple = (
            closest_point.x.floor() as i32,
            closest_point.y.floor() as i32,
        );
        if line.stations.iter().map(|station| station.position).collect::<Vec<(i32, i32)>>().contains(&closest_point_tuple) {
            commands.entity(e_train).insert(TrainStop { timer: Timer::from_seconds(TRAIN_STOP_TIME_SECS, TimerMode::Once) });
        }

        let diff = closest_point.extend(train_transform.translation.z) - train_transform.translation;
        let angle = diff.y.atan2(diff.x);
        train_transform.rotation = train_transform.rotation.lerp(Quat::from_rotation_z(angle), 12.0 * time.delta_secs());

        let direction = curve_positions[closest_index] - train_transform.translation.truncate();
        train_transform.translation += direction.normalize().extend(0.) * TRAIN_SPEED * time.delta_secs();
    }
}

fn stop_train(
    mut commands: Commands,
    mut q_train: Query<(Entity, &mut TrainStop)>,
    time: Res<Time>,
) {
    for (e_train, mut train_stop) in q_train.iter_mut() {
        train_stop.timer.tick(time.delta());

        if train_stop.timer.just_finished() {
            commands.entity(e_train).remove::<TrainStop>();
        }
    }
}

fn switch_train_direction(
    mut q_train: Query<&mut Train>,
    metro: Res<Metro>,
) {
    for mut train in q_train.iter_mut() {
        let line = &metro.lines[train.line];
        let Some(curve) = &line.curve else { return };
        let curve_positions: Vec<Vec2> = curve.iter_positions(32 * curve.segments().len()).collect();

        if train.current == 0 && train.direction == Direction::Backwards {
            train.direction = Direction::Forwards;
        }
        if train.current == curve_positions.len()-1 && train.direction == Direction::Forwards {
            train.direction = Direction::Backwards;
        }
    }
}