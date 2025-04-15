use bevy::prelude::*;

use crate::{cursor::CursorPosition, metro::Direction, station::StartBuildingEvent, GameState, DISTRICT_CELL_SIZE};

#[derive(Component)]
pub struct StationBlueprint {
    pub material: Handle<ColorMaterial>,
    pub connection: (i32, i32),
    pub direction: Direction,
    pub line_to_attach: usize, //if we want to add new line, send -1
    pub can_build: bool,
    pub menu_flag: bool,
}

pub struct StationBlueprintPlugin;

impl Plugin for StationBlueprintPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SetBlueprintColorEvent>();
        app.add_systems(Startup, init_blueprint);
        app.add_systems(
            Update,
            (stick_to_mouse, toggle_station_blueprint, start_building).run_if(in_state(GameState::InGame)),
        );
    }
}

fn init_blueprint(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let material = materials.add(Color::WHITE.with_alpha(0.0));
    commands.spawn((
        Mesh2d(meshes.add(Circle::new(25.))),
        MeshMaterial2d(material.clone()),
        StationBlueprint {
            material: material,
            connection: (0, 0),
            direction: Direction::Forwards,
            line_to_attach: 0,
            can_build: true,
            menu_flag: false,
        },
        Visibility::Hidden,
    ));
}

fn stick_to_mouse(
    mut q_blueprint: Query<&mut Transform, With<StationBlueprint>>,
    cursor_position: Res<CursorPosition>,
    cursor_position: Res<CursorPosition>,
) {
    for mut blueprint_transform in q_blueprint.iter_mut() {
        blueprint_transform.translation = Vec3::new(
            (cursor_position.0.x / DISTRICT_CELL_SIZE).round() * DISTRICT_CELL_SIZE,
            (cursor_position.0.y / DISTRICT_CELL_SIZE).round() * DISTRICT_CELL_SIZE,
            1.0,
        );
    }
}

#[derive(Event)]
pub struct SetBlueprintColorEvent(pub Color);

fn toggle_station_blueprint(
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut q_blueprint: Query<&StationBlueprint>,
    mut ev_toggle: EventReader<SetBlueprintColorEvent>,
) {
    for ev in ev_toggle.read() {
        for blueprint in q_blueprint.iter_mut() {
            if let Some(material) = materials.get_mut(&blueprint.material) {
                material.color = ev.0;
            }
        }
    }
}

fn start_building(
    mut ev_set_blueprint: EventReader<StartBuildingEvent>,
    mut blueprint_q: Query<(&mut StationBlueprint, &mut Visibility)>,
) {
    for ev in ev_set_blueprint.read() {
        let Ok((mut blueprint, mut vision)) = blueprint_q.get_single_mut() else{
            panic!("NO BLUEPRINT");
        };
        
        blueprint.connection = ev.connection;
        blueprint.direction = ev.direction;
        blueprint.line_to_attach = ev.line_to_attach;
        blueprint.menu_flag = ev.from_menu;
        *vision = Visibility::Visible;
    }
}