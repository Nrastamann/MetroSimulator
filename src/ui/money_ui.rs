use crate::{
    money::{self, Money},
    GameState,
};
use bevy::prelude::*;
use bevy_lunex::*;

use super::{UIStyles, UI_FONT};

pub struct MoneyUIPlugin;

impl Plugin for MoneyUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<MoneyRedrawEvent>();
        app.add_systems(OnEnter(GameState::InGame), MoneyUi::spawn_money_ui);
        app.add_systems(Update, MoneyUi::update);
    }
}
#[derive(Event)]
pub struct MoneyRedrawEvent;
#[derive(Component)]
pub struct MoneyUi;

#[derive(Component)]
pub struct RedrawMoney;

impl MoneyUi {
    fn spawn_money_ui(mut commands: Commands, asset_server: Res<AssetServer>, money: Res<Money>) {
        commands
            .spawn((
                UiLayoutRoot::new_2d(),
                StateScoped(GameState::InGame),
                UiFetchFromCamera::<0>,
                MoneyUi,
            ))
            .with_children(|ui| {
                ui.spawn((
                    UiLayout::window()
                        .anchor_left()
                        .rl_size(20., 5.)
                        .rl_pos(80., 95.)
                        .pack(),
                    Sprite::default(),
                    UiColor::from(Color::WHITE),
                ))
                .with_children(|ui| {
                    ui.spawn((
                        UiLayout::window().anchor_left().pack(),
                        UiColor::from(Color::BLACK.with_alpha(0.95)),
                        UiTextSize::from(Rh(100.)),
                        Text2d::new(money.0.to_string()),
                        TextFont {
                            font: asset_server.load(UI_FONT),
                            font_size: 96.,
                            ..default()
                        },
                        TextLayout {
                            justify: JustifyText::Center,
                            linebreak: LineBreak::WordBoundary,
                        },
                        RedrawMoney,
                    ));
                });
            });
    }
//разбить одну линию на две, + добавлять новые на основе существующих
    fn update(
        mut ev_redraw: EventReader<MoneyRedrawEvent>,
        money_res: Res<Money>,
        mut redraw_text: Query<&mut Text2d, (With<RedrawMoney>)>,
    ) {
        for ev in ev_redraw.read() {
            let Ok(mut text) = redraw_text.get_single_mut() else {
                println!("WHAT");
                return;
            };

            text.0 = money_res.0.to_string();
        }
    }
}
