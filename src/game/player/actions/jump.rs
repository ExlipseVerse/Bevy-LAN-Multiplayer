use bevy::prelude::*;
use avian3d::prelude::*;

use super::super::components::{Player, Grounded, LocalPlayer};

pub fn jump(
	keyboard: Res<ButtonInput<KeyCode>>,
	mut player: Query<(&Player, &Grounded, &mut LinearVelocity), With<LocalPlayer>>
) {
	// let (player, grounded, mut velocity) = player.single_mut().unwrap();
	let Ok((player, grounded, mut velocity)) = player.single_mut() else {
	    return;
	};
	
	if !grounded.0 {
		return;
	}

	if keyboard.just_pressed(KeyCode::Space) {
		velocity.y = player.jumpforce;
	}
}