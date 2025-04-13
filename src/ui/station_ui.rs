use bevy::prelude::*;
use bevy_lunex::*;
//ADD REDRAW EVENT HANDLER, ADD SUPPORT TO NOT RE-CHANGE ALL TEXTs
use crate::{
    camera::MainCamera, cursor::CursorPosition, station::{StartBuildingEvent, Station}, station_blueprint::Direction, ui::main_menu::METRO_BLUE_COLOR, GameState
};

pub const RMB_STATS: [&str; 3] = ["Поезда", "Люди на станции", "Прочность станции"];
pub const RMB_BUTTONS: [&str; 2] = ["Новая станция", "Новая линия"];

pub const POPUP_WIDTH: f32 = 464.;
pub const POPUP_HEIGHT: f32 = 192.;

pub const OFFSET_STATS: f32 = 20.;
pub const OFFSET_LINES: f32 = 20.;
pub const BORDER_WIDTH: f32 = 96.;
#[derive(Bundle)]
struct TextBundle {
    color: UiColor,
    text_size: UiTextSize,
    text: Text2d,
    text_font: TextFont,
    picking_beh: PickingBehavior,
}
impl TextBundle {
    fn default_text(color: Color, font: Handle<Font>, size: f32, text: String) -> Self {
        Self {
            color: UiColor::from(color),
            text_size: UiTextSize::from(Rh(size)),
            text: Text2d::new(text),
            text_font: TextFont {
                font: font,
                font_size: 96.,
                ..default()
            },
            picking_beh: PickingBehavior::IGNORE,
        }
    }
}
trait UIStyles {
    fn anchor_center(self) -> Self;
    fn rl_size(self, x: f32, y: f32) -> Self;
    fn rl_pos(self, x: f32, y: f32) -> Self;
    fn anchor_right(self) -> Self;
    fn anchor_left(self) -> Self;
}
impl UIStyles for UiLayoutTypeWindow {
    fn anchor_left(self) -> Self {
        self.anchor(Anchor::TopLeft).rl_pos(0., 0.)
    }
    fn anchor_right(self) -> Self {
        self.anchor(Anchor::TopRight).rl_pos(100., 50.)
    }
    fn anchor_center(self) -> Self {
        self.anchor(Anchor::Center).rl_pos(50., 50.)
    }
    fn rl_size(self, x: f32, y: f32) -> Self {
        self.size(Rl((x, y)))
    }
    fn rl_pos(self, x: f32, y: f32) -> Self {
        self.pos(Rl((x, y)))
    }
}
pub const UI_FONT: &str = "fonts/ofont.ru_FreeSet.ttf";

pub struct StationUIPlugin;

impl Plugin for StationUIPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<TextboxResource>()
            .add_event::<RedrawEvent>();
        app.add_systems(OnEnter(GameState::InGame), PopupMenu::draw_popup)
            .add_systems(
                Update,
                (redraw_menu, draw_menu).run_if(in_state(GameState::InGame)),
            );
    }
}

#[derive(Event)]
pub struct RedrawEvent {
    change_text: bool,
    mouse_pos: Option<Vec2>,
    station: Option<(i32, i32)>,
    name: Option<String>,
}
#[derive(Resource, Default)]
pub struct TextboxResource {
    entities: Vec<Entity>,
}
#[derive(Component)]
pub struct PopupMenu {
    pub station: (i32, i32),
}

impl PopupMenu {
    fn draw_popup(
        mut commands: Commands,
        camera_q: Query<&MainCamera>,
        asset_server: Res<AssetServer>,
        cursor_pos: Res<CursorPosition>,
        mut popup_textboxes: ResMut<TextboxResource>,
    ) {
        let camera = camera_q.get_single().unwrap();
        commands
            .spawn((
                Visibility::Hidden,
                UiLayoutRoot::new_2d(),
                Dimension::from((
                    POPUP_WIDTH * camera.target_zoom,
                    POPUP_HEIGHT * camera.target_zoom,
                )),
                Transform::from_xyz(
                    cursor_pos.0.x + POPUP_WIDTH / 2. * camera.target_zoom,
                    cursor_pos.0.y - POPUP_HEIGHT / 2. * camera.target_zoom,
                    1.,
                ),
                PopupMenu { station: (0, 0) },
            ))
            .with_children(|ui| {
                ui.spawn((
                    Name::new("Station Menu"),
                    UiLayout::window().rl_size(100., 100.).pack(),
                    Sprite::from_color(Color::BLACK, Vec2::new(1., 1.)),
                ))
                .with_children(|ui| {
                    ui.spawn((
                        Name::new("Name boundary"),
                        UiLayout::window()
                            .rl_size(
                                100. - BORDER_WIDTH / POPUP_WIDTH * 2.,
                                30. - BORDER_WIDTH / POPUP_HEIGHT,
                            )
                            .anchor_left() //could break smth
                            .y(Rl(BORDER_WIDTH / POPUP_HEIGHT))
                            .x(Rl(BORDER_WIDTH / POPUP_WIDTH))
                            .pack(),
                        Sprite::default(),
                        UiColor::from(METRO_BLUE_COLOR),
                    ))
                    .with_children(|ui| {
                        popup_textboxes.entities.push(
                            ui.spawn((
                                Name::new("Station name"),
                                UiLayout::window()
                                    //37.5
                                    .anchor_center()
                                    .pack(),
                                TextBundle::default_text(
                                    Color::WHITE,
                                    asset_server.load(UI_FONT),
                                    100.,
                                    "sample_text".to_string(),
                                ),
                            ))
                            .id(),
                        );
                    });

                    ui.spawn((
                        Name::new("Stats block"),
                        UiLayout::window()
                            .anchor_left()
                            .pos((
                                Rl(BORDER_WIDTH / POPUP_WIDTH),
                                Rl(30.) + Rl(BORDER_WIDTH / POPUP_HEIGHT),
                            ))
                            .size(Rl((
                                50. - BORDER_WIDTH / POPUP_WIDTH * 2.,
                                70. - 2. * BORDER_WIDTH / POPUP_HEIGHT,
                            )))
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
                                        .y(Rl(offset_stats))
                                        .size(Rl((100., 20.)))
                                        .pack(),
                                ))
                                .with_children(|ui| {
                                    ui.spawn((
                                        Name::new("Text"),
                                        UiLayout::window().anchor_right().y(Rl(10.)).pack(),
                                        TextBundle::default_text(
                                            Color::WHITE,
                                            asset_server.load(UI_FONT),
                                            80.,
                                            i.to_string(),
                                        ),
                                    ));
                                });
                                offset_stats += OFFSET_STATS;
                            }
                        });
                        ui.spawn((
                            Name::new("Values section"),
                            UiLayout::window()
                                .x(Rl(70.))
                                .size(Rl((30., 100.)))
                                .anchor(Anchor::TopLeft)
                                .pack(),
                        ))
                        .with_children(|ui| {
                            let mut offset_stats: f32 = 0.;
                            for i in RMB_STATS {
                                ui.spawn((
                                    Name::new(i),
                                    UiLayout::window()
                                        .y(Rl(offset_stats))
                                        .size(Rl((100., 20.)))
                                        .pack(),
                                ))
                                .with_children(|ui| {
                                    popup_textboxes.entities.push(
                                        ui.spawn((
                                            Name::new("Text"),
                                            UiLayout::window().anchor_center().pack(),
                                            TextBundle::default_text(
                                                Color::WHITE,
                                                asset_server.load(UI_FONT),
                                                80.,
                                                "42".to_string(),
                                            ),
                                        ))
                                        .id(),
                                    );
                                });
                                offset_stats += OFFSET_STATS;
                            }
                        });
                    });
                    ui.spawn((
                        Name::new("Lines block"),
                        UiLayout::window()
                            .anchor_left()
                            .pos(Rl((50., 30. + BORDER_WIDTH / POPUP_HEIGHT)))
                            .size(Rl((
                                50. - BORDER_WIDTH / POPUP_WIDTH,
                                70. - BORDER_WIDTH / POPUP_HEIGHT * 2.,
                            )))
                            .pack(),
                        Sprite::default(),
                        UiColor::from(METRO_BLUE_COLOR),
                    ))
                    .with_children(|ui| {
                        ui.spawn((
                            Name::new("Current lines block"),
                            UiLayout::window().size(Rl((100., 70.))).pack(),
                        ))
                        .with_children(|ui| {
                            popup_textboxes.entities.push(
                                ui.spawn((
                                    Name::new("Line Handler"),
                                    UiLayout::window().anchor_center().pack(),
                                    TextBundle::default_text(
                                        Color::WHITE,
                                        asset_server.load(UI_FONT),
                                        30.,
                                        "WORK IN PROGRESS".to_string(),
                                    ),
                                ))
                                .id(),
                            );
                        });
                        ui.spawn((
                            Name::new("Buttons section"),
                            UiLayout::window().y(Rl(70.)).size(Rl((100., 30.))).pack(),
                        ))
                        .with_children(|ui| {
                            let mut offset_buttons = 0.;
                            for i in RMB_BUTTONS {
                                let mut button_entity = ui.spawn((
                                    Name::new("Button handler"),
                                    UiLayout::window()
                                        .x(Rl(offset_buttons))
                                        .size(Rl((50., 100.)))
                                        .anchor(Anchor::TopLeft)
                                        .pack(),
                                ))
                                .with_children(|ui| {
                                    ui.spawn((
                                        Name::new("Button"),
                                        UiLayout::window().full().pack(),
                                        Sprite::default(),
                                        UiHover::new().forward_speed(20.0).backward_speed(4.0),
                                        UiColor::new(vec![
                                            (UiBase::id(), METRO_BLUE_COLOR),
                                            (UiHover::id(), Color::WHITE),
                                        ]),
                                    ))
                                    .with_children(|ui| {
                                        ui.spawn((
                                            Name::new(i),
                                            UiLayout::window().anchor_center().pack(),
                                            UiHover::new().forward_speed(20.0).backward_speed(4.0),
                                            UiColor::new(vec![
                                                (UiBase::id(), Color::WHITE),
                                                (UiHover::id(), METRO_BLUE_COLOR),
                                            ]),
                                            UiTextSize::from(Rh(50.)),
                                            Text2d::new(i),
                                            TextFont {
                                                font: asset_server.load(UI_FONT),
                                                font_size: 96.,
                                                ..default()
                                            },
                                            PickingBehavior::IGNORE,
                                        ));
                                    });
                                })
                                .observe(hover_set::<Pointer<Over>, true>)
                                .observe(hover_set::<Pointer<Out>, false>)
                                .observe(|_:Trigger<Pointer<Click>>, mut new_station: EventWriter<StartBuildingEvent>|{
                                    let mut line = 0;
                                    //pass there something like query
                                    new_station.send(StartBuildingEvent { connection: (0,0), direction: Direction::Forward, line_to_attach: 0 });
                                });
                                offset_buttons += 50.;
                            }
                        });
                    });
                });
            });
    }
}

fn draw_menu(
    stations: Query<&Station>,
    mut draw_popup: EventWriter<RedrawEvent>,
    mouse: Res<ButtonInput<MouseButton>>,
    cursor_pos: Res<CursorPosition>,
    mut popup_q: Query<(&mut Visibility, &PopupMenu), With<UiLayoutRoot>>,
) {
    if mouse.just_pressed(MouseButton::Right) || mouse.just_pressed(MouseButton::Left) {
        // начинаем строить, определяем, будет это продолжение старой ветки или создание новой

        let Ok(mut popup_visibility) = popup_q.get_single_mut() else {
            panic!("Error: Popup is not founded");
        };

        let mut visible = Visibility::Hidden;
        for station in stations.iter() {
            if station.selected {
                let mut redraw = false;
                if station.id != popup_visibility.1.station {
                    redraw = true;
                }
                draw_popup.send(RedrawEvent {
                    change_text: redraw,
                    mouse_pos: Some(cursor_pos.0),
                    station: Some(station.id),
                    name: Some(station.name.clone()),
                });
                visible = Visibility::Visible;
                break;
            }
        }
        *popup_visibility.0 = visible;
    }
}
fn redraw_menu(
    mut redraw_popup: EventReader<RedrawEvent>,
    mut text_query: Query<&mut Text2d, Without<UiLayoutRoot>>,
    mut root: Query<&mut Transform, (With<PopupMenu>, With<UiLayoutRoot>)>,
    text_references: Res<TextboxResource>,
    cursor_pos: Res<CursorPosition>,

    camera_q: Query<&MainCamera>,
) {
    for ev in redraw_popup.read() {
        if ev.change_text {
            for i in text_references.entities.clone() {
                let mut text = text_query.get_mut(i).unwrap();
                text.0 = "sosal".to_string();
            }
        }
        let mut position = root.get_single_mut().unwrap();

        let camera = camera_q.get_single().unwrap();

        *position = Transform::from_xyz(
            cursor_pos.0.x + POPUP_WIDTH / 2. * camera.target_zoom,
            cursor_pos.0.y - POPUP_HEIGHT / 2. * camera.target_zoom,
            0.,
        );
    }
}
