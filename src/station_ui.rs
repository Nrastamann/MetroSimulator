use bevy::prelude::*;
use bevy_lunex::{Ab, Rl, Rw, UiFetchFromCamera, UiLayout, UiLayoutRoot};

use crate::{
    cursor::CursorPosition, loading_screen::METRO_BLUE_COLOR, metro::Metro, station::Station,
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
    mouse_pos: CursorPosition,
    station: Station,
}

#[derive(Component)]
pub struct PopupMenu;

impl PopupMenu {
    fn draw_popup(mut draw_info: EventReader<SpawnPopupEvent>, mut commands: Commands) {
        for ev in draw_info.read() {
            commands
                .spawn((UiLayoutRoot::new_2d(), UiFetchFromCamera::<0>, PopupMenu))
                .with_children(|ui| {
                    ui.spawn((
                        Name::new("Station Menu"),
                        UiLayout::window().pos((ev.station.position.x,ev.station.position.y)).size(Rl((10., 20.))).pack(),
                        Sprite::from_color(METRO_BLUE_COLOR, (1., 1.).into()),
                    ));
                });
        }
    }
}

fn draw_menu(
    stations: Query<&Station>,
    mut draw_popup: EventWriter<SpawnPopupEvent>,
    cursor_position: Res<CursorPosition>,
    mouse: Res<ButtonInput<MouseButton>>,
) {
    if mouse.just_pressed(MouseButton::Right) {
        // начинаем строить, определяем, будет это продолжение старой ветки или создание новой
        for station in stations.iter() {
            if station.selected {
                draw_popup.send(SpawnPopupEvent {
                    mouse_pos: cursor_position.clone(),
                    station: *station,
                });
            }
        }
    }
}
