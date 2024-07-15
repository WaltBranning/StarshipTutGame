use bevy::{prelude::*, render::camera::Exposure};

const CAMERA_DISTANCE: f32 = 80.0;

pub struct CameraPlugin;
impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera);
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, CAMERA_DISTANCE, 0.0).looking_at(Vec3::ZERO, Vec3::Z),
        exposure: Exposure {
            ev100: Exposure::EV100_INDOOR,
        },
        camera: Camera {
            order: 0,
            ..default()
        },
        ..default()
    });
}

pub struct CameraPlugin2d;
impl Plugin for CameraPlugin2d {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_2d_camera);
    }
}

fn spawn_2d_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(0.0, CAMERA_DISTANCE, 0.0),
        camera: Camera {
            order: 1,
            ..default()
        },
        ..default()
    });
}
