use bevy::prelude::*;
use avian3d::prelude::*;

use crate::game::player::components::Grounded;

use super::super::camera::components::{CameraPivot};
use super::super::components::{Player, LocalPlayer};

pub fn read_input(
    time: Res<Time>,
    keyboard: Res<ButtonInput<KeyCode>>, 
    camera_query: Query<&Transform, With<CameraPivot>>,
    mut player_query: Query<(&Player, &mut LinearVelocity, &Grounded), With<LocalPlayer>>
) {
    // let (player, mut velocity, grounded) = player_query.single_mut().unwrap();
    
    // let camera_transform = camera_query.single().unwrap(); 

    let Ok((player, mut velocity, grounded)) = player_query.single_mut() else {
        return;
    };

    let Ok(camera_transform) = camera_query.single() else {
        return;
    };

    let mut forward = camera_transform.forward().as_vec3();
    forward.y = 0.0;
    forward = forward.normalize_or_zero();

    let mut right = camera_transform.right().as_vec3();
    right.y = 0.0;
    right = right.normalize_or_zero();

    let mut mov_dir = Vec3::ZERO;

    if keyboard.pressed(KeyCode::KeyW) {
        mov_dir += forward;
    }
    if keyboard.pressed(KeyCode::KeyS) {
        mov_dir -= forward;
    }
    if keyboard.pressed(KeyCode::KeyA) {
        mov_dir -= right;
    }
    if keyboard.pressed(KeyCode::KeyD) {
        mov_dir += right;
    }

    let target = mov_dir.normalize_or_zero() * player.speed;

    let acceleration = if grounded.0 {
        if mov_dir == Vec3::ZERO {
            player.ground_deacceleration
        } else {
            player.ground_acceleration
        }
    } else {
        if mov_dir == Vec3::ZERO {
            player.air_acceleration
        } else {
            player.air_deacceleration
        }
        
    };

    velocity.x = velocity.x.lerp(target.x, acceleration * time.delta_secs());
    velocity.z = velocity.z.lerp(target.z, acceleration * time.delta_secs());
}

