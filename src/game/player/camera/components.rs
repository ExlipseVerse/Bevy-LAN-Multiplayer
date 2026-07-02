use bevy::prelude::*;

#[derive(Component)]
pub struct MainCamera;

#[derive(Component)]
pub struct RenderCamera;

#[derive(Component)]
pub struct SpringArm {
	pub length: f32,
	pub target_length: f32,
	pub min_length: f32,
	pub max_length: f32,
}

#[derive(Component)]
pub struct CameraPivot;

#[derive(Component)]
pub struct CameraController {
	pub yaw: f32,
	pub pitch: f32,
}