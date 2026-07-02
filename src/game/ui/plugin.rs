use bevy::prelude::*;
use bevy_egui::prelude::*;

use super::main_menu::{main::main_menu, play::play_menu, settings::settings_menu};
use super::misc::{wait::wait_ui, resource::WaitScreenText};

use super::state::MenuState;
use crate::game::state::GameState;

pub struct UIPlugin;

impl Plugin for UIPlugin {
	fn build(&self, app: &mut App) {
	    app 
	    	.init_state::<MenuState>()
	    	.add_systems(EguiPrimaryContextPass, main_menu.run_if(in_state(GameState::MainMenu)).run_if(in_state(MenuState::Main)))
	    	.add_systems(EguiPrimaryContextPass, play_menu.run_if(in_state(GameState::MainMenu)).run_if(in_state(MenuState::Play)))
	    	.add_systems(EguiPrimaryContextPass, settings_menu.run_if(in_state(GameState::MainMenu)).run_if(in_state(MenuState::Settings)))
	    	.add_systems(EguiPrimaryContextPass, wait_ui.run_if(resource_exists::<WaitScreenText>));
	}
} 