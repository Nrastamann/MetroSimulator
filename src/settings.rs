use bevy::prelude::*;

pub struct SettingsPlugin;

impl Plugin for SettingsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Settings>();
    }
}

#[derive(Resource)]
pub struct Money(pub u32);

impl Default for Money {
    fn default() -> Self {
        Self(99999)
    }
}
