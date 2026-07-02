use bevy::prelude::*;

use super::{
	systems::setup_cursor,
};

pub struct WindowPlugin;

impl Plugin for WindowPlugin {
    fn build(&self, app: &mut App) {
        app
        	.add_systems(Update, setup_cursor);
    }
}