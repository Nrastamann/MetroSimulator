use crate::{audio::ChangeTrackEvent, cursor::CursorPosition, GameState};
use bevy::prelude::*;
use bevy_lunex::*;
const SETTINGS_LEN: usize = 5;
const SLIDER_SIZE: f32 = 10.;
const SETTINGS_NAME: [&str; SETTINGS_LEN] = [
    "Отключить звуковые эффекты",
    "Отключить звуки метро",
    "Громкость музыки",
    "Громкость звуковых эффектов",
    "Громкость звуков метро",
];
pub enum SettingsType {
    TurnOfSFX = 0,
    TurnOfMetroSFX,
    MusicVolume,
    SFXVolume,
    SFXMetroVolume,
}

impl From<usize> for SettingsType {
    fn from(value: usize) -> Self {
        match value {
            _ if value == SettingsType::TurnOfSFX as usize => Self::TurnOfSFX,
            _ if value == SettingsType::TurnOfMetroSFX as usize => Self::TurnOfMetroSFX,
            _ if value == SettingsType::MusicVolume as usize => Self::MusicVolume,
            _ if value == SettingsType::SFXVolume as usize => Self::SFXVolume,
            _ if value == SettingsType::SFXMetroVolume as usize => Self::SFXMetroVolume,
            _ => panic!("WRONG VALUE"),
        }
    }
}
const SETTING_SIZE: f32 = 11.;
const SETTINGS_OFFSET: f32 = 15.;
const NAMING_SIZE: f32 = 60.;
use super::{
    TutorialSpawnEvent, UIStyles, METRO_LIGHT_BLUE_COLOR, OPACITY_LEVEL_BLUR,
    OPACITY_LEVEL_HIGHEST, OPACITY_LEVEL_MAIN, UI_FONT,
};
pub struct SettingsUIPlugin;
#[derive(Component)]
pub struct Flag;

impl Plugin for SettingsUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Settings), Settings::spawn);
        app.add_systems(
            Update,
            (exit_hotkey, move_slider, locate_slider).run_if(in_state(GameState::Settings)),
        );
    }
}
#[derive(Component)]
pub struct Slider {
    picked: bool,
}
impl Default for Slider {
    fn default() -> Self {
        Self { picked: false }
    }
}

#[derive(Component)]
pub struct Settings;
impl Settings {
    fn spawn(
        mut commands: Commands,
        asset_server: Res<AssetServer>,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<ColorMaterial>>,
    ) {
        commands
            .spawn((UiLayoutRoot::new_2d(), UiFetchFromCamera::<0>, Settings))
            .with_children(|ui| {
                ui.spawn((
                    Name::new("Settings BG"),
                    UiLayoutTypeWindow::new().full().pack(),
                    StateScoped(GameState::Settings),
                    Sprite {
                        image: asset_server.load("Settings.png"),
                        ..default()
                    }, //add some background to settings, make it blur?
                ))
                .with_children(|ui| {
                    ui.spawn((
                        Name::new("Settings"),
                        UiLayoutTypeWindow::new().full().pack(),
                        Sprite::default(),
                        UiColor::from(METRO_LIGHT_BLUE_COLOR.with_alpha(OPACITY_LEVEL_HIGHEST)),
                    ))
                    .with_children(|ui| {
                        ui.spawn((
                            Name::new("Settings background"),
                            UiLayoutTypeWindow::new()
                                .anchor_left()
                                .rl_pos(10., 10.)
                                .rl_size(80., 80.)
                                .pack(),
                            Sprite::default(),
                            UiColor::from(Color::WHITE.with_alpha(OPACITY_LEVEL_MAIN)),
                        ))
                        .with_children(|ui| {
                            ui.spawn((
                                Name::new("Text part"),
                                UiLayoutTypeWindow::new()
                                    .anchor_left()
                                    .rl_size(NAMING_SIZE, 100.)
                                    .pack(),
                            ))
                            .with_children(|ui| {
                                let mut current_offset = 0.;
                                for i in SETTINGS_NAME {
                                    ui.spawn((
                                        Name::new("settings name"),
                                        UiLayoutTypeWindow::new()
                                            .anchor_left()
                                            .rl_pos(0., current_offset)
                                            .rl_size(100., SETTING_SIZE)
                                            .pack(),
                                    ))
                                    .with_children(|ui| {
                                        ui.spawn((
                                            Name::new("text"),
                                            UiLayoutTypeWindow::new().anchor_left().pack(),
                                            UiColor::from(Color::BLACK),
                                            UiTextSize::from(Rh(80.)),
                                            Text2d::new(i.to_string()),
                                            TextFont {
                                                font: asset_server.load(UI_FONT),
                                                font_size: 96.,
                                                ..default()
                                            },
                                        ));
                                    });
                                    current_offset += SETTINGS_OFFSET;
                                }
                            });
                            ui.spawn((
                                Name::new("Settings set part"),
                                UiLayoutTypeWindow::new()
                                    .anchor_left()
                                    .x(Rl(NAMING_SIZE + 10.))
                                    .rl_size(100. - NAMING_SIZE - 10., 100.)
                                    .pack(),
                            ))
                            .with_children(|ui| {
                                let mut settings_offset = 0.;
                                for i in 0..SETTINGS_LEN {
                                    match i.into() {
                                        SettingsType::TurnOfSFX => {}
                                        _ => {}
                                    }
                                    ui.spawn((
                                        Name::new("A"),
                                        UiLayoutTypeWindow::new()
                                            .rl_pos(0., settings_offset)
                                            .rl_size(100., SETTING_SIZE * 0.8 + 1.)
                                            .pack(),
                                        Sprite {
                                            color: Color::BLACK,
                                            image: asset_server.load("button_symetric.png"),
                                            image_mode: SpriteImageMode::Sliced(TextureSlicer {
                                                border: BorderRect::square(32.0),
                                                ..default()
                                            }),
                                            ..Default::default()
                                        },
                                        UiColor::from(Color::BLACK),
                                    ))
                                    .with_children(|ui| {
                                        ui.spawn((
                                            Name::new("SLIDER ITSELF"),
                                            UiLayoutTypeWindow::new()
                                                .rl_pos(100. - SLIDER_SIZE, 0.)
                                                .rl_size(SLIDER_SIZE, 100.)
                                                .pack(),
                                            Sprite {
                                                color: Color::BLACK,
                                                image: asset_server.load("button_symetric.png"),
                                                image_mode: SpriteImageMode::Sliced(
                                                    TextureSlicer {
                                                        border: BorderRect::square(8.0),
                                                        ..default()
                                                    },
                                                ),
                                                ..Default::default()
                                            },
                                            Slider::default(),
                                            UiColor::from(Color::BLACK),
                                        ));
                                    });
                                    settings_offset += SETTINGS_OFFSET;
                                }
                            });
                        });
                    });
                });
            });
    }
}

fn locate_slider(
    mut slider_q: Query<(&GlobalTransform, &mut Slider)>,
    cursor_position: Res<CursorPosition>,
    mouse: ResMut<ButtonInput<MouseButton>>,
) {
    if mouse.just_released(MouseButton::Left) {
        for mut i in slider_q.iter_mut() {
            i.1.picked = false;
        }
    }
    if slider_q
        .iter_mut()
        .filter(|(_, slider)| slider.picked == true)
        .next()
        .is_none()
    {
        for (global_t, mut slider) in slider_q.iter_mut() {
            if global_t
                .translation()
                .truncate()
                .distance(cursor_position.0)
                < 50.
                && mouse.just_pressed(MouseButton::Left)
            {
                slider.picked = true;
                break;
            }
        }
    }
}

fn move_slider(
    mut slider_q: Query<(&mut Transform, &GlobalTransform, &Slider, &Parent)>,
    global_t: Query<(&GlobalTransform, &Dimension), Without<Slider>>,
    mouse: ResMut<ButtonInput<MouseButton>>,
    cursor_position: Res<CursorPosition>,
) {
    let slider_t = slider_q
        .iter_mut()
        .filter(|(_, _, slider, _)| slider.picked == true)
        .next();
    if slider_t.is_some() {
        let (parent_pos, dimension) = global_t.get(**slider_t.as_ref().unwrap().3).unwrap();
        println!("Parent pos - {} {}, cursor - {} {} ", parent_pos.translation().x,parent_pos.translation().y,cursor_position.0.x,cursor_position.0.y);
        println!("Dimension - {} {}", dimension.x,dimension.y);
        let mut difference = cursor_position.0.x - slider_t.as_ref().unwrap().1.translation().x;
        if difference > 0. && parent_pos.translation().x + dimension.x / 2. - 15. < cursor_position.0.x{
            difference = 0.0;
        }else if difference < 0. && parent_pos.translation().x - dimension.x / 2. + 15. > cursor_position.0.x{
            difference = 0.0;
        }
        if mouse.pressed(MouseButton::Left) {
            slider_t.unwrap().0.translation.x += difference;
        }
    }
}
//307.2 56.448
//358.4 -85.82
fn exit_hotkey(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut state_manager: ResMut<NextState<GameState>>,
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        state_manager.set(GameState::MainMenu);
    }
}
