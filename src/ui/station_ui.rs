use bevy::prelude::*;
use bevy_lunex::*;

use crate::{station::Station, ui::main_menu::METRO_BLUE_COLOR, GameState};

pub const RMB_STATS: [&str; 3] = ["Поезда", "Люди на станции", "Прочность станции"];
pub const OFFSET: f32 = 30.;
pub struct StationUIPlugin;

impl Plugin for StationUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnPopupEvent>();
        app.add_systems(
            Update,
            (draw_menu, PopupMenu::draw_popup).run_if(in_state(GameState::InGame)),
        );
    }
}

#[derive(Event)]
pub struct SpawnPopupEvent {
    mouse_pos: Vec2,
    station: Station,
}

#[derive(Component)]
pub struct PopupMenu {
    pub station: (i32, i32),
}

impl PopupMenu {
    fn draw_popup(
        mut draw_info: EventReader<SpawnPopupEvent>,
        mut commands: Commands,
        //        mut popup_query: Query<Entity, With<PopupMenu>>,
        query_window: Query<&Window>,
        asset_server: Res<AssetServer>,
    ) {
        let window = query_window.single();
        for ev in draw_info.read() {
            let rel_pos = (
                (ev.mouse_pos.x / window.width()).abs(),
                (ev.mouse_pos.y / window.height()).abs(),
            );
            commands
                .spawn((
                    UiLayoutRoot::new_2d(),
                    UiFetchFromCamera::<0>,
                    PopupMenu {
                        station: ev.station.id,
                    },
                ))
                .with_children(|ui| {
                    ui.spawn((
                        Name::new("Station Menu"),
                        UiLayout::window()
                            .pos(Rl((rel_pos.0 * 100., rel_pos.1 * 100.)))
                            .size(Rl((40., 30.)))
                            .pack(),
                        Sprite::from_color(Color::BLACK, Vec2::new(1., 1.)),
                    ))
                    .with_children(|ui| {
                        ui.spawn((
                            Name::new("Name boundary"),
                            UiLayout::window()
                                .size(Rl((100., 30.)))
                                .pos(Rl((50.0, 15.0)))
                                .anchor(Anchor::Center)
                                .pack(),
                            Sprite::default(),
                            UiColor::from(METRO_BLUE_COLOR),
                        ))
                        .with_children(|ui| {
                            ui.spawn((
                                Name::new("Station name"),
                                UiLayout::window()
                                    //37.5
                                    .pos((Rl(50.0), Rh(50.0)))
                                    .anchor(Anchor::Center)
                                    .pack(),
                                UiColor::from(Color::WHITE),
                                UiTextSize::from(Rh(100.)),
                                Text2d::new("STATION NAME"),
                                TextFont {
                                    font: asset_server.load("fonts/ofont.ru_FreeSet.ttf"),
                                    font_size: 64.,
                                    ..default()
                                },
                                PickingBehavior::IGNORE,
                            ));
                        });

                        ui.spawn((
                            Name::new("Stats block"),
                            UiLayout::window()
                                .pos(Rl((0., 30.)))
                                .size(Rl((50., 70.)))
                                .pack(),
                            Sprite::default(),
                            UiColor::from(METRO_BLUE_COLOR),
                        ))
                        .with_children(|ui| {
                            ui.spawn((
                                Name::new("Name section"),
                                UiLayout::window().size(Rl((70., 100.))).pack(),
                            ))
                            .with_children(|ui| {
                                let mut offset_stats: f32 = 0.0;
                                for i in RMB_STATS {
                                    ui.spawn((
                                        Name::new(i),
                                        UiLayout::window()
                                            .y(offset_stats)
                                            .size(Rl((100., 20.)))
                                            .pack(),
                                    ))
                                    .with_children(|ui| {
                                        ui.spawn((
                                            Name::new("Text"),
                                            UiLayout::window()
                                                .pos((Rl(100.), Rl(50.)))
                                                .anchor(Anchor::CenterRight)
                                                .pack(),
                                            UiColor::from(Color::WHITE),
                                            UiTextSize::from(Rh(80.)),
                                            Text2d::new(i),
                                            TextFont {
                                                font: asset_server
                                                    .load("fonts/ofont.ru_FreeSet.ttf"),
                                                font_size: 64.,
                                                ..default()
                                            },
                                            PickingBehavior::IGNORE,
                                        ));
                                    });
                                    offset_stats += OFFSET;
                                }
                            });
                            ui.spawn((
                                Name::new("Values section"),
                                UiLayout::window().x(Rl(70.)).size(Rl((30., 100.))).pack(),
                            ))
                            .with_children(|ui| {
                                let mut offset_stats: f32 = 0.;
                                for i in RMB_STATS {
                                    ui.spawn((
                                        Name::new(i),
                                        UiLayout::window()
                                            .y(offset_stats)
                                            .size(Rl((100., 20.)))
                                            .pack(),
                                    ))
                                    .with_children(|ui| {
                                        ui.spawn((
                                            Name::new("Text"),
                                            UiLayout::window()
                                                .pos((Rl(50.), Rl(50.)))
                                                .anchor(Anchor::Center)
                                                .pack(),
                                            UiColor::from(Color::WHITE),
                                            UiTextSize::from(Rh(80.)),
                                            Text2d::new("42"),
                                            TextFont {
                                                font: asset_server
                                                    .load("fonts/ofont.ru_FreeSet.ttf"),
                                                font_size: 64.,
                                                ..default()
                                            },
                                            PickingBehavior::IGNORE,
                                        ));
                                    });
                                    offset_stats += OFFSET;
                                }
                            });
                        });
                        ui.spawn((
                            Name::new("Lines block"),
                            UiLayout::window()
                                .pos(Rl((50., 30.)))
                                .size(Rl((50., 70.)))
                                .pack(),
                            Sprite::default(),
                            UiColor::from(METRO_BLUE_COLOR),
                        ))
                        .with_children(|ui| {
                            ui.spawn((
                                Name::new("Current lines block"),
                                UiLayout::window().size(Rl((100., 70.))).pack(),
                            ));
                        });
                    });
                });
        }
    }
}

fn draw_menu(
    stations: Query<&Station>,
    mut draw_popup: EventWriter<SpawnPopupEvent>,
    mouse: Res<ButtonInput<MouseButton>>,
    query_window: Query<&Window>,
) {
    if mouse.just_pressed(MouseButton::Right) {
        // начинаем строить, определяем, будет это продолжение старой ветки или создание новой
        for station in stations.iter() {
            if station.selected {
                let window = query_window.single();
                if let Some(cursor_pos) = window.cursor_position() {
                    draw_popup.send(SpawnPopupEvent {
                        mouse_pos: cursor_pos.clone(),
                        station: station.clone(),
                    });
                } else {
                    panic!("Error: Cursor is not founded");
                }
            }
        }
    }
}
