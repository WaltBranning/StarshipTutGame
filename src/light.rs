use bevy::prelude::*;

const POINT_LIGHT_DISTANCE: f32 = 80.0;

pub struct PointLightPlugin;
impl Plugin for PointLightPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_point_light);
    }
}

fn spawn_point_light(mut commands: Commands){
    info!("Point Light Spawned");
    commands.spawn(PointLightBundle {
        point_light: PointLight {intensity: 10000.0, ..default()},
        transform: Transform::from_xyz(0.0, POINT_LIGHT_DISTANCE, 0.0),
        ..default()
    });
}