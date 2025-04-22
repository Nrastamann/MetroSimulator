use bevy::prelude::*;
use bevy_lunex::*;

use crate::GameState;

use super::UIStyles;

pub struct TutorialUIPlugin;

impl Plugin for TutorialUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<TutorialSpawnEvent>()
            .add_systems(OnEnter(GameState::InGame), Tutorial::spawn_tutorial);
    }
}
const TUTORIAL_TASKS: [&str; 6] = [
    "Подвигай камерой",
    "Воспользуйся зумом",
    "Открой меню строительства",
    "Продли ветку",
    "Построй новую ветку",
    "Купи поезд",
];
const TUTORIAL_HINTS: [&str; 6] = [
    "С помощью WASD ты можешь двигать карту.",
    "Используя колесо мышки  ты можешь приближать и отдалять карту.",
    "Чтобы открыть меню станции нажми ПКМ на станцию.",
    "Чтобы продлить ветку, нужно выбрать одну из существующих веток и нажать 'новая станция'.",
    "Чтобы построить новую ветку, нужно открыть меню станции и нажать 'новая ветка'.",
    "Поезд на выбранную ветку можно купить также в меню станции.",
];
#[derive(Component)]
pub struct Tutorial;

#[derive(Event)]
pub struct TutorialSpawnEvent;
impl Tutorial {
    fn spawn_tutorial(
        mut commands: Commands,
        mut spawn_tutorial_ev: EventReader<TutorialSpawnEvent>,
    ) {
        for _ in spawn_tutorial_ev.read() {
            commands
                .spawn((UiLayoutRoot::new_2d(), UiFetchFromCamera::<0>, Tutorial))
                .with_children(|ui| {
                    ui.spawn(UiLayoutTypeWindow::new().full().pack())
                        .with_children(|ui| {
                            ui.spawn((
                                Name::new("Hints window"),
                                UiLayoutTypeWindow::new()
                                    .anchor_left()
                                    .rl_pos(10., 70.)
                                    .rl_size(80., 20.)
                                    .pack(),
                                UiColor::from(Color::srgba(255., 255., 255., 0.5)),
                                Sprite::default(),
                            ));

                            ui.spawn((
                                Name::new("Tasks to complete"),
                                UiLayoutTypeWindow::new()
                                    .anchor_left()
                                    .rl_size(25., 40.)
                                    .pack(),
                                UiColor::from(Color::srgba(255., 255., 255., 0.5)),
                                Sprite::default(),
                            ));
                        });
                });
        }
    }
}
