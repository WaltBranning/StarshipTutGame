use bevy::{core_pipeline::core_2d::graph::input, prelude::*, transform};
use crate::{asset_loader::SceneAssets, movement::{Acceleration, MovingObjectBundle, Velocity}};


const STARTING_TRANSLATION: Vec3 = Vec3::new(0.0, 0.0, -20.0);
const SPACESHIP_SPEED: f32 = 25.0;
const SPACESHIP_ROTATION_SPEED: f32 = 2.5;
const SPACESHIP_ROLL_SPEED: f32 = 2.5;



#[derive(Component, Debug)]
pub struct Spaceship;

pub struct SpaceShipPlugin;
impl Plugin for SpaceShipPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, spawn_spaceship)
            .add_systems(Update, spaceship_movement_controls);
    }    
}

fn spawn_spaceship(mut commands: Commands, scene_assets: Res<SceneAssets>) {
    commands.spawn((MovingObjectBundle {
        velocity: Velocity::new(Vec3::ZERO),
        acceleration: Acceleration::new(Vec3::ZERO),
        model: SceneBundle {
            scene: scene_assets.spaceship.clone(),
            transform: Transform::from_translation(STARTING_TRANSLATION),
            ..default()
            },
        },
        Spaceship
    ));
}

fn spaceship_movement_controls(
    mut query: Query<(&mut Transform, &mut Velocity), With<Spaceship>>, 
    keyboard_input: Res<ButtonInput<KeyCode>>, 
    time: Res<Time>) 
    {
        use Directions::{Forward, RightBank, LeftBank}; 

        let (mut transform, mut velocity) = query.single_mut();
        let mut rotation = 0.0;
        let mut roll = 0.0;
        let mut movement = 0.0;
            
        let banking = directional_bank(keyboard_input.clone()).unwrap_or(Forward);
        // info!("{:?}", banking);

        if keyboard_input.pressed(KeyCode::KeyS) {
            movement = -SPACESHIP_SPEED;
        } else if keyboard_input.pressed(KeyCode::KeyW) {
            movement = SPACESHIP_SPEED;
        } 
        
        if keyboard_input.pressed(KeyCode::KeyD) {
            rotation = -SPACESHIP_ROTATION_SPEED * time.delta_seconds();
        } else if keyboard_input.pressed(KeyCode::KeyA) {
            rotation = SPACESHIP_ROTATION_SPEED * time.delta_seconds();
        }

        // let roll = match directional_bank(keyboard_input.clone()).unwrap_or_default() {
        //     RightBank if transform.rotation.z < 0.5 => SPACESHIP_ROLL_SPEED * time.delta_seconds(),
        //     LeftBanK if transform.rotation.z > -0.5 => SPACESHIP_ROLL_SPEED * time.delta_seconds(),
        //     _ => {roll}
            
        // };


        if matches!(directional_bank(keyboard_input.clone()).unwrap_or_default(), LeftBank) && transform.rotation.z > -0.25 {
            roll = -SPACESHIP_ROLL_SPEED * time.delta_seconds();
            info!("x is {:?}", transform.rotation.x);
        } else if matches!(directional_bank(keyboard_input.clone()).unwrap_or_default(), RightBank) && transform.rotation.z < 0.25 {
            roll = SPACESHIP_ROLL_SPEED * time.delta_seconds();
            // info!("z is {:?}", transform.rotation.z);
        
        } else if keyboard_input.pressed(KeyCode::ShiftLeft) && transform.rotation.z > -0.5 {
            roll = -SPACESHIP_ROLL_SPEED * time.delta_seconds();
            // info!("z is {:?}", transform.rotation.z);
        } else if keyboard_input.pressed(KeyCode::KeyV) && transform.rotation.z < 0.5 {
            roll = SPACESHIP_ROLL_SPEED * time.delta_seconds();
            // info!("z is {:?}", transform.rotation.z);
        }
        
        else {
            
            if transform.rotation.z  > 0.0 {
                roll = -SPACESHIP_ROLL_SPEED * time.delta_seconds();
            } else if transform.rotation.z < 0.0 {
                roll = SPACESHIP_ROLL_SPEED * time.delta_seconds();
            }
        }
        
        info!("Z axis is: {:?} Y axis is: {:?} X axis is {:?}", transform.rotation.z, transform.rotation.y, transform.rotation.x);
        velocity.value = -transform.forward() * movement;
        transform.rotate_y(rotation);
        transform.rotate_local_z(roll)
}

#[derive(Default, PartialEq)]
enum Directions {
    Right,
    Left,
    RightBank,
    LeftBank,
    #[default]
    Forward
}

impl std::fmt::Debug for Directions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Right => write!(f, "Right"),
            Self::Left => write!(f, "Left"),
            Self::RightBank => write!(f, "RightBank"),
            Self::LeftBank => write!(f, "LeftBank"),
            Self::Forward => write!(f, "Forward"),
        }
    }
}

fn directional_bank(input: ButtonInput<KeyCode>) -> Option<Directions> {
    if input.pressed(KeyCode::KeyW) && input.pressed(KeyCode::KeyD) {
        Some(Directions::RightBank)
    } else if input.pressed(KeyCode::KeyW) && input.pressed(KeyCode::KeyA) {
        Some(Directions::LeftBank)
    } else { None }
 }

fn check_sign(number: f32) -> f32 {
    if number > 0.0 {
        0.0
    } else {
        -1.0
    }
}