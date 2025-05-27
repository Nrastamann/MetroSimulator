use bevy::{
    prelude::*,
};
use bevy_lunex::*;

use crate::{
    audio::{ChangeOrderOfPlaying, ChangeTrackEvent, MusicPlayer, PlayerMode, Soundtrack, MUSIC_NAMES},
    GameState,
};
pub const PLAYER_SIGNS: [&str; 3] = ["По порядку", "Пауза", "Мут"];
pub const PLAYER_BOT_BUTTONS: [&str; 3] = ["Влево", "Номер стр", "Вправо"];
pub const PLAYER_TOP_BUTTONS: [&str; 3] = ["Влево", "Пауза", "Вправо"];
pub const SMALL_PLAYER_SIZE: f32 = 20.;
use super::{
    LinesResource, RedrawEvent, TextboxResource, UIStyles, METRO_BLUE_COLOR, METRO_LIGHT_BLUE_COLOR, OPACITY_LEVEL_HIGHEST, OPACITY_LEVEL_MAIN, UI_FONT
};

pub struct AudioUIPlugin;

impl Plugin for AudioUIPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PlayerEntities>()
            .add_event::<HideUIEvent>()
            .add_event::<ShowUIEvent>()
            .add_event::<ChangeSongNameEvent>()
            .add_event::<ChangeSongNameEvent>()
            .add_event::<PlayerUISpawnEvent>()
            .add_event::<RedrawTracksEvent>();
        app.add_systems(
            Update,
            (show_player, hide_player, redraw_tracks, hotkey_player,change_song_mini_player).run_if(in_state(GameState::InGame)),
            (show_player, hide_player, redraw_tracks, hotkey_player,change_song_mini_player).run_if(in_state(GameState::InGame)),
        );
        app.add_systems(OnEnter(GameState::InGame), PlayerUI::spawn);
    }
}
#[derive(Event)]
pub struct RedrawTracksEvent{
    redraw_type: RedrawType,
}

#[derive(PartialEq)]
pub enum RedrawType{
    ChangePage,
    Redraw,
    ChangePicked(usize),
}

pub enum ButtonState{
    Pause,
    Play,
    Mute,
    Loud,
    Shuffle,
    Line,
    Repeat,
}

#[derive(Component)]
pub struct ButtonStateComponent{
    button_state: ButtonState,
}

#[derive(Component)]
pub struct PlayerUI;

#[derive(Resource, Default)]
pub struct PlayerEntities {
    entities_text: Vec<Entity>,
    entities_tracks: Vec<Entity>
}

#[derive(Component)]
pub struct PlayerType(usize);

#[derive(Component)]
pub struct PlayerType(usize);

#[derive(Component)]
pub struct PageNumber(pub usize);

#[derive(Component, Default)]
pub struct PlayerButton(pub usize);

#[derive(Component)]
pub struct CurrentTrack;

#[derive(Component)]
pub struct CurrentTrackSmall;

#[derive(Component)]
pub struct CurrentTrackSmall;

#[derive(Event)]
pub struct HideUIEvent;

#[derive(Event)]
pub struct ShowUIEvent;

#[derive(Event)]
pub struct ChangeSongNameEvent;

#[derive(Event)]
pub struct ChangeSongNameEvent;

#[derive(Clone,Copy, PartialEq)]
pub enum ComponentOrientation{
    Left,
    Right
}
#[derive(Component,Clone,Copy)]
pub struct MiniButtons(usize);

#[derive(Component,Clone,Copy)]
pub struct MiniButtons(usize);

#[derive(Component, Clone, Copy)]
pub struct Arrow(pub ComponentOrientation);

pub const MINIPLAYER_OFFSET: f32 = 30.;

pub const MINIPLAYER_OFFSET: f32 = 30.;

#[derive(Event)]
pub struct PlayerUISpawnEvent;
impl PlayerUI {
    fn spawn(
        mut commands: Commands,
        asset_server: Res<AssetServer>,
        music_player: Res<MusicPlayer>,
        music_player: Res<MusicPlayer>,
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
                            Name::new("MiniPlayer"),
                            Visibility::Visible,
                            PlayerType(0),
                            UiLayoutTypeWindow::new().anchor_left().rl_size(SMALL_PLAYER_SIZE, SMALL_PLAYER_SIZE /2.).rl_pos(50. - SMALL_PLAYER_SIZE / 2.,0.).pack(),
                            Sprite::default(),
                            UiColor::from(METRO_BLUE_COLOR),//kokok
                        )).with_children(|ui|{
                            let mut current_offset = 0.;
                            for i in 0..3{
                                let mut additional_size = 0.;
                                if i == 1{
                                    additional_size += 10.;
                                }
                            ui.spawn((
                                Name::new("Button".to_string()+&i.to_string()),
                                UiLayoutTypeWindow::new().anchor_left().rl_pos(current_offset, 0.).rl_size(MINIPLAYER_OFFSET+additional_size,100.).pack(),
                            )).with_children(|ui|{
                                let sprite;
                                let button_type;
                                    if i == 1{
                                    ui.spawn((Name::new("Text"),
                                    UiLayoutTypeWindow::new().anchor_center().pack(),
                                    UiColor::from(Color::BLACK//.with_alpha(0.95)
                                    ),
                                        UiTextSize::from(Rh(100.)),
                                        CurrentTrackSmall,
                                        Text2d::new(MUSIC_NAMES[music_player.current_composition][..MUSIC_NAMES[music_player.current_composition].len()-4].to_string()),
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
                            }else{
                                match i{
                                    0 => {
                                        //buttons
                                        sprite = Sprite{image: asset_server.load("left_button.png"), ..default()};
                                        button_type = 0;
                                    }
                                    2 =>{
                                        sprite = Sprite{image: asset_server.load("right_button.png"), ..default()};
                                        button_type = 1;
                                    }
                                    _ =>{
                                        sprite = Sprite::default();
                                        button_type = 0;
                                        println!("NO SUCH BUTTONaaaaaaaaaaaaa");
                                    }
                                    }
                                    ui.spawn((
                                        Name::new("Buttons"),
                                        UiColor::from(Color::BLACK),
                                        UiLayoutTypeWindow::new().full().pack(),
                                        sprite,
                                        MiniButtons(button_type),

                                    )).observe(|click: Trigger<Pointer<Click>>, mini_buttons_q:Query<&MiniButtons>,music_player: Res<MusicPlayer>, mut change_track: EventWriter<ChangeTrackEvent> |{
                                        match mini_buttons_q.get(click.target).unwrap().0{
                                            0 => {
                                                change_track.send(ChangeTrackEvent { track: Some(usize::MAX) });
                                            }
                                            1 =>{
                                                // if music_player.current_composition >= MUSIC_NAMES.len() - 1{
                                                //     change_track.send(ChangeTrackEvent { track: Some(0) });
                                                //     return;
                                                // }
                                                change_track.send(ChangeTrackEvent { track: Some(usize::MAX - 1) });                                                
                                            }
                                            _ =>{
                                                panic!("there's no such button");
                                            }
                                        }
                                        
                                });
                            }
                            }
                        );
                        current_offset += MINIPLAYER_OFFSET + additional_size;
                        }
                        });
                        
                        ui.spawn((
                            Name::new("Player window"),
                            UiLayoutTypeWindow::new()
                                .anchor_left()
                                .rl_pos(30., 30.)
                                .rl_size(40., 40.)
                                .pack(),
                            UiColor::from(Color::srgba(255., 255., 255., OPACITY_LEVEL_BLUR)),
                            Sprite::default(),
                            PlayerType(1),
                            Visibility::Hidden,
                            PlayerType(1),
                            Visibility::Hidden,
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
                                        UiColor::from(Color::BLACK//.with_alpha(0.95)
                                    ),
                                        UiTextSize::from(Rh(100.)),
                                        CurrentTrack,
                                        Text2d::new(MUSIC_NAMES[0][..MUSIC_NAMES[0].len()-4].to_string()),
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
                                        let mut size = 30.;
                                        if i == "Пауза"{
                                            size = 40.;
                                        }
                                        let component;
                                        let sprite;
                                        let button_state;
                                        match i{
                                            "По порядку" =>{
                                                button_state = ButtonStateComponent{button_state: ButtonState::Line};
                                                component = PlayerButton(0);
                                                sprite = Sprite{image: asset_server.load("mix.png"),..default()}
                                            }
                                            "Пауза" =>{
                                                button_state = ButtonStateComponent{button_state: ButtonState::Pause};
                                                component = PlayerButton(1);
                                                sprite = Sprite{image: asset_server.load("pause.png"),..default()}
                                            }
                                            "Мут" =>{
                                                button_state = ButtonStateComponent{button_state: ButtonState::Loud};
                                                component = PlayerButton(2);
                                                sprite = Sprite{image: asset_server.load("loud.png"),..default()}
                                            }
                                            _ =>{
                                                button_state = ButtonStateComponent{button_state: ButtonState::Loud};
                                                sprite = Sprite::default();
                                                component = PlayerButton(3);
                                            }
                                        }
                                        ui.spawn((
                                            Name::new(i),
                                            UiLayoutTypeWindow::new()
                                                .anchor_left()
                                                .rl_pos(offset, 0.)
                                                .rl_size(size, 100.)
                                                .pack(),
                                                component,
                                        )).with_children(|ui| {
                                            let mut button_e = ui.spawn((
                                                Name::new("Button"),
                                                UiLayoutTypeWindow::new()
                                                .anchor_center()
                                                    .size((Rh(100.), Rl(100.)))
                                                    .pack(),
                                                sprite,
                                                button_state,
                                            ));
                                            button_e.with_children(|ui| {
                                                player_entities.entities_text.push(
                                                    ui.spawn((
                                                        Name::new("ButtonTextx"),
                                                        UiLayout::window().anchor_center().pack(),
                                                        UiColor::from(Color::BLACK),
                                                        // UiTextSize::from(Rh(100.)),
                                                        // Text2d::new(i.to_string()),
                                                        // TextFont {
                                                        //     font: asset_server.load(UI_FONT),
                                                        //     font_size: 96.,
                                                        //     ..default()
                                                        // },
                                                        PickingBehavior::IGNORE,
                                                    ))
                                                    .id(),
                                                );
                                            });
                                            button_e.observe(|clck: Trigger<Pointer<Click>>,mut change_order_ev: EventWriter<ChangeOrderOfPlaying>, mut button_q: Query<(&mut Sprite,&mut ButtonStateComponent)> , mut music: Query<&mut AudioSink, With<Soundtrack>>,mut music_player: ResMut<MusicPlayer>,player_entities: ResMut<PlayerEntities>, asset_server:Res<AssetServer>,mut text_q: Query<&mut Text2d>,| {
                                                let (mut button_sprite, mut button_type) = button_q.get_mut(clck.entity()).unwrap();
                                                
                                                match button_type.button_state{
                                                    ButtonState::Loud => {
                                                            button_sprite.image = asset_server.load("mute.png");
                                                            button_type.button_state = ButtonState::Mute;
                                                            for i in music.iter_mut(){
                                                                i.set_volume(0.);
                                                            }
                                                        }
                                                        ButtonState::Mute =>{
                                                            button_sprite.image = asset_server.load("loud.png");
                                                            button_type.button_state = ButtonState::Loud;
                                                        for i in music.iter_mut(){
                                                            i.set_volume(1.);
                                                        }
                                                    }
    
                                                    ButtonState::Pause =>{
                                                        button_sprite.image = asset_server.load("play.png");
                                                        button_type.button_state = ButtonState::Play;
                                                        for i in music.iter_mut(){
                                                            i.toggle();
                                                        }
                                                    }
    
                                                    ButtonState::Play =>{
                                                        button_sprite.image = asset_server.load("pause.png");
                                                        button_type.button_state = ButtonState::Pause;
                                                        for i in music.iter_mut(){
                                                            i.toggle();
                                                        }
                                                    }
    
                                                    ButtonState::Line =>{
                                                        button_sprite.image = asset_server.load("ShuffleOn.png");//change
                                                        button_type.button_state = ButtonState::Shuffle;
                                                        change_order_ev.send(ChangeOrderOfPlaying);
                                                        music_player.player_mode = PlayerMode::Shuffle;
                                                    }
                                                    
                                                    ButtonState::Shuffle =>{
                                                        button_sprite.image = asset_server.load("repeat.png");
                                                        button_type.button_state = ButtonState::Repeat;
                                                        music_player.player_mode = PlayerMode::SingleRepeat;
                                                    }
    
                                                    ButtonState::Repeat =>{
                                                        button_sprite.image = asset_server.load("mix.png");
                                                        button_type.button_state = ButtonState::Line;
                                                        change_order_ev.send(ChangeOrderOfPlaying);
                                                        music_player.player_mode = PlayerMode::Straight;
                                                    }
                                                    _ =>{
                                                     panic!("MY MAMA");    
                                                    }
                                                }
                                            });
                                        });
                                        offset += size;
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
                                        UiColor::from(Color::srgba(255., 255., 255., OPACITY_LEVEL_BLUR)),

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
                                }).observe(|click:Trigger<Pointer<Click>>,mut music_player: ResMut<MusicPlayer>,mut redraw_tracks_ev: EventWriter<RedrawTracksEvent> ,page_q: Query<&PageNumber>,mut change_track: EventWriter<ChangeTrackEvent>, player_entities: ResMut<PlayerEntities>|{
                                    for i in 0..player_entities.entities_tracks.len(){
                                        if player_entities.entities_tracks[i] == click.target{
                                            if i + (page_q.get_single().unwrap().0 - 1) * 5 >= MUSIC_NAMES.len(){
                                                return;
                                            }
                                            change_track.send(ChangeTrackEvent{track: Some((page_q.get_single().unwrap().0 - 1) * 5 + i)});
                                            redraw_tracks_ev.send(RedrawTracksEvent{redraw_type: RedrawType::ChangePicked(i)});                                                    
                                            break;
                                        }
                                    }
                               
                                }
                            ).id());
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
                                            let sprite;
                                            let visibility;
                                            match i{
                                                "Влево" => {
                                                    arrow = Arrow(ComponentOrientation::Left);
                                                    sprite = Sprite{image: asset_server.load("playleft.png"),..default()};
                                                    visibility = Visibility::Hidden;
                                                }
                                                _ =>{
                                                    arrow = Arrow(ComponentOrientation::Right);
                                                    sprite = Sprite{image: asset_server.load("play.png"),..default()};
                                                    visibility = Visibility::Hidden;
                                                }
                                            }
                                        ui.spawn((
                                            Name::new("Button"),
                                            UiLayoutTypeWindow::new().anchor_center().size(Rh((100.,100.))).pack(),
                                            sprite,
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

                                                            if page.0 <= 1{
                                                                *visibility = Visibility::Hidden;
                                                            }
                                                        }
                                                        ComponentOrientation::Right =>{
                                                            page.0 += 1;
                                                           
                                                            if page.0 * 5 >= MUSIC_NAMES.len(){
                                                                *visibility = Visibility::Hidden;
                                                            }                                                            
                                                        }
                                                    }
                                                    text.0 = page.0.to_string();
                                                    redraw_tracks_ev.send(RedrawTracksEvent{redraw_type: RedrawType::ChangePage});                                                    
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
fn change_song_mini_player(mut change_song_name_ev: EventReader<ChangeSongNameEvent>, music_player: Res<MusicPlayer>, mut name_q: Query<&mut Text2d, With<CurrentTrackSmall>>){
    for _ in change_song_name_ev.read(){
        name_q.get_single_mut().unwrap().0 = MUSIC_NAMES[music_player.current_composition][..MUSIC_NAMES[music_player.current_composition].len()-4].to_string();
    }
}
fn change_song_mini_player(mut change_song_name_ev: EventReader<ChangeSongNameEvent>, music_player: Res<MusicPlayer>, mut name_q: Query<&mut Text2d, With<CurrentTrackSmall>>){
    for _ in change_song_name_ev.read(){
        name_q.get_single_mut().unwrap().0 = MUSIC_NAMES[music_player.current_composition][..MUSIC_NAMES[music_player.current_composition].len()-4].to_string();
    }
}
fn redraw_tracks(mut redraw_tracks_ev: EventReader<RedrawTracksEvent>, 
    mut player_buttons: Query<(&Arrow, &mut Visibility),Without<PlayerType>>,
    mut pages_q: Query<(&mut Text2d,&mut PageNumber), (Without<CurrentTrack>,Without<PlayerType>)>,
    music_player: Res<MusicPlayer>,
    player_enitites: Res<PlayerEntities>,
    mut tracks_holders_q: Query<(&mut UiColor, &Children), (With<UiLayout>, Without<Text2d>,Without<CurrentTrack>,Without<PageNumber>,Without<PlayerType>)>,
    mut track_names_q: Query<(&mut UiColor, &mut Text2d),(Without<PageNumber>,Without<CurrentTrack>,Without<PlayerType>)>,
    mut name_tag_q: Query<&mut Text2d,(With<CurrentTrack>,Without<Children>,Without<UiLayoutRoot>,Without<PlayerType>)>,
    mut player_root_q: Query<(&Visibility, &PlayerType)>,
){
    for ev in redraw_tracks_ev.read(){
        let (mut text_page,mut page_count) = pages_q.get_single_mut().unwrap();
        
        if ev.redraw_type == RedrawType::Redraw{
            page_count.0 = music_player.current_composition / 5 + 1;            
            text_page.0 = page_count.0.to_string(); 
        }
        
        let mut counter = (page_count.0 - 1) * 5;
        for track_e in player_enitites.entities_tracks.iter(){
            let (mut track_handler_color, kids) = tracks_holders_q.get_mut(*track_e).unwrap(); 
            *track_handler_color = UiColor::from(Color::srgba(255., 255., 255., OPACITY_LEVEL_HIGHEST));
            for kid in kids.iter(){
                let (mut color,mut text) = track_names_q.get_mut(*kid).unwrap();
                
                if counter >= MUSIC_NAMES.len(){
                    text.0 = "".to_string();
                    break;
                }
                
                *color = UiColor::from(Color::BLACK);
                text.0 = MUSIC_NAMES[counter][..MUSIC_NAMES[counter].len()-4].to_string();
            }
            counter += 1;
        }

        let picked_track;

        match ev.redraw_type{
            RedrawType::ChangePicked(value) =>{
                picked_track = value;
                //накинуть на value, эффект выбранного
            }
            _ =>{
                if music_player.current_composition / 5 == page_count.0 - 1{
                    picked_track = music_player.current_composition % 5;
                }else{
                picked_track = usize::MAX;
                }
            }
        } 
        name_tag_q.get_single_mut().unwrap().0 = MUSIC_NAMES[music_player.current_composition][..MUSIC_NAMES[music_player.current_composition].len()-4].to_string();

        if picked_track != usize::MAX{
            name_tag_q.get_single_mut().unwrap().0 = MUSIC_NAMES[picked_track + (page_count.0 - 1) * 5][..MUSIC_NAMES[picked_track + (page_count.0 - 1) * 5].len()-4].to_string();

            let (mut track_handler_color, kids) = tracks_holders_q.get_mut(player_enitites.entities_tracks[picked_track]).unwrap(); 
            *track_handler_color = UiColor::from(Color::srgba(0xd3 as f32 /255., 0xd3 as f32 /255., 0xd3 as f32 /255., OPACITY_LEVEL_MAIN));
            for _kid in kids.iter(){
//                let (mut color,mut text) = track_names_q.get_mut(*kid).unwrap();
            }
        }
        let (visibility_p, _) = player_root_q.iter().filter(|(player_v, player_type)|{player_type.0 == 1}).next().unwrap();
        if *visibility_p == Visibility::Visible{
        for (arrow, mut visibility) in player_buttons.iter_mut(){
            if arrow.0 == ComponentOrientation::Left && page_count.0 > 1   {
                *visibility = Visibility::Visible; 
                println!("GOL");
                continue;
            }
            if arrow.0 == ComponentOrientation::Right && page_count.0 * 5 < MUSIC_NAMES.len() {
                *visibility = Visibility::Visible; 
                continue;
            }
        }
        }
    }
}

fn hide_player(
    mut player_q: Query<(&mut Visibility, &PlayerType), Without<Arrow>>,
    mut player_q: Query<(&mut Visibility, &PlayerType), Without<Arrow>>,
    mut hide_ui_ev: EventReader<HideUIEvent>,
    mut buttons_q: Query<&mut Visibility, (With<Arrow>,Without<UiLayoutRoot>)>,
) {
    for _ev in hide_ui_ev.read() {
        for (mut player_v,_) in player_q.iter_mut(){
            if *player_v == Visibility::Hidden{
                *player_v = Visibility::Visible;
                continue;
            }
            *player_v = Visibility::Hidden;
        }
        
        for (mut player_v,_) in player_q.iter_mut(){
            if *player_v == Visibility::Hidden{
                *player_v = Visibility::Visible;
                continue;
            }
            *player_v = Visibility::Hidden;
        }
        
        for mut button in buttons_q.iter_mut(){
            *button = Visibility::Hidden;
        }
    }
}

fn show_player(
    mut player_q: Query<&mut Visibility, (With<PlayerType>, Without<Arrow>)>,
    mut player_q: Query<&mut Visibility, (With<PlayerType>, Without<Arrow>)>,
    mut show_ui_ev: EventReader<ShowUIEvent>,
    mut redraw_tracks_ev: EventWriter<RedrawTracksEvent>, 
    mut buttons_q: Query<&mut Visibility, (With<Arrow>,Without<UiLayoutRoot>, Without<PlayerType>)>,
    mut buttons_q: Query<&mut Visibility, (With<Arrow>,Without<UiLayoutRoot>, Without<PlayerType>)>,
) {
    for _ev in show_ui_ev.read() {
        for mut player_v in player_q.iter_mut(){
            if *player_v == Visibility::Hidden{
                *player_v = Visibility::Visible;
                continue;
            }
            *player_v = Visibility::Hidden;
        }
        
        for mut button in buttons_q.iter_mut(){
            *button = Visibility::Hidden;
        }

        for mut player_v in player_q.iter_mut(){
            if *player_v == Visibility::Hidden{
                *player_v = Visibility::Visible;
                continue;
            }
            *player_v = Visibility::Hidden;
        }
        
        for mut button in buttons_q.iter_mut(){
            *button = Visibility::Hidden;
        }

        redraw_tracks_ev.send(RedrawTracksEvent { redraw_type: RedrawType::Redraw });
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
