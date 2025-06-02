//redraw lines and text into mb different events?
use bevy::prelude::*;
use bevy_lunex::*;
//ADD REDRAW EVENT HANDLER, ADD SUPPORT TO NOT RE-CHANGE ALL TEXTs
use crate::{
    camera::MainCamera, cursor::CursorPosition, line::MetroLine, metro::{Direction, Metro}, money::{Money, TRAIN_COST}, station::{StartBuildingEvent, Station, StationButton}, station_blueprint::SetBlueprintColorEvent, train::SpawnTrainEvent, ui::MoneyRedrawEvent, GameState
};

use super::{BuyTrainTutorial, METRO_LIGHT_BLUE_COLOR, UI_FONT};

pub const RMB_STATS: [&str; 2] = ["Поезда", "Люди на станции"];
pub const RMB_BUTTONS: [&str; 2] = ["Новая станция", "Новая линия"];

pub const POPUP_WIDTH: f32 = 464.;
pub const POPUP_HEIGHT: f32 = 192.;

pub const OFFSET_STATS: f32 = 20.;
pub const OFFSET_LINES: f32 = 20.;
pub const LINES_SIZE: f32 = 20.; //shouldn't be greater than offset
pub const BORDER_WIDTH: f32 = 96.;

const POPUP_NAME: usize = 0;
const POPUP_TRAINS_AMOUNT: usize = 1;
const POPUP_AMOUNT_OF_PEOPLE: usize = 2;
const POPUP_LINE_HANDLER: usize = 3;
const POPUP_STATION_BUTTON: usize = 8;

pub const OPACITY_LEVEL_MAIN: f32 = 0.8;
pub const OPACITY_LEVEL_BLUR: f32 = 0.6;
pub const OPACITY_LEVEL_HIGHEST: f32 = 0.2;

pub trait UIStyles {
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

pub struct StationUIPlugin;

impl Plugin for StationUIPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<TextboxResource>()
            .init_resource::<LinesResource>()
            .add_event::<RedrawEvent>()
            .add_event::<ChangeLinesVisibility>()
            .add_event::<RedrawPickedLineEvent>();
        app.add_systems(OnEnter(GameState::InGame), PopupMenu::draw_popup)
            .add_systems(
                Update,
                (
                    redraw_menu,
                    draw_menu,
                    redraw_lines_menu,
                    change_visibility_of_lines,
                )
                    .run_if(in_state(GameState::InGame)),
            );
    }
}
#[derive(Event)]
pub struct RedrawPickedLineEvent {
    picked_line_now: usize,
}
#[derive(Component)]
struct NewStationFlag {
    can_continue: bool,
}
#[derive(Component)]
struct LineHandlerFlag {
    line_id: usize,
}
#[derive(Event)]
pub struct ChangeLinesVisibility;
#[allow(unused)]
#[derive(Event)]
pub struct RedrawEvent {
    change_text: bool,
    station: Option<(i32, i32)>,
}
#[derive(Resource, Default)]
pub struct TextboxResource {
    entities: Vec<Entity>, //
}

#[derive(Component)]
pub struct PopupMenu {
    pub station: (i32, i32),
    pub picked_line: usize,
}
#[derive(Resource, Default)]
pub struct LinesResource {
    pub entities: Vec<Entity>,
}
impl PopupMenu {
    fn draw_popup(
        mut commands: Commands,
        camera_q: Query<&MainCamera>,
        asset_server: Res<AssetServer>,
        cursor_pos: Res<CursorPosition>,
        mut popup_textboxes: ResMut<TextboxResource>,
        mut popup_lines: ResMut<LinesResource>,
    ) {
        let camera = camera_q.get_single().unwrap();
        commands
            .spawn((
                StateScoped(GameState::InGame),
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
                PopupMenu {
                    station: (0, 0),
                    picked_line: 0,
                },
            ))
            .with_children(|ui| {
                ui.spawn((
                    Name::new("Station Menu"),
                    UiLayout::window().rl_size(100., 100.).pack(),
                    Sprite{image: asset_server.load(
                        "button_symetric_sliced.png",
                    ),
                    color: METRO_LIGHT_BLUE_COLOR,//.with_alpha(OPACITY_LEVEL_HIGHEST),
                    image_mode: SpriteImageMode::Sliced(TextureSlicer { border: BorderRect::square(32.0), ..default() }), 
                    ..default()},
                ))
                .with_children(|ui| {
                    ui.spawn((
                        Name::new("Name boundary"),
                        UiLayout::window()
                            .rl_size(
                                100. - BORDER_WIDTH / POPUP_WIDTH * 2.,
                                20. - BORDER_WIDTH / POPUP_HEIGHT,
                            )
                            .anchor_left() //could break smth
                            .y(Rl(BORDER_WIDTH / POPUP_HEIGHT))
                            .x(Rl(BORDER_WIDTH / POPUP_WIDTH))
                            .pack(),
                    ))
                    .with_children(|ui| {
                        popup_textboxes.entities.push(
                            ui.spawn((
                                Name::new("Station name"),
                                UiLayout::window()
                                    .x(Rl(100.))
                                    //37.5
                                    .anchor_center()
                                    .pack(),
//                                UiColor::from(Color::WHITE.with_alpha(0.8)),
                                UiTextSize::from(Rh(100.)),
                                Text2d::new("sample_txt"),
                                TextFont {
                                    font: asset_server.load(UI_FONT),
                                    font_size: 96.,
                                    ..default()
                                },
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
                                Rl(20.) + Rl(BORDER_WIDTH / POPUP_HEIGHT),
                            ))
                            .size(Rl((
                                50. - BORDER_WIDTH / POPUP_WIDTH * 2.,
                                80. - 2. * BORDER_WIDTH / POPUP_HEIGHT,
                            )))
                            .pack(),
//                        Sprite::default(),
//                        UiColor::from(METRO_LIGHT_BLUE_COLOR.with_alpha(OPACITY_LEVEL_HIGHEST)),
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
                                        UiColor::from(Color::WHITE.with_alpha(0.8)),
                                        UiTextSize::from(Rh(70.)),
                                        Text2d::new(i.to_string()),
                                        TextFont {
                                            font: asset_server.load(UI_FONT),
                                            font_size: 96.,
                                            ..default()
                                        },
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
                                            UiColor::from(Color::WHITE.with_alpha(0.8)),
                                            UiTextSize::from(Rh(80.)),
                                            Text2d::new("42"),
                                            TextFont {
                                                font: asset_server.load(UI_FONT),
                                                font_size: 96.,
                                                ..default()
                                            },
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
                            .pos(Rl((50., 20. + BORDER_WIDTH / POPUP_HEIGHT)))
                            .size(Rl((
                                50. - BORDER_WIDTH / POPUP_WIDTH,
                                80. - BORDER_WIDTH / POPUP_HEIGHT * 2.,
                            )))
                            .pack(),
//                        Sprite::default(),
//                        UiColor::from(METRO_LIGHT_BLUE_COLOR.with_alpha(OPACITY_LEVEL_HIGHEST)),
                    ))
                    .with_children(|ui| {
                        ui.spawn((
                            Name::new("Current lines block"),
                            UiLayout::window().size(Rl((100., 70.))).pack(),
//                            UiColor::from(METRO_LIGHT_BLUE_COLOR.with_alpha(OPACITY_LEVEL_HIGHEST)),
//                            Sprite::default(),
                        ))
                        .with_children(|ui| {
                            let mut height_off = 0.;
                            for i in 0..5 {
                                popup_lines.entities.push(
                                    ui.spawn((
                                        Name::new("Line Handler "),
                                        UiLayout::window()
                                            .anchor_left()
                                            .rl_size(100., LINES_SIZE)
                                            .rl_pos(0., height_off)
                                            .pack(),
                                        LineHandlerFlag { line_id: i },
//                                        Sprite::default(),
//                                        UiColor::from(METRO_LIGHT_BLUE_COLOR.with_alpha(OPACITY_LEVEL_HIGHEST)),
                                        Visibility::Hidden,
                                    ))
                                    .with_children(|ui| {
                                        let text = format!("Линия {}", i + 1);
                                        popup_textboxes.entities.push(
                                            ui.spawn((
                                                Name::new("line name"),
                                                UiLayout::window().anchor_center().pack(),
                                                UiColor::from(Color::WHITE.with_alpha(OPACITY_LEVEL_MAIN)),
                                                UiTextSize::from(Rh(100.)),
                                                Text2d::new(text),
                                                TextFont {
                                                    font: asset_server.load(UI_FONT),
                                                    font_size: 96.,
                                                    ..default()
                                                },
                                                Visibility::Inherited,
                                                PickingBehavior::IGNORE,
                                            ))
                                            .id(),
                                        );
                                    })
                                    .observe(
                                        |clck: Trigger<Pointer<Click>>,
                                         mut lines_handler_q: Query<&mut LineHandlerFlag>,
                                         mut ui_root_q: Query<
                                            &mut PopupMenu,
                                            With<UiLayoutRoot>,
                                        >,
                                         mut redraw_lines_ev: EventWriter<
                                            RedrawPickedLineEvent,
                                        >| {
                                            let mut root = ui_root_q.get_single_mut().unwrap();
                                            if !lines_handler_q.get_mut(clck.target).is_ok(){
                                                return;
                                            }
                                            root.picked_line = lines_handler_q
                                                .get_mut(clck.target).unwrap()//???? crashes there sometimes
                                                .line_id;
                                            redraw_lines_ev.send(RedrawPickedLineEvent {
                                                picked_line_now: root.picked_line,
                                            });
                                        },
                                    )
                                    .id(),
                                );
                                height_off += OFFSET_LINES;
                            }
                        });
                        ui.spawn((
                            Name::new("Buttons section"),
                            UiLayout::window().y(Rl(70.)).size(Rl((100., 30.))).pack(),
                        ))
                        .with_children(|ui| {
                            ui.spawn((
                                Name::new("Buy Train"),
                                UiLayout::window().rl_size(100., 35.).rl_pos(0., 0.).pack(),
                            ))
                            .with_children(|ui| {
                                ui.spawn((
                                    Name::new("New train handler"),
                                    UiLayout::window()
                                        .rl_pos(0., 0.)
                                        .size(Rl((100., 100.)))
                                        .pack(),
//                                    Sprite::default(),
                                    UiHover::new().forward_speed(20.0).backward_speed(4.0),
//                                    UiColor::from(Color::WHITE.with_alpha(OPACITY_LEVEL_HIGHEST)),
/*                                    UiColor::new(vec![
                                        (UiBase::id(), Color::WHITE.with_alpha(0.2)),
                                        (UiHover::id(), METRO_LIGHT_BLUE_COLOR.with_alpha(0.2)),
                                    ])*/
                                ))
                                .with_children(|ui| {
                                    ui.spawn((
                                        Name::new("Buy Train"),
                                        UiLayout::window().anchor_center().pack(),
                                        UiHover::new().forward_speed(20.0).backward_speed(4.0),
                                        UiColor::new(vec![
                                            (UiBase::id(), Color::WHITE),
                                            (UiHover::id(), METRO_LIGHT_BLUE_COLOR),
                                        ]),
                                        UiTextSize::from(Rh(100.)),
                                        Text2d::new("Купить поезд"),
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
                            .observe(|_: Trigger<Pointer<Click>>,mut money: ResMut<Money>, metro: Res<Metro>,mut change_money_ui: EventWriter<MoneyRedrawEvent>, mut buy_train: EventWriter<SpawnTrainEvent>, mut buy_train_t: EventWriter<BuyTrainTutorial>,popup_q: Query<&PopupMenu, With<UiLayoutRoot>>| {
                                let popup = popup_q.get_single().unwrap();
                                buy_train.send(SpawnTrainEvent{
                                    line: popup.picked_line,
                                    station: popup.station,
                                });

                                money.0 -= TRAIN_COST;
                                change_money_ui.send(MoneyRedrawEvent);
                        
                                println!("BOUGHT A TRAIN");
                                buy_train_t.send(BuyTrainTutorial);
                            });

                            let mut offset_buttons = 0.;
                            for i in RMB_BUTTONS {
                                let mut button_entity = ui.spawn((
                                    Name::new("Button handler"),
                                    UiLayout::window()
                                        .y(Rl(35.))
                                        .x(Rl(offset_buttons))
                                        .size(Rl((50., 65.)))
                                        .anchor(Anchor::TopLeft)
                                        .pack(),
                                ));
                                button_entity
                                    .with_children(|ui| {
                                        ui.spawn((
                                            Name::new("Button"),
                                            UiLayout::window().full().pack(),
//                                            Sprite::default(),
                                            UiHover::new().forward_speed(20.0).backward_speed(4.0),
//                                            UiColor::from(Color::WHITE.with_alpha(OPACITY_LEVEL_HIGHEST))
/*                                            UiColor::new(vec![
                                                (UiBase::id(), METRO_LIGHT_BLUE_COLOR.with_alpha(0.0)),
                                                (UiHover::id(), Color::WHITE.with_alpha(OPACITY_LEVEL_HIGHEST)),
                                            ]),
*/                                        ))
                                        .with_children(
                                            |ui| {
                                                popup_textboxes.entities.push(
                                                    ui.spawn((
                                                        Name::new(i),
                                                        UiLayout::window().anchor_center().pack(),
                                                        UiHover::new()
                                                            .forward_speed(20.0)
                                                            .backward_speed(4.0),
                                                        UiColor::new(vec![
                                                            (UiBase::id(), Color::WHITE),
                                                            (UiHover::id(), METRO_LIGHT_BLUE_COLOR),
                                                        ]),
                                                        UiTextSize::from(Rh(65.)),
                                                        Text2d::new(i),
                                                        TextFont {
                                                            font: asset_server.load(UI_FONT),
                                                            font_size: 96.,
                                                            ..default()
                                                        },
                                                        PickingBehavior::IGNORE,
                                                    ))
                                                    .id(),
                                                );
                                            },
                                        );
                                    })
                                    .observe(hover_set::<Pointer<Over>, true>)
                                    .observe(hover_set::<Pointer<Out>, false>);

                                match i {
                                    "Новая линия" => {
                                        button_entity.observe(
                                            |_: Trigger<Pointer<Click>>,
                                             mut new_station: EventWriter<StartBuildingEvent>,
                                             mut ui_root_q: Query<
                                                (&mut Visibility, &PopupMenu),
                                                With<UiLayoutRoot>,
                                            >,

                                             mut ev_set_blueprint: EventWriter<
                                                SetBlueprintColorEvent,
                                            >,
                                             mut ev_change_vision: EventWriter<
                                                ChangeLinesVisibility,
                                            >| {
                                                let (mut vision, position) =
                                                    ui_root_q.get_single_mut().unwrap();
                                                new_station.send(StartBuildingEvent {
                                                    connection: position.station,
                                                    direction: Direction::Forwards,
                                                    line_to_attach: usize::MAX,
                                                    from_menu: true,
                                                });
                                                *vision = Visibility::Hidden;
                                                ev_change_vision.send(ChangeLinesVisibility);
                                                ev_set_blueprint.send(SetBlueprintColorEvent(
                                                    Color::BLACK.with_alpha(0.5),
                                                ));
                                            },
                                        );
                                    }
                                    "Новая станция" => {
                                        button_entity
                                            .insert(NewStationFlag { can_continue: true })
                                            .observe(
                                                |_: Trigger<Pointer<Click>>,
                                                 mut new_station: EventWriter<
                                                    StartBuildingEvent,
                                                >,

                                                 mut ev_set_blueprint: EventWriter<
                                                    SetBlueprintColorEvent,
                                                >,
                                                 button_q: Query<&NewStationFlag>,
                                                 mut ui_root_q: Query<
                                                    (&mut Visibility, &PopupMenu),
                                                    With<UiLayoutRoot>,
                                                >,
                                                 mut metro: ResMut<Metro>,
                                                 mut ev_change_vision: EventWriter<
                                                    ChangeLinesVisibility,
                                                >| {
                                                    let (mut vision, position) =
                                                        ui_root_q.get_single_mut().unwrap();
                                                    let button_flag =
                                                        button_q.get_single().unwrap();
                                                    if !button_flag.can_continue {
                                                        return;
                                                    }
                                                    let line = metro
                                                        .lines
                                                        .iter()
                                                        .filter(|line| {
                                                            line.id == position.picked_line
                                                        })
                                                        .next()
                                                        .unwrap()
                                                        .clone();

                                                    let mut direction = Direction::Forwards;

                                                    if line.stations.front()
                                                        == metro.find_station(position.station)
                                                    {
                                                        direction = Direction::Backwards;
                                                    }

                                                    new_station.send(StartBuildingEvent {
                                                        connection: position.station,
                                                        direction: direction,
                                                        line_to_attach: position.picked_line,
                                                        from_menu: true,
                                                    });
                                                    *vision = Visibility::Hidden;
                                                    ev_change_vision.send(ChangeLinesVisibility);
                                                    ev_set_blueprint.send(SetBlueprintColorEvent(
                                                        Color::BLACK.with_alpha(0.5),
                                                    ));
                                                },
                                            );
                                    }
                                    _ => {
                                        println!("{i}");
                                        panic!("NONAME BUTTON");
                                    }
                                }
                                offset_buttons += 50.;
                            }
                        });
                    });
                });
            });
    }
}

fn redraw_lines_menu(
    mut redraw_linev_ev: EventReader<RedrawPickedLineEvent>,
    mut button_q: Query<
        (&mut NewStationFlag, &Children),
        (Without<LineHandlerFlag>, Without<Text2d>),
    >,
    mut text_query: Query<&mut UiColor, (With<UiLayout>, Without<LineHandlerFlag>, With<Text2d>)>,
    mut root: Query<&mut PopupMenu, (With<UiLayoutRoot>, Without<LineHandlerFlag>)>,
    mut metro: ResMut<Metro>,
    mut line_handlers_q: Query<
        (&mut LineHandlerFlag, &mut Children),
        Without<Text2d>,
    >,
    text_references: Res<TextboxResource>,
) {
    for ev in redraw_linev_ev.read() {
        let Ok(menu) = root.get_single_mut() else {
            panic!("Error: Popup is not founded");
        };
        for (_previous_handler, child_prev) in line_handlers_q.iter_mut() {
//            *color = UiColor::from(METRO_LIGHT_BLUE_COLOR.with_alpha(OPACITY_LEVEL_HIGHEST));

            *text_query
                .get_mut(*child_prev.iter().next().unwrap())
                .unwrap() = UiColor::from(Color::WHITE);
        }
        /*
                let (mut color, _previous_handler, child_prev) = line_handlers_q
                    .iter_mut()
                    .filter(|(_, line_numb, _)| line_numb.line_id == ev.picked_line_prev)
                    .next().unwrap();
                *color = UiColor::from(METRO_LIGHT_BLUE_COLOR);

                *text_query
                    .get_mut(*child_prev.iter().next().unwrap())
                    .unwrap() = UiColor::from(Color::WHITE);
        */
        let (_new_handler, child_now) = line_handlers_q
            .iter_mut()
            .filter(|(line_numb, _)| line_numb.line_id == ev.picked_line_now)
            .next()
            .unwrap();
        let mut bg_color: Hsla;
        bg_color = metro.lines[menu.picked_line].color.into();

        if bg_color.hue() < 180. {
            bg_color.hue += 180.;
        } else {
            bg_color.hue -= 180.;
        }

        // *color_new = UiColor::from(Color::srgba(0.2,0.2,0.2,0.6));

        *text_query
            .get_mut(*child_now.iter().next().unwrap())
            .unwrap() = UiColor::from(Color::BLACK);

        let (mut button_press, button_child) = button_q.get_single_mut().unwrap();

        *text_query
            .get_mut(text_references.entities[POPUP_STATION_BUTTON])
            .unwrap() = UiColor::new(vec![
            (UiBase::id(), Color::WHITE.with_alpha(OPACITY_LEVEL_MAIN)),
            (UiHover::id(), METRO_LIGHT_BLUE_COLOR.with_alpha(OPACITY_LEVEL_MAIN)),
        ]);

        button_press.can_continue = true;

        let line = metro
            .lines
            .iter()
            .filter(|line| line.id == ev.picked_line_now)
            .next()
            .unwrap()
            .clone();

        let popup_station = metro.find_station(menu.station).unwrap().clone();

        if !(*line.stations.front().unwrap() == popup_station)
            && !(*line.stations.back().unwrap() == popup_station)
        {
            *text_query
                .get_mut(text_references.entities[POPUP_STATION_BUTTON])
                .unwrap() = UiColor::new(vec![
                (UiBase::id(), Color::BLACK),
                (UiHover::id(), Color::BLACK),
            ]);
            println!("{}", text_references.entities.len(),);
            button_press.can_continue = false;
        }
    }
}

fn draw_menu(
    q_station: Query<(&Station, &StationButton)>,
    mut draw_popup: EventWriter<RedrawEvent>,
    mouse: Res<ButtonInput<MouseButton>>,
    cursor_pos: Res<CursorPosition>,
    mut popup_q: Query<(&mut Visibility, &PopupMenu, &Dimension, &Transform), With<UiLayoutRoot>>,
    mut ev_change_vision: EventWriter<ChangeLinesVisibility>,
) {
    let check = mouse.just_pressed(MouseButton::Left);
    if mouse.just_pressed(MouseButton::Right) || check {
        let Ok((mut popup_visibility, menu, size, pos)) = popup_q.get_single_mut() else {
            panic!("Error: Popup is not founded");
        };

        let Some((selected_station, _)) = q_station.iter().filter(|(_, btn)| btn.selected).next()
        else {
            if check {
                if cursor_pos.0.x > pos.translation.x + (size.x / 2.).floor()
                    || cursor_pos.0.y > pos.translation.y + (size.y / 2.).floor()
                    || cursor_pos.0.x < pos.translation.x - (size.x / 2.).floor()
                    || cursor_pos.0.y < pos.translation.y - (size.y / 2.).floor()
                {
                    *popup_visibility = Visibility::Hidden;

                    ev_change_vision.send(ChangeLinesVisibility);
                }
                return;
            }

            ev_change_vision.send(ChangeLinesVisibility);
            *popup_visibility = Visibility::Hidden;
            return;
        };
        let mut redraw = false;
        if selected_station.position != menu.station {
            redraw = true;
        }
        if !check {
            draw_popup.send(RedrawEvent {
                change_text: redraw,
                station: Some(selected_station.position),
            });
        }
    }
}
fn redraw_menu(
    mut redraw_popup: EventReader<RedrawEvent>,
    mut text_query: Query<&mut Text2d, (With<UiLayout>, Without<LineHandlerFlag>)>,
    mut root: Query<
        (&mut Transform, &mut Visibility, &mut PopupMenu),
        (With<UiLayoutRoot>, Without<LineHandlerFlag>),
    >,
    text_references: Res<TextboxResource>,
    cursor_pos: Res<CursorPosition>,
    camera_q: Query<&MainCamera>,
    mut metro: ResMut<Metro>,
    mut line_handlers_q: Query<(&mut Visibility, &mut LineHandlerFlag), Without<Text2d>>,
    line_handler_resource: Res<LinesResource>,
    mut redraw_linev_ev: EventWriter<RedrawPickedLineEvent>,
    station_q: Query<(&Station, &StationButton)>,
) {
    for ev in redraw_popup.read() {
        let (mut position, mut popup_visibility, mut popup_station) =
            root.get_single_mut().unwrap();

        //===================================START OF LINES VISUALISATION====================================================================
        // setup lines, where
        let mut lines_vec: Vec<MetroLine> = vec![];

        let station = metro.find_station(ev.station.unwrap()).unwrap().clone();

        for line in metro.lines.iter() {
            if line.stations.contains(&station) {
                lines_vec.push(line.clone());
            }
        }

        redraw_linev_ev.send(RedrawPickedLineEvent {
            picked_line_now: lines_vec[0].id,
        });

        popup_station.picked_line = lines_vec[0].id;

        popup_station.station = ev.station.unwrap();
        let mut line_position = 0;
        for i in 0..lines_vec.len() {
            line_position += 1;
            let Ok((mut vision, mut line_number)) =
                line_handlers_q.get_mut(line_handler_resource.entities[i])
            else {
                panic!("NO VISIBILITY IN LINE HANDLER!");
            };
            line_number.line_id = lines_vec[i].id;
            *vision = Visibility::Visible;
        }
        for i in line_position..5 {
            let Ok((mut vision, mut line_number)) =
                line_handlers_q.get_mut(line_handler_resource.entities[i])
            else {
                panic!("NO VISIBILITY IN LINE HANDLER!");
            };
            *vision = Visibility::Hidden;
            line_number.line_id = usize::MAX;
        }

        //===================================END OF LINES VISUALISATION====================================================================
        //there i change text by iterating through entities
        let (_station, station_info) = station_q
            .iter()
            .filter(|(stn, _)| stn.position == popup_station.station)
            .next()
            .unwrap();

        text_query
            .get_mut(text_references.entities[POPUP_NAME])
            .unwrap()
            .0 = station_info.name.clone();

        text_query
            .get_mut(text_references.entities[POPUP_AMOUNT_OF_PEOPLE])
            .unwrap()
            .0 = station_info.passenger_ids.len().to_string();

        text_query
            .get_mut(text_references.entities[POPUP_TRAINS_AMOUNT])
            .unwrap()
            .0 = "1".to_string();

        for i in POPUP_LINE_HANDLER..8 + lines_vec.len() - 5 {
            text_query.get_mut(text_references.entities[i]).unwrap().0 =
                lines_vec[i - POPUP_LINE_HANDLER].name.clone();
        }

        let camera = camera_q.get_single().unwrap();

        *position = Transform::from_xyz(
            cursor_pos.0.x + POPUP_WIDTH / 2. * camera.target_zoom,
            cursor_pos.0.y - POPUP_HEIGHT / 2. * camera.target_zoom,
            10.,
        );
        *popup_visibility = Visibility::Visible;
    }
}
fn change_visibility_of_lines(
    mut ev_change_vision: EventReader<ChangeLinesVisibility>,
    lines: Res<LinesResource>,
    mut lines_q: Query<&mut Visibility>,
) {
    for _ev in ev_change_vision.read() {
        for i in lines.entities.clone() {
            *lines_q.get_mut(i).unwrap() = Visibility::Hidden;
        }
    }
}