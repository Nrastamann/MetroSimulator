use bevy::prelude::*;

use crate::{
    ui::main_menu::METRO_BLUE_COLOR, station::StationButton,
    GameState,
};

pub struct StationUIPlugin;

impl Plugin for StationUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnPopupEvent>();
        app.add_systems(Update, draw_menu.run_if(in_state(GameState::InGame)));
        app.add_systems(Update, PopupMenu::draw_popup);
    }
}
#[derive(Event)]
pub struct SpawnPopupEvent {
    mouse_pos: Vec2,
//    station: Station,
}

#[derive(Component)]
pub struct PopupMenu;

impl PopupMenu {
    fn draw_popup(
        mut draw_info: EventReader<SpawnPopupEvent>,
        mut commands: Commands,
        mut popup_query: Query<Entity, With<PopupMenu>>,
        query_window: Query<&Window>,
        asset_server: Res<AssetServer>,
    ) {
        let window = query_window.single();
        for ev in draw_info.read() {
            for entity in popup_query.iter_mut() {
                commands.entity(entity).despawn_recursive();
            }

            let rel_pos = (
                (ev.mouse_pos.x / window.width()).abs(),
                (ev.mouse_pos.y / window.height()).abs(),
            );

            commands
                .spawn((
                    Node {
                        position_type: PositionType::Absolute,
                        top: Val::Percent(rel_pos.1 * 100.),
                        left: Val::Percent(rel_pos.0 * 100.),

                        width: Val::Percent(35.0),
                        height: Val::Percent(20.0),
                        ..default()
                    },
                    BackgroundColor(Color::BLACK),
                    PopupMenu,
                ))
                .with_children(|ui| {
                    //NAME ROOT NODE
                    ui.spawn((
                        Node {
                            position_type: PositionType::Absolute,
                            top: Val::Percent(0.),
                            left: Val::Percent(0.),

                            width: Val::Percent(100.),
                            height: Val::Percent(30.),

                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,

                            ..default()
                        },
                        BackgroundColor(METRO_BLUE_COLOR),
                    )).with_children(|ui|{
                        ui.spawn((
                           Text::new(//ev.station.name
                            "STATION NAME"
                           ), 
                           TextFont{
                            font: asset_server.load("fonts/Metro.ttf"),
                            font_size: 18.,
                            ..default()
                           },
                           TextColor(Color::WHITE),
                        ));
                    });
                    
                    //BORDER
                    
                    ui.spawn((
                        Node{
                            position_type: PositionType::Absolute,
                            top: Val::Percent(30.),
                            left: Val::Percent(0.),

                            width: Val::Percent(100.),
                            height: Val::Percent(1.),

                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,

                            ..default()
                        },
                        BackgroundColor(Color::BLACK),
                    ));

                    //STATS ROOT NODE
                    
                    ui.spawn((
                        Node {
                            position_type: PositionType::Absolute,
                            top: Val::Percent(31.),
                            left: Val::Percent(0.),

                            width: Val::Percent(49.5),
                            height: Val::Percent(69.),

                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,

                            ..default()
                        },
                        BackgroundColor(METRO_BLUE_COLOR),
                    )).with_children(|ui|{
                        ui.spawn((
                           Text::new(//ev.station.name
                            "STATS SECTION"
                           ), 
                           TextFont{
                            font: asset_server.load("fonts/Metro.ttf"),
                            font_size: 18.,
                            ..default()
                           },
                           TextColor(Color::WHITE),
                        ));
                    });

                    //BORDER NODE

                    ui.spawn((
                        Node{
                            position_type: PositionType::Absolute,
                            top: Val::Percent(31.),
                            left: Val::Percent(49.5),

                            width: Val::Percent(1.),
                            height: Val::Percent(69.),

                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,

                            ..default()
                        },
                        BackgroundColor(Color::BLACK),
                    ));
                    
                    //BUTTON ROOT NODE

                    ui.spawn((
                        Node {
                            position_type: PositionType::Absolute,
                            top: Val::Percent(31.),
                            left: Val::Percent(50.5),

                            width: Val::Percent(49.5),
                            height: Val::Percent(69.),

                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,

                            ..default()
                        },
                        BackgroundColor(METRO_BLUE_COLOR),
                    )).with_children(|ui|{
                        ui.spawn((
                           Text::new(//ev.station.name
                            "LINES SECTION"
                           ), 
                           TextFont{
                            font: asset_server.load("fonts/Metro.ttf"),
                            font_size: 18.,
                            ..default()
                           },
                           TextColor(Color::WHITE),
                        ));
                    });
                });
        }
    }
}

fn draw_menu(
    stations: Query<&StationButton>,
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
//                        station: *station,
                    });
                } else {
                    panic!("Error: Cursor is not founded");
                }
            }
        }
    }
}
