use std::default;

use bevy::{
    input::mouse::{MouseButtonInput, MouseWheel},
    prelude::*,
    state::commands,
};
use bevy_lunex::*;

use crate::{
    camera::MainCamera, district::DistrictMap, metro::Metro, money::Money,
    passenger::PassengerDatabase, GameState,
};

use super::{
    LinesResource, RedrawEvent, TextboxResource, UIStyles, METRO_LIGHT_BLUE_COLOR, UI_FONT,
};

pub struct TutorialUIPlugin;

impl Plugin for TutorialUIPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Progress>();
        app.enable_state_scoped_entities::<GameState>();
        app.add_event::<BuyTrainTutorial>()
            .add_event::<TutorialSpawnEvent>()
            .add_event::<RedrawTextEvent>()
            .add_event::<BuildingLineTutorial>()
            .add_event::<ProlongLineTutorial>();
        app.add_systems(Update, (track_progress, rewrite_text_of_hint))
            .add_systems(OnEnter(GameState::InGame), Tutorial::spawn_tutorial);

        app.add_systems(
            OnExit(GameState::InGame),
            (
                clear_resource::<Metro>,
                clear_resource::<PassengerDatabase>,
                clear_resource::<DistrictMap>,
                clear_resource::<Money>,
                clear_resource::<TextboxResource>,
                clear_resource::<LinesResource>,
            ),
        );
    }
}

#[derive(Default, Copy, Clone)]
pub enum TasksInTutorial {
    #[default]
    MoveCamera,
    UseZoom,
    OpenBuildMenu,
    ProlongTheLine,
    BuildNewLine,
    BuyTrain,
    EndOfTutorial,
}

#[derive(Event)]
pub struct BuyTrainTutorial;

#[derive(Event)]
pub struct BuildingLineTutorial;

#[derive(Event)]
pub struct ProlongLineTutorial;

#[derive(Resource, Default, Copy, Clone)]
pub struct Progress {
    current_task: TasksInTutorial,
}
const TASKS_OFFSET: f32 = 15.;
const TASKS_SIZE: f32 = 12.;
const TUTORIAL_TASKS: [&str; 6] = [
    "Подвигай камерой",
    "Воспользуйся зумом",
    "Открой меню строительства",
    "Продли ветку",
    "Построй новую ветку",
    "Купи поезд",
];

// const TUTORIAL_HINTS: [&str; 6] = [
//     "     С помощью WASD\nты можешь двигать карту.",
//     "Используя колесо мышки\n ты можешь приближать\n     и отдалять карту.",
//     "     Чтобы открыть\n меню станции нажми\n   ПКМ на станцию.",
//     "Чтобы продлить ветку, нужно выбрать одну из существующих веток и нажать 'новая станция'.",
//     "Чтобы построить новую ветку, нужно открыть меню станции и нажать 'новая ветка'.",
//     "Поезд на выбранную ветку можно купить также в меню станции.",
// ];

const TUTORIAL_HINTS: [&str; 7] = [
    "С помощью WASD\nты можешь двигать карту.",
    "Используя колесо мышки\nты можешь приближать\nи отдалять карту.",
    "Чтобы открыть меню станции\nнажми ПКМ на станцию.",
    "Чтобы продлить ветку,\nнужно выбрать одну из\nсуществующих веток и нажать\n'новая станция'.",
    "Чтобы построить новую ветку,\nнужно открыть меню станции\nи нажать 'новая линия'.",
    "Поезд на выбранную ветку\nможно купить также в\nменю станции.",
    "Обучение на этом заканчивается\nВы можете продолжить играть в нем\n или же выйти в меню с помощью кнопки справа сверху.\nЧтобы закрыть окно подсказок - нажмите q."
];
#[derive(Component)]
struct HintToRedraw;

#[derive(Event)]
pub struct RedrawTextEvent;

#[derive(Component)]
pub struct Tutorial;

#[derive(Event)]
pub struct TutorialSpawnEvent;
impl Tutorial {
    fn spawn_tutorial(
        mut commands: Commands,
        mut spawn_tutorial_ev: EventReader<TutorialSpawnEvent>,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<ColorMaterial>>,
        asset_server: Res<AssetServer>,
    ) {
        for _ in spawn_tutorial_ev.read() {
            commands
                .spawn((
                    UiLayoutRoot::new_2d(),
                    StateScoped(GameState::InGame),
                    UiFetchFromCamera::<0>,
                    Tutorial,
                ))
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
                                UiColor::from(Color::srgba(255., 255., 255., 0.2)),
                                Sprite::default(),
                            ))
                            .with_children(|ui| {
                                ui.spawn((
                                    Name::new("Hint"),
                                    UiLayoutTypeWindow::new().anchor_center().pack(),
                                    UiColor::from(Color::BLACK.with_alpha(0.95)),
                                    UiTextSize::from(Rh(100.)),
                                    Text2d::new(TUTORIAL_HINTS[0].to_string()),
                                    TextFont {
                                        font: asset_server.load(UI_FONT),
                                        font_size: 96.,
                                        ..default()
                                    },
                                    TextLayout {
                                        justify: JustifyText::Center,
                                        linebreak: LineBreak::WordBoundary,
                                    },
                                    HintToRedraw,
                                ));
                            });

                            ui.spawn((
                                Name::new("Tasks to complete"),
                                UiLayoutTypeWindow::new()
                                    .anchor_left()
                                    .rl_size(25., 40.)
                                    .pack(),
                                UiColor::from(Color::srgba(255., 255., 255., 0.2)),
                                Sprite::default(),
                            ))
                            .with_children(|ui| {
                                ui.spawn((
                                    Name::new("Checkmarks handler"),
                                    UiLayoutTypeWindow::new()
                                        .anchor_left()
                                        .rl_size(20., 100.)
                                        .pack(),
                                ))
                                .with_children(|ui| {
                                    let mut current_pos = 0.;
                                    for _ in TUTORIAL_TASKS.iter() {
                                        ui.spawn((
                                            Name::new("Checkmark boundary"),
                                            UiLayoutTypeWindow::new()
                                                .anchor_left()
                                                .rl_pos(0., current_pos)
                                                .rl_size(100., TASKS_SIZE)
                                                .pack(),
                                        ))
                                        .with_children(
                                            |ui| {
                                                ui.spawn((
                                                    Name::new(""),
                                                    Mesh2d(meshes.add(Circle::new(17.5))),
                                                    MeshMaterial2d(materials.add(Color::WHITE)),
                                                    Transform::from_translation(Vec3::new(
                                                        0.0, 0.0, 2.0,
                                                    )),
                                                ));
                                            },
                                        );
                                        current_pos += TASKS_OFFSET;
                                    }
                                });
                                ui.spawn((
                                    Name::new("Task text handler"),
                                    UiLayoutTypeWindow::new()
                                        .anchor_left()
                                        .rl_pos(20., 0.)
                                        .rl_size(80., 100.)
                                        .pack(),
                                ))
                                .with_children(|ui| {
                                    let mut current_pos: f32 = 0.;
                                    for text in TUTORIAL_TASKS.iter() {
                                        ui.spawn((
                                            Name::new("TextHandler"),
                                            UiLayoutTypeWindow::new()
                                                .anchor_left()
                                                .rl_pos(0., current_pos)
                                                .rl_size(100., TASKS_SIZE)
                                                .pack(),
                                        ))
                                        .with_children(
                                            |ui| {
                                                ui.spawn((
                                                    Name::new(*text),
                                                    UiLayoutTypeWindow::new()
                                                        .anchor_left()
                                                        .rl_pos(0., 17.5)
                                                        .pack(),
                                                    UiColor::from(Color::BLACK.with_alpha(0.95)),
                                                    UiTextSize::from(Rh(67.5)),
                                                    Text2d::new(text.to_string()),
                                                    TextFont {
                                                        font: asset_server.load(UI_FONT),
                                                        font_size: 96.,
                                                        ..default()
                                                    },
                                                ));
                                            },
                                        );
                                        current_pos += TASKS_OFFSET;
                                    }
                                });
                            });
                        });
                });
        }
    }
}

fn track_progress(
    ev_mouse_wheel: EventReader<MouseWheel>,
    keyboard: Res<ButtonInput<KeyCode>>,
    time: ResMut<Time>,
    mut progress: ResMut<Progress>,
    mut camera_q: Query<&mut Transform, With<MainCamera>>,
    redraw_menu: EventReader<RedrawEvent>,
    prolong_line_ev: EventReader<ProlongLineTutorial>,
    build_line_tutorial_ev: EventReader<BuildingLineTutorial>,
    mut redraw_hint: EventWriter<RedrawTextEvent>,
    buy_train_ev: EventReader<BuyTrainTutorial>,
    tutorial_ui_root_q: Query<Entity, (With<UiLayoutRoot>, With<Tutorial>)>,
    mut hint: Query<&mut Text2d, With<HintToRedraw>>,
    mut commands: Commands,
    mut state_manager: ResMut<NextState<GameState>>,
) {
    //add some text as easter egg, like why are you taking so long to complete the task
    match progress.current_task {
        TasksInTutorial::MoveCamera => {
            if hint.iter().len() <= 0 {
                return;
            }
            if keyboard.just_pressed(KeyCode::KeyW)
                || keyboard.just_pressed(KeyCode::KeyA)
                || keyboard.just_pressed(KeyCode::KeyS)
                || keyboard.just_pressed(KeyCode::KeyD)
            {
                progress.current_task = TasksInTutorial::UseZoom;
                redraw_hint.send(RedrawTextEvent);
            }
        }
        TasksInTutorial::UseZoom => {
            if !ev_mouse_wheel.is_empty() {
                progress.current_task = TasksInTutorial::OpenBuildMenu;
                redraw_hint.send(RedrawTextEvent);
            }
        }
        TasksInTutorial::OpenBuildMenu => {
            if !redraw_menu.is_empty() {
                progress.current_task = TasksInTutorial::ProlongTheLine;
                redraw_hint.send(RedrawTextEvent);
            }
        }
        TasksInTutorial::ProlongTheLine => {
            if !prolong_line_ev.is_empty() {
                progress.current_task = TasksInTutorial::BuildNewLine;
                redraw_hint.send(RedrawTextEvent);
            }
        }
        TasksInTutorial::BuildNewLine => {
            if !build_line_tutorial_ev.is_empty() {
                progress.current_task = TasksInTutorial::BuyTrain;
                redraw_hint.send(RedrawTextEvent);
            }
        }
        TasksInTutorial::BuyTrain => {
            if !buy_train_ev.is_empty() {
                progress.current_task = TasksInTutorial::EndOfTutorial;
                redraw_hint.send(RedrawTextEvent);
            }
        }
        TasksInTutorial::EndOfTutorial => {
            if keyboard.just_pressed(KeyCode::KeyQ) {
                commands
                    .entity(tutorial_ui_root_q.get_single().unwrap())
                    .despawn_recursive();
            }
            if keyboard.just_pressed(KeyCode::KeyE) {
                state_manager.set(GameState::MainMenu);
                camera_q.get_single_mut().unwrap().translation = Vec3::new(0., 0., 0.);
                //add delete on smth.
            }
        }
    }
}

fn rewrite_text_of_hint(
    mut redraw_hint_ev: EventReader<RedrawTextEvent>,
    tutorial_progress: Res<Progress>,
    mut hint: Query<&mut Text2d, With<HintToRedraw>>,
) {
    for _ev in redraw_hint_ev.read() {
        let mut text = hint.get_single_mut().unwrap();

        text.0 = TUTORIAL_HINTS[tutorial_progress.current_task as usize].to_string();
    }
}

fn clear_resource<T: Resource + Default>(mut commands: Commands, resource: ResMut<T>) {
    commands.insert_resource(T::default());
}
