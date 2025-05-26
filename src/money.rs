use bevy::prelude::*;

pub struct MoneyPlugin;

pub const TRAIN_COST: u32 = 50;

impl Plugin for MoneyPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Money>();
    }
}

#[derive(Resource)]
pub struct Money(pub u32);

impl Default for Money {
    fn default() -> Self {
        Self(99999)
    }
}
