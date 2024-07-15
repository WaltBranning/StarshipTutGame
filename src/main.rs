mod debug;
mod movement;
mod spaceship;
mod camera;
mod asteroids;
mod light;
mod asset_loader;
mod windows;

use bevy::prelude::*;
// use windows::WindowPlugin;
use asset_loader::AssetLoaderPlugin;
use spaceship::SpaceShipPlugin;
use asteroids::AsteroidPlugin;
use movement::MovementPlugin;
use camera::CameraPlugin;
use camera::CameraPlugin2d;
use light::PointLightPlugin;
use debug::DebugPlugin;


fn main() {
    App::new()
        // .insert_resource(ClearColor(Color::rgb(0.05, 0.05, 0.01)))
        .insert_resource(AmbientLight{color: Color::default(), brightness: 150.0})
        .add_plugins((
            // WindowPlugin,
            CameraPlugin,
            CameraPlugin2d,
            AssetLoaderPlugin,
            SpaceShipPlugin, 
            MovementPlugin,
            AsteroidPlugin,
            PointLightPlugin, 
            // DebugPlugin
            DefaultPlugins.set(
                WindowPlugin {
                    primary_window: Some(Window {
                        title: "Galatic Buccaneers".into(),
                        resolution: (900., 1080.).into(),
                    ..default()
                    }),
                    ..default()
                }
            )
        )) 
        // .add_plugins(DefaultPlugins)
        .run();
}





