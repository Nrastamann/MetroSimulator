use crate::GameState;
use bevy::prelude::*;
use bevy_lunex::*;
pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::MainMenu), MainMenuScene::spawn);
    }
}

const BUTTON_SIZE: f32 = 14.0;
const BUTTON_GAP: f32 = 3.0;
const MAIN_MENU_BUTTONS: [&str; 3] = ["Placeholder1", "Новая игра", "Выйти"];
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
                ui.spawn((UiLayout::solid().size((881., 1600.)).align_x(1.).pack(),))
                    .with_children(|ui| {
                        ui.spawn((
                            Name::new("Panel"),
                            UiLayout::window()
                                .x(Rl(50.))
                                .anchor(Anchor::TopCenter)
                                .size(Rl(105.))
                                .pack(),
                            Sprite::from_color(Color::hsv(0., 0., 100.), (1., 1.).into()),
                        ));

                        ui.spawn((UiLayout::window()
                            .y(Rl(11.0))
                            .size(Rl((105.0, 20.0)))
                            .pack(),))
                            .with_children(|ui| {
                                // Spawn the logo
                                ui.spawn((
                                    Name::new("Logo"),
                                    UiLayout::solid().size((1240.0, 381.0)).pack(),
                                    Sprite::from_image(asset_server.load("placeholder.png")),
                                ));
                            });

                        // Spawn button boundary
                        ui.spawn((UiLayout::window()
                            .pos(Rl((22.0, 33.0)))
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
                                                (UiBase::id(), Color::WHITE.with_alpha(4.5)),
                                                (UiHover::id(), Color::WHITE)
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
                                                    (UiHover::id(), Color::BLACK.with_alpha(4.5)),
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
                                        "Новая игра" => {}
                                        "Выйти" => {}
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
