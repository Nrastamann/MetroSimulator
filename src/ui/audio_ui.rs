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
    audio::{ChangeOrderOfPlaying, ChangeTrackEvent, MusicPlayer, PlayerMode, Soundtrack, MUSIC_NAMES},
    camera::MainCamera,
    district::DistrictMap,
    metro::Metro,
    money::Money,
    passenger::PassengerDatabase,
    GameState,
};
pub const PLAYER_SIGNS: [&str; 3] = ["По порядку", "Пауза", "Мут"];
pub const PLAYER_BOT_BUTTONS: [&str; 3] = ["Влево", "Номер стр", "Вправо"];
pub const PLAYER_TOP_BUTTONS: [&str; 3] = ["Влево", "Пауза", "Вправо"];
use super::{
    LinesResource, RedrawEvent, TextboxResource, UIStyles, METRO_LIGHT_BLUE_COLOR, UI_FONT,
};

pub struct AudioUIPlugin;

impl Plugin for AudioUIPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PlayerEntities>()
            .add_event::<HideUIEvent>()
            .add_event::<ShowUIEvent>()
            .add_event::<PlayerUISpawnEvent>()
            .add_event::<RedrawTracksEvent>();
        app.add_systems(
            Update,
            (show_player, hide_player, redraw_tracks, hotkey_player).run_if(in_state(GameState::InGame)),
        );
        app.add_systems(OnEnter(GameState::InGame), PlayerUI::spawn);
    }
}
#[derive(Event)]
pub struct RedrawTracksEvent{
    from_player: bool
}
#[derive(Component)]
pub struct PlayerUI;

#[derive(Resource, Default)]
pub struct PlayerEntities {
    entities_text: Vec<Entity>,
    entities_tracks: Vec<Entity>
}

#[derive(Component)]
pub struct PageNumber(pub usize);

#[derive(Component, Default)]
pub struct PlayerButton(pub usize);

#[derive(Event)]
pub struct HideUIEvent;

#[derive(Event)]
pub struct ShowUIEvent;

#[derive(Clone,Copy, PartialEq)]
pub enum ComponentOrientation{
    Left,
    Right
}
#[derive(Component, Clone, Copy)]
pub struct Arrow(pub ComponentOrientation);

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
                                        button_e.observe(|clck: Trigger<Pointer<Click>>,mut change_order_ev: EventWriter<ChangeOrderOfPlaying>, button_q: Query<&PlayerButton> , mut music: Query<&mut AudioSink, With<Soundtrack>>,mut music_player: ResMut<MusicPlayer>,player_entities: ResMut<PlayerEntities>, mut text_q: Query<&mut Text2d>,| {
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
                                                    change_order_ev.send(ChangeOrderOfPlaying);
                                                    music_player.player_mode = PlayerMode::Shuffle;
                                                }
                                                
                                                "Случайно" =>{
                                                    text_for_button = "Зациклено";
                                                    music_player.player_mode = PlayerMode::SingleRepeat;
                                                }

                                                "Зациклено" =>{
                                                    text_for_button = "По порядку";
                                                    change_order_ev.send(ChangeOrderOfPlaying);
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
                            ui.spawn((
                                Name::new("Bottom Part"), 
                                UiLayoutTypeWindow::new().anchor_left()
                                .y(Rl(20.)).rl_size(100., 80.).pack()
                            )).with_children(|ui|{
                                ui.spawn((
                                    Name::new("Tracklist"),
                                    UiLayoutTypeWindow::new().anchor_left().rl_size(100., 90.).pack()
                                )).with_children(|ui|{
                                    let mut offset: f32 = 0.;
                                    for i in MUSIC_NAMES{
                                        if offset.round() >= 100.0{
                                            break;
                                        }
                                    player_entities.entities_tracks.push(ui.spawn((
                                        Name::new(i.to_string()),
                                        UiLayoutTypeWindow::new().anchor_left().rl_size(100., 20.).y(Rl(offset)).pack(),
                                        Sprite::default(),
                                        UiColor::from(Color::srgba(255., 255., 255., 0.2)),

                                    )).with_children(|ui|{
                                    ui.spawn((
                                       Name::new("Track"),
                                       UiLayoutTypeWindow::new().anchor_center().pack(),
                                       UiColor::from(Color::BLACK),
                                       UiTextSize::from(Rh(100.)),
                                       Text2d::new(i[..i.len()-4].to_string()),
                                       TextFont {
                                           font: asset_server.load(UI_FONT),
                                           font_size: 96.,
                                           ..default()
                                       },
                                       PickingBehavior::IGNORE,
                                    ));
                                }).id());
                            offset+=20.;
                            }
                            });
                                ui.spawn((
                                    Name::new("Buttons section"),
                                    UiLayoutTypeWindow::new().anchor_left().rl_pos(0.,90.).rl_size(100., 10.).pack()
                                )).with_children(|ui|{
                                    let mut offset = 0.;
                                    for i in PLAYER_BOT_BUTTONS{
                                        println!("{}", offset);
                                        let mut size = 30.;
                                        if i == "Номер стр"{
                                            size = 40.;
                                        }
                                        ui.spawn((
                                        Name::new(i.to_string()),
                                        UiLayoutTypeWindow::new().anchor_left().rl_pos(offset, 0.).rl_size(size, 100.).pack(),
                                    )).with_children(|ui|{ 
                                        if i == "Номер стр"{
                                             ui.spawn((
                                                Name::new(i.to_string()),
                                                UiLayoutTypeWindow::new().anchor_center().pack(),
                                                UiColor::from(Color::BLACK),
                                                UiTextSize::from(Rh(100.)),
                                                Text2d::new("1".to_string()),
                                                PageNumber(1),
                                                TextFont {
                                                    font: asset_server.load(UI_FONT),
                                                    font_size: 96.,
                                                    ..default()
                                                },
                                                PickingBehavior::IGNORE,         
                                            ));
                                        }else{
                                            let arrow;
                                            let visibility;
                                            match i{
                                                "Влево" => {
                                                    arrow = Arrow(ComponentOrientation::Left);
                                                    visibility = Visibility::Hidden;
                                                }
                                                _ =>{
                                                    arrow = Arrow(ComponentOrientation::Right);
                                                    visibility = Visibility::Visible;
                                                }
                                            }
                                        ui.spawn((
                                            Name::new("Button"),
                                            UiLayoutTypeWindow::new().full().pack(),
                                            Sprite::from_image(asset_server.load("button.png")),
                                            arrow,
                                            visibility,
                                        )).observe(|clck: Trigger<Pointer<Click>>,
                                            mut redraw_tracks_ev: EventWriter<RedrawTracksEvent>, 
                                            mut button_q: Query<(&Arrow, &mut Visibility)> , 
                                            mut text_q: Query<(&mut Text2d, &mut PageNumber)>,| {
                                                let (button_type, mut visibility) = button_q.get_mut(clck.target).unwrap(); 
                                                if *visibility == Visibility::Hidden{
                                                    return
                                                }

                                                let (mut text, mut page)  = text_q.get_single_mut().unwrap();
                                                    match button_type.0{
                                                        ComponentOrientation::Left =>{
                                                            page.0 -= 1;
                                                        }
                                                        ComponentOrientation::Right =>{
                                                            page.0 += 1;                                                            
                                                        }
                                                    }
                                                    text.0 = page.0.to_string();
                                                    redraw_tracks_ev.send(RedrawTracksEvent{from_player: true});                                                    
                                            });
                                    }
                                    }); 
                                    //spawn buttons there
                                    offset += size;
                                }
                            });
                            });
                        });
                    });
            });
    }
}

fn redraw_tracks(mut redraw_tracks_ev: EventReader<RedrawTracksEvent>, 
    mut player_buttons: Query<(&Arrow, &mut Visibility)>,
    mut pages_q: Query<&mut PageNumber>,){
    for ev in redraw_tracks_ev.read(){
        let page_count = pages_q.get_single().unwrap();

        for (arrow, mut visibility) in player_buttons.iter_mut(){
            if arrow.0 == ComponentOrientation::Left && page_count.0 > 1   {
                *visibility = Visibility::Visible; 
                continue;
            }
            if arrow.0 == ComponentOrientation::Right && page_count.0 * 5 < MUSIC_NAMES.len() {
                *visibility = Visibility::Visible; 
                continue;
            } //FIX THIS SHIT
            print!("{}",page_count.0);
        }
        match ev.from_player{
            true =>{

            }
            _ =>{

            }
        }
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
