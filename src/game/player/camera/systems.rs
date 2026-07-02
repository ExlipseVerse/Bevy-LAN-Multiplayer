use bevy::prelude::*;

use bevy::{
    ecs::message::MessageReader,
    input::mouse::{MouseMotion, MouseWheel},
};

use crate::game::resources::{GameSettings};

use super::components::{CameraController, CameraPivot, SpringArm};


pub fn camera_look(
    mut mouse_events: MessageReader<MouseMotion>,
    settings: Res<GameSettings>,
    mut camera: Query<(&mut CameraController, &mut Transform),  With<CameraPivot>>
) {
	// let (mut controller, mut transform) = camera.single_mut().unwrap();
	let Ok((mut controller, mut transform)) = camera.single_mut() else {
	    return;
	};
	let mut delta = Vec2::ZERO;

	for event in mouse_events.read() {
		delta += event.delta;
	}

	controller.yaw -= delta.x * settings.mouse_sensitivity;
	controller.pitch -= delta.y * settings.mouse_sensitivity;

	let max_look_up = -45.0_f32.to_radians(); 
    controller.pitch = controller.pitch.clamp(max_look_up, 0.0);

	transform.rotation = Quat::from_rotation_y(controller.yaw) * Quat::from_rotation_x(controller.pitch);
}

pub fn spring_arm(
	time: Res<Time>,
	mut wheel: MessageReader<MouseWheel>,
	mut arms: Query<(&mut SpringArm, &mut Transform)>
) {
	// let (mut arm, mut transform) = arms.single_mut().unwrap();

	let Ok((mut arm, mut transform)) = arms.single_mut() else {
	    return;
	};
	
	for event in wheel.read() {
		arm.target_length -= event.y * 0.5;
		arm.target_length = arm.target_length.clamp(arm.min_length, arm.max_length);
	}

	arm.length = arm.length.lerp(arm.target_length, 10.0 * time.delta_secs());

	transform.translation = Vec3::new(0.0, 0.0, arm.length);
}