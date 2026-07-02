use bevy::prelude::*;
use crate::game::player::components::RemoteTarget;

pub fn interpolate_remote_plrs(time: Res<Time>, mut query: Query<(&RemoteTarget, &mut Transform)>) {
	let smoothing = 1.0 - (-12.0_f32 * time.delta_secs()).exp();

	for (target, mut transform) in &mut query {
		transform.translation = transform.translation.lerp(target.position, smoothing);
	}
}