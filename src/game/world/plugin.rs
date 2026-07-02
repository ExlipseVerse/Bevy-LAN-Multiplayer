use bevy::prelude::*;

// use crate::game::state::GameState;

use super::{
	spawn::spawn_world,
    systems::setup_gravity,
};

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app
        	.add_systems(Startup, (setup_gravity, spawn_world));
    }
}