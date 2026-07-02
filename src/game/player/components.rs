use bevy::prelude::*;


#[derive(Component)]
pub struct LocalPlayer;

#[derive(Component)]
pub struct RemoteTarget {
    pub position: Vec3,
}

#[derive(Component)]
pub struct Player {
	pub speed: f32,
	pub ground_acceleration: f32,
	pub ground_deacceleration: f32,
	pub air_acceleration: f32,
	pub air_deacceleration: f32,
	pub jumpforce: f32,
}

#[derive(Component, Default)]
pub struct Grounded(pub bool);