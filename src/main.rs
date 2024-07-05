mod debug;
mod movement;
mod spaceship;
mod camera;
mod asteroids;
mod light;
mod asset_loader;

use bevy::prelude::*;
use asset_loader::AssetLoaderPlugin;
use spaceship::SpaceShipPlugin;
use asteroids::AsteroidPlugin;
use movement::MovementPlugin;
use camera::CameraPlugin;
use light::PointLightPlugin;
use debug::DebugPlugin;


fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.1, 0.0, 0.15)))
        .insert_resource(AmbientLight{color: Color::default(), brightness: 150.0})
        .add_plugins((
            CameraPlugin,
            AssetLoaderPlugin,
            SpaceShipPlugin, 
            MovementPlugin,
            AsteroidPlugin,
            PointLightPlugin, 
            // DebugPlugin
        )) 
        .add_plugins(DefaultPlugins)
        .run();
}





