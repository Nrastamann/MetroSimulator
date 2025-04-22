use bevy::prelude::*;

pub struct MoneyPlugin;

impl Plugin for MoneyPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Money>();
    }
} 

#[derive(Resource)]
pub struct Money(pub u32);

impl Default for Money {
    fn default() -> Self {
        Self(500)
    }
}