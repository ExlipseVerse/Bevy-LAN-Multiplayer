//C:\rubbish\rust_game\game\target\debug\game.exe

mod game;

use std::env;

use avian3d::PhysicsPlugins;
use bevy::prelude::*;
use bevy_egui::{EguiPlugin};

use game::{plugin::GamePlugins, resources::GameSettings, state::GameState};


#[derive(Resource)]
pub struct NetworkMode {
    pub is_host: bool,
}

fn main() {
	let is_host = env::args().any(|arg| arg == "--host");

	App::new()
	.add_plugins(DefaultPlugins)
	.add_plugins(PhysicsPlugins::default())
	.add_plugins(EguiPlugin::default())
	.init_state::<GameState>()
	.add_plugins(GamePlugins)
	.insert_resource(GameSettings {
		mouse_sensitivity: 0.003,
	})
	.insert_resource(NetworkMode { is_host })
	.run();	

	println!("initialized");
}