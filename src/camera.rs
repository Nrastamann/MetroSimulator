use bevy::{input::mouse::MouseWheel, prelude::*};
use bevy_lunex::{Dimension, UiLayoutRoot, UiSourceCamera};

use crate::{ui::{PopupMenu, POPUP_HEIGHT, POPUP_WIDTH}, GameState};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera);
        app.add_systems(
            Update,
            (move_camera, zoom_camera).run_if(in_state(GameState::InGame)),
        );
    }
}

#[derive(Component)]
pub struct MainCamera {
    move_speed: f32,
    max_zoom: f32,
    min_zoom: f32,
    pub target_zoom: f32,
}

impl Default for MainCamera {
    fn default() -> Self {
        MainCamera {
            move_speed: 500.,
            max_zoom: 10.,
            min_zoom: 1.,
            target_zoom: 1.,
        }
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.insert_resource(ClearColor(Color::srgb(1.0, 0.9, 0.9)));
    commands.spawn((Camera2d, MainCamera::default(), UiSourceCamera::<0>));
}

fn move_camera(
    mut q_camera: Query<(&mut Transform, &MainCamera)>,
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    let Ok((mut camera_transform, camera)) = q_camera.get_single_mut() else {
        return;
    };

    let mut direction = Vec2::ZERO;

    if keyboard.pressed(KeyCode::KeyA) {
        direction.x -= 1.;
    }
    if keyboard.pressed(KeyCode::KeyD) {
        direction.x += 1.;
    }
    if keyboard.pressed(KeyCode::KeyS) {
        direction.y -= 1.;
    }
    if keyboard.pressed(KeyCode::KeyW) {
        direction.y += 1.;
    }

    camera_transform.translation += direction.extend(0.0) * camera.move_speed * camera.target_zoom * time.delta_secs();
}

fn zoom_camera(
    mut q_camera: Query<(&mut OrthographicProjection, &mut MainCamera)>,
    mut ev_mouse_wheel: EventReader<MouseWheel>,
    time: Res<Time>,
    mut popup_q: Query<(&mut Visibility, &mut Dimension), (With<UiLayoutRoot>, With<PopupMenu>)>,
) {
    let Ok((mut ortho, mut camera)) = q_camera.get_single_mut() else {
        return;
    };

    use bevy::input::mouse::MouseScrollUnit;
    for ev in ev_mouse_wheel.read() {
        match ev.unit {
            MouseScrollUnit::Line => {
                if ev.y > 0.0 && camera.target_zoom - 0.25 >= camera.min_zoom {
                    camera.target_zoom -= 0.25;
                }
                if ev.y < 0.0 && camera.target_zoom + 0.25 <= camera.max_zoom {
                    camera.target_zoom += 0.25;
                }
                let Ok((mut vision, mut size)) = popup_q.get_single_mut() else {
                    return;
                };

                *vision = Visibility::Hidden;
                *size = Dimension::from((
                    POPUP_WIDTH * camera.target_zoom,
                    POPUP_HEIGHT * camera.target_zoom,
                ));
            }
            _ => {}
        }
    }
    ortho.scale = ortho
        .scale
        .lerp(camera.target_zoom, 15. * time.delta_secs());
}
