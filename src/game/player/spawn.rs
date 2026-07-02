use avian3d::prelude::*;
use bevy::prelude::*;

use super::components::{Player, Grounded, RemoteTarget};
use super::camera::components::{CameraController, CameraPivot, MainCamera, SpringArm, RenderCamera};

use crate::game::network::components::NetworkPlayer;

pub fn spawn_local_player(
	cmds: &mut Commands,
	meshes: &mut ResMut<Assets<Mesh>>,
	materials: &mut ResMut<Assets<StandardMaterial>>,
) -> Entity {

	cmds.spawn((
		Player{
			speed: 10.0,
			ground_acceleration: 40.0,
			ground_deacceleration: 20.0,
			air_acceleration: 10.0,
			air_deacceleration: 5.0,
			jumpforce: 8.0,
		},
		Grounded(false),
		LinearVelocity::ZERO,
		RigidBody::Dynamic,
		LockedAxes::ROTATION_LOCKED,
		Collider::capsule(0.5, 1.0),
		Mesh3d(meshes.add(Capsule3d::default())),
		MeshMaterial3d(materials.add(Color::srgb(0.2, 0.8, 0.4))),
		Transform::from_xyz(0.0, 1.5, 0.0)
	)).id()
}

pub fn spawn_remote_puppet(
	commands: &mut Commands,
	meshes: &mut ResMut<Assets<Mesh>>,
	materials: &mut ResMut<Assets<StandardMaterial>>,
	id: u64,
	position: Vec3,
) -> Entity {
	commands.spawn((
		NetworkPlayer { id },
		RemoteTarget { position },
		Mesh3d(meshes.add(Capsule3d::default())),
		MeshMaterial3d(materials.add(Color::srgb(
            0.8,
            0.3,
            0.2,
        ))),

        Transform::from_translation(position),
	)).id()
}

pub fn spawn_camera(
	commands: &mut Commands,
	player: Entity,
) {
	commands.entity(player)
		.with_children(|parent| {
		    parent.spawn((
		        CameraPivot,
		        CameraController {
		            yaw: 0.0,
		            pitch: -30.0_f32.to_radians(),
		        },
		        Transform::default(),
		    ))
		    .with_children(|parent| {
		        parent.spawn((
		            SpringArm { length: 8.0, target_length: 8.0, min_length: 3.0, max_length: 15.0, },
		            Transform::from_xyz(0.0, 0.0, 8.0),
		        ))
		        .with_children(|parent| {
		            parent.spawn((
		                MainCamera,
		                Camera3d::default(),
		                Transform::default(),
		            ));
		   
			     });
		    });
		});
}

pub fn spawn_render_camera(mut commands: Commands) {
    commands.spawn((
    	RenderCamera,
        Camera3d::default(),
        Transform::from_xyz(0.0, 5.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

pub fn despawn_render_camera(
    mut commands: Commands, 
    query: Query<Entity, With<RenderCamera>>
) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}