use bevy::prelude::*;

use super::{
	player::plugin::PlayerPlugin,
	ui::plugin::UIPlugin,
	world::plugin::WorldPlugin,
	window::plugin::WindowPlugin,
	network::plugin::NetworkPlugin,
};

pub struct GamePlugins;

impl Plugin for GamePlugins {
	fn build(&self, app: &mut App) {
		app 
			.add_plugins(PlayerPlugin)
			.add_plugins(UIPlugin)
			.add_plugins(WorldPlugin)
			.add_plugins(WindowPlugin)
			.add_plugins(NetworkPlugin);

		println!("plugins done");
	}
}