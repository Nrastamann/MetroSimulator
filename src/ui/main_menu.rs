use crate::GameState;
use bevy::prelude::*;
use bevy_lunex::*;
pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SwapStatesEvent>().add_systems(OnEnter(GameState::MainMenu), (MainMenuScene::spawn,changing_states_handler)).add_systems(OnExit(GameState::MainMenu), despawn_scene_with::<MainMenuScene>);
    }
}

fn despawn_scene_with<S: Component>(mut commands: Commands, query: Query<Entity, With<S>>){
    for entity in &query{
        commands.entity(entity).despawn_recursive();
    }
}
pub const METRO_BLUE_COLOR: Color = Color::srgb(0.09, 0.337, 0.635);
pub const METRO_LIGHT_BLUE_COLOR: Color = Color::srgb(0.09, 0.337, 0.57);
pub const BUTTON_SIZE: f32 = 14.0;
pub const BUTTON_GAP: f32 = 11.0;
pub const MAIN_MENU_BUTTONS: [&str; 4] = ["Новая игра","Обучение","Настройки", "Выйти"];

pub enum MainMenuStates {
    Tutorial = 0,
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
                                        font:asset_server.load("fonts/Metro.ttf"),
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
                                                image: asset_server.load("button.png"),
                                                image_mode: SpriteImageMode::Sliced(TextureSlicer { border: BorderRect::square(32.0), ..default()}),
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
                                                    font:asset_server.load("fonts/ofont.ru_FreeSet.ttf"),
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
                                            button_entity.observe(|_:Trigger<Pointer<Click>>,mut next: ResMut<NextState<GameState>>|{
                                                next.set(GameState::InGame);
                                            });
                                        }
                                        "Обучение" => {
                                            button_entity.observe(|_:Trigger<Pointer<Click>>,mut tutorial_state: EventWriter<SwapStatesEvent>|{
                                                tutorial_state.send( SwapStatesEvent { move_to_where: MainMenuStates::Tutorial });       
                                            });
                                        }
                                        "Настройки" => {
                                            button_entity.observe(|_:Trigger<Pointer<Click>>,mut tutorial_state: EventWriter<SwapStatesEvent>|{
                                                tutorial_state.send( SwapStatesEvent { move_to_where: MainMenuStates::Settings });       
                                            });
                                        }
                                        "Выйти" => {
                                            button_entity.observe(|_:Trigger<Pointer<Click>>, mut quit: EventWriter<AppExit>|{
                                                quit.send(AppExit::Success);
                                            });
                                        }
                                        "Placeholder1" => {}
                                        _ => {}
                                    }

                                    offset += BUTTON_GAP + BUTTON_SIZE;

                                }
                            });
                    });
            });
    }
}

fn changing_states_handler(mut swap_state_ev: EventReader<SwapStatesEvent>){
    for ev in swap_state_ev.read(){
        match ev.move_to_where {
            MainMenuStates::Tutorial =>{
                //do something
            }
            MainMenuStates::Settings =>{
                //do something
            }
            _ =>{
                panic!("NO SUCH STATE TO TRANSFER");
            }
        }
    }
}