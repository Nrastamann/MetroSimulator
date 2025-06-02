use std::time::Duration;

use crate::{
    audio::ChangeTrackEvent,
    cursor::CursorPosition,
    settings::{ChangeSettingEvent, Settings, SettingsType},
    GameState,
};
use bevy::prelude::*;
use bevy_lunex::*;
const SETTINGS_LEN: usize = 5;
const SLIDER_SIZE: f32 = 10.;
const SETTINGS_NAME: [&str; SETTINGS_LEN] = [
    "Звуковые эффекты",
    "Звуки метро",
    "Громкость музыки",
    "Громкость звуковых эффектов",
    "Громкость звуков метро",
];

const SETTING_SIZE: f32 = 11.;
const SETTINGS_OFFSET: f32 = 15.;
const NAMING_SIZE: f32 = 60.;
use super::{
    TutorialSpawnEvent, UIStyles, METRO_LIGHT_BLUE_COLOR, OPACITY_LEVEL_BLUR,
    OPACITY_LEVEL_HIGHEST, OPACITY_LEVEL_MAIN, UI_FONT,
};
pub struct SettingsUIPlugin;

#[derive(Component)]
pub struct ButtonFunction(usize);

impl Plugin for SettingsUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<CalculateSliderEvent>();
        app.init_resource::<TimerForRedraw>();
        app.add_event::<RedrawSlidersEvent>();
        app.add_systems(OnEnter(GameState::Settings), SettingsUI::spawn);
        app.add_systems(
            Update,
            (exit_hotkey, move_slider, redraw_from_values, locate_slider, recalculate_sliders, timer_tick)
                .run_if(in_state(GameState::Settings)),
        );
    }
}
#[derive(Component)]
pub struct CheckBox {
    pub pressed: bool,
    pub setting_type: SettingsType,
}

impl Default for CheckBox {
    fn default() -> Self {
        Self {
            pressed: true,
            setting_type: SettingsType::TurnOfMetroSFX,
        }
    }
}

#[derive(Component)]
pub struct Slider {
    picked: bool,
    pub value: f32,
    pub setting_type: SettingsType,
}
impl Default for Slider {
    fn default() -> Self {
        Self {
            picked: false,
            value: 1.0,
            setting_type: SettingsType::MusicVolume,
        }
    }
}

#[derive(Resource)]
pub struct TimerForRedraw{
    timer: Timer
}
impl Default for TimerForRedraw{
    fn default() -> Self {
        Self { timer: Timer::new(Duration::from_millis(250), TimerMode::Once) }
    }
}


#[derive(Component)]
pub struct SettingsUI;
impl SettingsUI {
    fn spawn(mut commands: Commands, asset_server: Res<AssetServer>, ) {
        commands
            .spawn((UiLayoutRoot::new_2d(), UiFetchFromCamera::<0>, SettingsUI))
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
                                    .rl_pos(10.,10.)
                                    .rl_size(NAMING_SIZE, 80.)
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
                                    .y(Rl(10.))
                                    .x(Rl(NAMING_SIZE + 10.))
                                    .rl_size(100. - NAMING_SIZE - 10., 80.)
                                    .pack(),
                            ))
                            .with_children(|ui| {
                                let mut settings_offset = 0.;
                                for i in 0..SETTINGS_LEN {
                                    if i >= SettingsType::MusicVolume as usize {
                                        ui.spawn((
                                            Name::new("A"),
                                            UiLayoutTypeWindow::new()
                                                .rl_pos(0., settings_offset)
                                                .rl_size(100., SETTING_SIZE * 0.8 + 1.)
                                                .pack(),
                                            Sprite {
                                                color: Color::BLACK,
                                                image: asset_server.load("button_symetric.png"),
                                                image_mode: SpriteImageMode::Sliced(
                                                    TextureSlicer {
                                                        border: BorderRect::square(32.0),
                                                        ..default()
                                                    },
                                                ),
                                                ..Default::default()
                                            },
                                            UiColor::from(Color::BLACK),
                                        ))
                                        .with_children(
                                            |ui| {
                                                ui.spawn((
                                                    Name::new("SLIDER ITSELF"),
                                                    UiLayoutTypeWindow::new()
                                                        .rl_pos(100. - SLIDER_SIZE, 0.)
                                                        .rl_size(SLIDER_SIZE, 100.)
                                                        .pack(),
                                                    Sprite {
                                                        color: Color::BLACK,
                                                        image: asset_server
                                                            .load("button_symetric.png"),
                                                        image_mode: SpriteImageMode::Sliced(
                                                            TextureSlicer {
                                                                border: BorderRect::square(8.0),
                                                                ..default()
                                                            },
                                                        ),
                                                        ..Default::default()
                                                    },
                                                    Slider {
                                                        setting_type: i.into(),
                                                        ..Default::default()
                                                    },
                                                    UiColor::from(Color::BLACK),
                                                ));
                                            },
                                        );
                                    }else{
                                        ui.spawn((
                                            Name::new("B"),
                                            UiLayoutTypeWindow::new()
                                                .rl_pos(0., settings_offset)
                                                .rl_size(SETTING_SIZE+10.,SETTING_SIZE)
                                                .pack(),
                                            Sprite {
                                                color: Color::BLACK,
                                                image: asset_server.load("checkbox_2.png"),
                                                ..Default::default()
                                            },
                                            CheckBox{
                                                setting_type: i.into(),
                                                ..default()
                                            },
                                        )).observe(|click: Trigger<Pointer<Click>>, asset_server: Res<AssetServer>, mut button_q: Query<(&mut Sprite,&mut CheckBox)>|{
                                        let (mut sprite, mut checkbox) = button_q.get_mut(click.target).unwrap();
                                            if checkbox.pressed{
                                                sprite.image = asset_server.load("checkbox_1.png");
                                                checkbox.pressed = false;
                                                return;
                                            }
                                            sprite.image = asset_server.load("checkbox_2.png");
                                            checkbox.pressed = true;
                                        });
                                    }
                                    settings_offset += SETTINGS_OFFSET;
                                }
                            });
                            ui.spawn((
                                Name::new("Button apply"),
                                UiLayoutTypeWindow::new().anchor_left().rl_pos(0.,90.).rl_size(100.,10.).pack(),
                            )).with_children(|ui|{
                                for i in 0..2{
                                    let mut text = "Сохранить настройки";
                                    if i == 1{
                                        text = "Сбросить до начальных";
                                    }
                                let mut button_e =ui.spawn((
                                    Name::new("Button"),
                                    UiLayoutTypeWindow::new().rl_pos(50. * i as f32, 0.).rl_size(50., 100.).pack(),
                                    Sprite::default(),
                                    UiColor::new(vec![
                                        (UiBase::id(), Color::WHITE.with_alpha(OPACITY_LEVEL_BLUR)),
                                        (UiHover::id(), METRO_LIGHT_BLUE_COLOR.with_alpha(OPACITY_LEVEL_BLUR)),
                                    ]),
//                                    UiHover::new().forward_speed(20.0).backward_speed(4.0),
                                    ButtonFunction(i),
                                ));
                                button_e.with_children(|ui|{
                                    ui.spawn((
                                        Name::new("Text"),
                                        UiLayoutTypeWindow::new().anchor_center().pack(),
                                        UiColor::new(vec![
                                            (UiBase::id(), Color::BLACK),
                                            (UiHover::id(), Color::WHITE),
                                        ]),
//                                        UiHover::new().forward_speed(20.0).backward_speed(4.0),
                                        UiTextSize::from(Rh(80.)),
                                        Text2d::new(text.to_string()),
                                        TextFont {
                                            font: asset_server.load(UI_FONT),
                                            font_size: 96.,
                                            ..default()
                                        },
                                        PickingBehavior::IGNORE,

                                    ));
                                });
                                button_e
//                                .observe(hover_set::<Pointer<Over>, true>)
//                                .observe(hover_set::<Pointer<Out>, false>)
                                .observe(|click: Trigger<Pointer<Click>>, mut settings: ResMut<Settings>,mut button_q: Query<&ButtonFunction>,mut redraw_ev: EventWriter<RedrawSlidersEvent>,mut change_settings_ev: EventWriter<ChangeSettingEvent>|{
                                    if button_q.get_mut(click.target).is_ok(){
                                        match button_q.get_mut(click.target).unwrap().0{
                                        0 =>{
                                            change_settings_ev.send(ChangeSettingEvent);
                                        }
                                        _ =>{
                                            settings.metro_sfx_volume = 1.0;
                                            settings.music_volume = 1.0;
                                            settings.sfx_volume = 1.0;
                                            settings.turn_on_metro_sfx = true;
                                            settings.turn_on_sfx = true;
                                            
                                            redraw_ev.send(RedrawSlidersEvent);
                                        }
                                    }
                                    }else{
                                        println!("Error");
                                    }
                                });
                            }
                            });
                        });
                    });
                });
            });
    }
}

fn timer_tick(time: Res<Time>,mut timer: ResMut<TimerForRedraw>,mut redraw_ev: EventWriter<RedrawSlidersEvent>){
    timer.timer.tick(time.delta());
    if timer.timer.just_finished(){
        redraw_ev.send(RedrawSlidersEvent);
    }
}

#[derive(Event)]
pub struct RedrawSlidersEvent;

fn redraw_from_values(mut redraw_ev: EventReader<RedrawSlidersEvent>,settings: Res<Settings>,
    mut slider_q: Query<(&mut Transform, &GlobalTransform, &Parent, &mut Slider), Without<CheckBox>>,
    mut checkboxes: Query<(&mut Sprite, &CheckBox),Without<Slider>>,
    asset_server: Res<AssetServer>,
){
    for _ev in redraw_ev.read(){
        println!("RUST POBEEDA");
        for (mut sprite, checkbox) in checkboxes.iter_mut(){
            if checkbox.setting_type == SettingsType::TurnOfMetroSFX {
                if settings.turn_on_metro_sfx{
                    sprite.image = asset_server.load("checkbox_2.png");
                    continue;    
                }
                sprite.image = asset_server.load("checkbox_1.png");
                continue;
            }
            if settings.turn_on_sfx{
                sprite.image = asset_server.load("checkbox_2.png");                
                continue;
            }
            sprite.image = asset_server.load("checkbox_1.png");
        }

        for (mut transform, transform_g, parent, slider) in slider_q.iter_mut(){
            let ratio;
            match slider.setting_type {
                SettingsType::MusicVolume =>{
                    ratio = settings.music_volume;
                }
                SettingsType::SFXMetroVolume =>{
                    ratio = settings.metro_sfx_volume;
                }
                _ => {
                    ratio = settings.sfx_volume;
                }
            }
            println!("RUST what {}", transform.translation.x);
            let x_new;
            if ratio > 0.5{
                x_new = 138. * ratio;
            }else{
                x_new = -138. * (1. - ratio);
            }
            transform.translation.x = x_new;
            println!("RUST who {}", transform.translation.x);
        }
    }
}

#[derive(Event)]
pub struct CalculateSliderEvent;

fn recalculate_sliders(
    mut calculate_slider_ev: EventReader<CalculateSliderEvent>,
    mut slider_q: Query<(&GlobalTransform, &Parent, &mut Slider)>,
    mut global_transform_q: Query<(&GlobalTransform, &Dimension), Without<Slider>>,
) {
    for _ev in calculate_slider_ev.read() {
        for (global_t, parent, mut slider) in slider_q.iter_mut(){
            let (pos,size) = global_transform_q.get_mut(**parent).unwrap();
            //220., 497

            let mut ratio = (global_t.translation().x - 497.).abs() / (497.-220.);

            if ratio <= 0.01{
                ratio = 0.0;
            }

            if ratio >= 0.99{
                ratio = 1.;
            }

            slider.value = ratio;
        }
    }
}


fn locate_slider(
    mut slider_q: Query<(&GlobalTransform, &mut Slider)>,
    cursor_position: Res<CursorPosition>,
    mouse: ResMut<ButtonInput<MouseButton>>,
    mut calculate_slider_ev: EventWriter<CalculateSliderEvent>,
) {
    if mouse.just_released(MouseButton::Left) {
        for mut i in slider_q.iter_mut() {
            i.1.picked = false;
        }
        calculate_slider_ev.send(CalculateSliderEvent);
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
        println!(
            "Parent pos - {} {}, cursor - {} {} ",
            slider_t.as_ref().unwrap().0.translation.x,
            slider_t.as_ref().unwrap().0.translation.y,
            cursor_position.0.x,
            cursor_position.0.y
        );
        println!("Dimension - {} {}", dimension.x, dimension.y);
        let mut difference = cursor_position.0.x - slider_t.as_ref().unwrap().1.translation().x;
        if difference > 0.
            && parent_pos.translation().x + dimension.x / 2. - 15. < cursor_position.0.x
        {
            difference = 0.0;
        } else if difference < 0.
            && parent_pos.translation().x - dimension.x / 2. + 15. > cursor_position.0.x
        {
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
