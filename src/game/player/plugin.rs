use bevy::prelude::*;

use crate::game::state::GameState;

use super::{
	spawn::{spawn_render_camera, despawn_render_camera},
	camera::systems::{camera_look, spring_arm},
	actions::{
		movement::{read_input},
		jump::jump,
	},
	states::{
		grounded::ground_check
	}
};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
        	.add_systems(OnEnter(GameState::MainMenu), spawn_render_camera)
        	.add_systems(OnExit(GameState::Connecting), despawn_render_camera)
        	.add_systems(Update, (
        		ground_check,
        		read_input,
        		jump,
        		camera_look,
        		spring_arm,
        	).run_if(in_state(GameState::InGame)));
    }
}