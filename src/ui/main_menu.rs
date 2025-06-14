use crate::{audio::ChangeTrackEvent, GameState};
use bevy::prelude::*;
use bevy_lunex::*;

use super::{TutorialSpawnEvent};
pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SwapStatesEvent>()
        .add_systems(Update,changing_states_handler.run_if(in_state(GameState::MainMenu)));
        app.add_systems(OnEnter(GameState::MainMenu), MainMenuScene::spawn).add_systems(OnExit(GameState::MainMenu), despawn_scene_with::<MainMenuScene>);
    }
}

fn despawn_scene_with<S: Component>(mut commands: Commands, query: Query<Entity, With<S>>){
    for entity in &query{
        commands.entity(entity).despawn_recursive();
    }
}
//2b5797
pub const METRO_BLUE_COLOR: Color = Color::srgb(0x45 as f32 /255., 0x79 as f32 /255., 0xAE as f32 /255.);
pub const METRO_LIGHT_BLUE_COLOR: Color = Color::srgb(0x29 as f32 / 255., 0x9b as f32 / 255., 0xe2 as f32 / 255.);
pub const BUTTON_SIZE: f32 = 14.0;
pub const BUTTON_GAP: f32 = 11.0;
pub const MAIN_MENU_BUTTONS: [&str; 4] = ["Новая игра","Обучение","Настройки", "Выйти"];

pub const UI_FONT: &str = "fonts/FiraSans-Medium.ttf";
pub const UI_MENU_FONT: &str = "fonts/metromodern.ttf";

pub enum MainMenuStates {
    NewGame = 0,
    Tutorial,
    Settings,
}

#[derive(Event)]
pub struct SwapStatesEvent{
    move_to_where: MainMenuStates,
}

#[derive(Component)]
pub struct MainMenuScene;

impl MainMenuScene {
    fn spawn(mut commands: Commands, asset_server: Res<AssetServer>) {
        commands
            .spawn((
                UiLayoutRoot::new_2d(),
                UiFetchFromCamera::<0>,
                MainMenuScene,
            ))
            .with_children(|ui| {
                ui.spawn((
                    //background
                    Name::new("Menu"),
                    UiLayout::solid()
                        .size((1920., 1080.))
                        .scaling(Scaling::Fill)
                        .pack(),
                    Sprite::from_image(asset_server.load("check.png")),
                )); //boundary for panel
                ui.spawn((UiLayout::solid().size((1300., 1941.)).align_x(0.85).pack(),))
                    .with_children(|ui| {
                        ui.spawn((
                            Name::new("Panel"),
                            UiLayout::window()
                                .x(Rl(50.))
                                .anchor(Anchor::TopCenter)
                                .size(Rl(105.))
                                .pack(),
                            Sprite::from_color(Color::linear_rgba(1., 1., 1.,0.8), (1., 1.).into()),
                        ));

                        ui.spawn((UiLayout::window()
                            .x(Rl(20.0))
                            .y(Rl(11.0))
                            .size(Rl((80.0, 20.0)))
                            .pack(),))
                            .with_children(|ui| {
                                // Spawn the logo
                                ui.spawn((
                                    Name::new("Logo"),
                                    UiLayout::window().pos((Rh(100.0), Rl(50.0))).anchor(Anchor::Center).pack(),
                                    UiColor::from(METRO_BLUE_COLOR),
                                    UiTextSize::from(Rh(60.)),
                                    Text2d::new("P.O.D.Z.E.M.K.A"),
                                    TextFont{
                                        font:asset_server.load(UI_MENU_FONT),
                                        font_size: 64.,
                                        ..default()
                                    },
                                    PickingBehavior::IGNORE,
                                ));
                            });

                        // Spawn button boundary
                        ui.spawn((UiLayout::window()
                            .pos(Rl((22.0, 44.0)))
                            .size(Rl((55.0, 34.0)))
                            .pack(),))
                            .with_children(|ui| {
                                let mut offset = 0.0;
                                for button in MAIN_MENU_BUTTONS {
                                    let mut button_entity = ui.spawn((
                                        Name::new(button),
                                        UiLayout::window()
                                            .y(Rl(offset))
                                            .size(Rl((100., BUTTON_SIZE)))
                                            .pack(),
                                        OnHoverSetCursor::new(
                                            bevy::window::SystemCursorIcon::Pointer,
                                        ),
                                    ));

                                    button_entity.with_children(|ui|{
                                        ui.spawn((
                                            UiLayout::new(vec![
                                                (UiBase::id(), UiLayout::window().full()),
                                                (UiHover::id(), UiLayout::window().x(Rl(10.0)).full())
                                            ]),
                                            UiHover::new().forward_speed(20.0).backward_speed(4.0),
                                            UiColor::new(vec![
                                                (UiBase::id(), Color::WHITE),
                                                (UiHover::id(), METRO_BLUE_COLOR)
                                            ]),
                                            Sprite{
                                                image: asset_server.load("button_symetric_sliced.png"),
                        // Here we enable sprite slicing
                                                image_mode: SpriteImageMode::Sliced(TextureSlicer { border: BorderRect::square(32.0), ..default() }),
                                                ..default() 
                                            },
                                            PickingBehavior::IGNORE,
                                        )).with_children(|ui|{
                                            ui.spawn((
                                                UiLayout::window().pos((Rh(40.0), Rl(50.0))).anchor(Anchor::CenterLeft).pack(),
                                                UiColor::new(vec![
                                                    (UiBase::id(), Color::BLACK),
                                                    (UiHover::id(), Color::WHITE),
                                                ]),
                                                UiHover::new().forward_speed(20.0).backward_speed(4.0),
                                                UiTextSize::from(Rh(60.0)), 
                                                Text2d::new(button),
                                                //text animator, maybe need to steal some crates)0000000000000
                                                TextFont{
                                                    font:asset_server.load(UI_MENU_FONT),
                                                    font_size: 64.,
                                                    ..default()
                                                },
                                                PickingBehavior::IGNORE
                                            ));
                                            //add smth here if you want add something to text on button
                                        });

                                    }).observe(hover_set::<Pointer<Over>, true>).observe(hover_set::<Pointer<Out>,false>);
                                    
                                    match button {
                                        "Новая игра" => {
                                            button_entity.observe(|_:Trigger<Pointer<Click>>,mut swap_state: EventWriter<SwapStatesEvent>,mut change_music: EventWriter<ChangeTrackEvent>|{
                                                swap_state.send( SwapStatesEvent { move_to_where: MainMenuStates::NewGame });                                                       
                                                change_music.send(ChangeTrackEvent{track: None});
                                            });
                                        }
                                        "Обучение" => {
                                            button_entity.observe(|_:Trigger<Pointer<Click>>,mut swap_state: EventWriter<SwapStatesEvent>,mut change_music: EventWriter<ChangeTrackEvent>|{
                                                swap_state.send( SwapStatesEvent { move_to_where: MainMenuStates::Tutorial });       
                                                change_music.send(ChangeTrackEvent{track: None});
                                            });
                                        }
                                        "Настройки" => {
                                            button_entity.observe(|_:Trigger<Pointer<Click>>,mut swap_state: EventWriter<SwapStatesEvent>|{
                                                swap_state.send( SwapStatesEvent { move_to_where: MainMenuStates::Settings });       
                                            });
                                        }
                                        "Выйти" => {
                                            button_entity.observe(|_:Trigger<Pointer<Click>>, mut quit: EventWriter<AppExit>|{
                                                quit.send(AppExit::Success);
                                            });
                                        }
                                        _ => {
                                            panic!("Error option");
                                        }
                                    }

                                    offset += BUTTON_GAP + BUTTON_SIZE;

                                }
                            });
                    });
            });
    }
}

fn changing_states_handler(mut swap_state_ev: EventReader<SwapStatesEvent>,mut spawn_tutorial_ev: EventWriter<TutorialSpawnEvent>,mut state_manager: ResMut<NextState<GameState>>){
    for ev in swap_state_ev.read(){
        println!("What?");
        match ev.move_to_where {
            MainMenuStates::NewGame =>{
                state_manager.set(GameState::InGame);
                println!("A?");
            }
            MainMenuStates::Tutorial =>{
                spawn_tutorial_ev.send(TutorialSpawnEvent);
                state_manager.set(GameState::InGame);
                println!("B?");
            }
            MainMenuStates::Settings =>{
                state_manager.set(GameState::Settings);
            }
            _ =>{
                panic!("NO SUCH STATE TO TRANSFER");
            }
        }
    }
}