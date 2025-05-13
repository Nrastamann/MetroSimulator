use std::{default, ops::DerefMut, path::Component};

use bevy::{
    input::{
        keyboard,
        mouse::{MouseButtonInput, MouseWheel},
    },
    prelude::*,
    state::commands,
};
use bevy_lunex::*;

use crate::{
    audio::{ChangeTrackEvent, MusicPlayer, PlayerMode, Soundtrack, MUSIC_NAMES},
    camera::MainCamera,
    district::DistrictMap,
    metro::Metro,
    money::Money,
    passenger::PassengerDatabase,
    GameState,
};
pub const PLAYER_SIGNS: [&str; 3] = ["По порядку", "Пауза", "Мут"];

use super::{
    LinesResource, RedrawEvent, TextboxResource, UIStyles, METRO_LIGHT_BLUE_COLOR, UI_FONT,
};

pub struct AudioUIPlugin;

impl Plugin for AudioUIPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PlayerEntities>()
            .add_event::<HideUIEvent>()
            .add_event::<ShowUIEvent>()
            .add_event::<PlayerUISpawnEvent>();
        app.add_systems(
            Update,
            (show_player, hide_player, hotkey_player).run_if(in_state(GameState::InGame)),
        );
        app.add_systems(OnEnter(GameState::InGame), PlayerUI::spawn);
    }
}
#[derive(Component)]
pub struct PlayerUI;

#[derive(Resource, Default)]
pub struct PlayerEntities {
    entities_text: Vec<Entity>,
}

#[derive(Component, Default)]
pub struct PlayerButton(pub usize);

#[derive(Event)]
pub struct HideUIEvent;

#[derive(Event)]
pub struct ShowUIEvent;

#[derive(Event)]
pub struct PlayerUISpawnEvent;
impl PlayerUI {
    fn spawn(
        mut commands: Commands,
        mut spawn_tutorial_ev: EventReader<PlayerUISpawnEvent>,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<ColorMaterial>>,
        asset_server: Res<AssetServer>,
        mut player_entities: ResMut<PlayerEntities>,
    ) {
        commands
            .spawn((
                UiLayoutRoot::new_2d(),
                StateScoped(GameState::InGame),
                UiFetchFromCamera::<0>,
                PlayerUI,
                Visibility::Hidden,
            ))
            .with_children(|ui| {
                ui.spawn(UiLayoutTypeWindow::new().full().pack())
                    .with_children(|ui| {
                        ui.spawn((
                            Name::new("Player window"),
                            UiLayoutTypeWindow::new()
                                .anchor_left()
                                .rl_pos(30., 30.)
                                .rl_size(40., 40.)
                                .pack(),
                            UiColor::from(Color::srgba(255., 255., 255., 0.2)),
                            Sprite::default(),
                        ))
                        .with_children(|ui| {
                            ui.spawn((
                                Name::new("Top part"),
                                UiLayoutTypeWindow::new()
                                    .anchor_left()
                                    .rl_size(100., 20.)
                                    .pack(),
                            ))
                            .with_children(|ui| {
                                ui.spawn((
                                    Name::new("NameTag"),
                                    UiLayoutTypeWindow::new()
                                        .anchor_left()
                                        .rl_size(100., 75.)
                                        .pack(),
                                ))
                                .with_children(|ui| {
                                    ui.spawn((
                                        Name::new("TextHandler"),
                                        UiLayoutTypeWindow::new().anchor_center().pack(),
                                        UiColor::from(Color::BLACK.with_alpha(0.95)),
                                        UiTextSize::from(Rh(100.)),
                                        Text2d::new(MUSIC_NAMES[0].to_string()),
                                        TextFont {
                                            font: asset_server.load(UI_FONT),
                                            font_size: 96.,
                                            ..default()
                                        },
                                        TextLayout {
                                            justify: JustifyText::Center,
                                            linebreak: LineBreak::WordBoundary,
                                        },
                                    ));
                                });
                                ui.spawn((
                                    Name::new("Button Handler"),
                                    UiLayoutTypeWindow::new()
                                        .anchor_left()
                                        .y(Rl(75.))
                                        .rl_size(100., 25.)
                                        .pack(),
                                ))
                                .with_children(|ui| {
                                    let mut offset = 0.;
                                    for i in PLAYER_SIGNS {
                                        let component;
                                        match i{
                                            "По порядку" =>{
                                                component = PlayerButton(0);
                                            }
                                            "Пауза" =>{
                                                component = PlayerButton(1);
                                            }
                                            "Мут" =>{
                                                component = PlayerButton(2);
                                            }
                                            _ =>{
                                                component = PlayerButton(3);
                                            }
                                        }
                                        let mut button_e = ui.spawn((
                                            Name::new(i),
                                            UiLayoutTypeWindow::new()
                                                .anchor_left()
                                                .rl_pos(0.5 + offset, 0.)
                                                .rl_size(33., 100.)
                                                .pack(),
                                                component
                                        ));
                                        button_e.with_children(|ui| {
                                            ui.spawn((
                                                Name::new("Button"),
                                                UiLayoutTypeWindow::new()
                                                    .rl_size(100., 100.)
                                                    .pack(),
                                            ))
                                            .with_children(|ui| {
                                                player_entities.entities_text.push(
                                                    ui.spawn((
                                                        Name::new("ButtonTextx"),
                                                        UiLayout::window().anchor_center().pack(),
                                                        UiColor::from(Color::BLACK),
                                                        UiTextSize::from(Rh(100.)),
                                                        Text2d::new(i.to_string()),
                                                        TextFont {
                                                            font: asset_server.load(UI_FONT),
                                                            font_size: 96.,
                                                            ..default()
                                                        },
                                                        PickingBehavior::IGNORE,
                                                    ))
                                                    .id(),
                                                );
                                            });
                                        });
                                        button_e.observe(|clck: Trigger<Pointer<Click>>,mut commands: Commands, button_q: Query<&PlayerButton> , mut music: Query<&mut AudioSink, With<Soundtrack>>,mut music_player: ResMut<MusicPlayer>,player_entities: ResMut<PlayerEntities>, mut text_q: Query<&mut Text2d>,| {
                                            let button_type = button_q.get(clck.entity()).unwrap();
                                        
                                            if button_type.0 > 2{
                                                println!("FUCK WRONG SMTh");
                                                return 
                                            }
                                            
                                            let mut text = text_q.get_mut(player_entities.entities_text[button_type.0]).unwrap();
                                            let text_for_button;
                                            match &*text.0{
                                                "Мут" => {
                                                        text_for_button = "Вернуть громкость";
                                                        for i in music.iter_mut(){
                                                            i.set_volume(0.);
                                                        }
                                                    }
                                                "Вернуть громкость" =>{
                                                    text_for_button = "Мут";
                                                    for i in music.iter_mut(){
                                                        i.set_volume(1.);
                                                    }
                                                }

                                                "Пауза" =>{
                                                    text_for_button = "Продолжить";
                                                    for i in music.iter_mut(){
                                                        i.toggle();
                                                    }
                                                }

                                                "Продолжить" =>{
                                                    text_for_button = "Пауза";
                                                    for i in music.iter_mut(){
                                                        i.toggle();
                                                    }
                                                }

                                                "По порядку" =>{
                                                    text_for_button = "Случайно";

                                                    music_player.player_mode = PlayerMode::Shuffle;
                                                }
                                                
                                                "Случайно" =>{
                                                    text_for_button = "Зациклено";

                                                    music_player.player_mode = PlayerMode::SingleRepeat;
                                                }

                                                "Зациклено" =>{
                                                    text_for_button = "По порядку";

                                                    music_player.player_mode = PlayerMode::Straight;
                                                }
                                                _ =>{
                                                 panic!("MY MAMA");    
                                                }
                                            }
                                            text.0 = text_for_button.to_string();
                                        });

                                        offset += 33.;
                                    }
                                });
                            });
                        });
                    });
            });
    }
}

fn hide_player(
    mut player_q: Query<&mut Visibility, (With<PlayerUI>, With<UiLayoutRoot>)>,
    mut hide_ui_ev: EventReader<HideUIEvent>,
) {
    for _ev in hide_ui_ev.read() {
        let mut player_v = player_q.get_single_mut().unwrap();

        *player_v = Visibility::Hidden;
    }
}

fn show_player(
    mut player_q: Query<&mut Visibility, (With<PlayerUI>, With<UiLayoutRoot>)>,
    mut show_ui_ev: EventReader<ShowUIEvent>,
) {
    for _ev in show_ui_ev.read() {
        let mut player_v = player_q.get_single_mut().unwrap();

        *player_v = Visibility::Visible;
    }
}

fn hotkey_player(
    mut hide_ui: EventWriter<HideUIEvent>,
    mut show_ui: EventWriter<ShowUIEvent>,
    mut player_q: Query<&mut Visibility, (With<PlayerUI>, With<UiLayoutRoot>)>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    if keyboard.just_pressed(KeyCode::KeyP) {
        let player_v = player_q.get_single_mut().unwrap();
        match *player_v {
            Visibility::Visible => {
                hide_ui.send(HideUIEvent);
            }

            Visibility::Hidden => {
                show_ui.send(ShowUIEvent);
            }

            Visibility::Inherited => {
                hide_ui.send(HideUIEvent);
            }
        }
    }
}
