use bevy::state::state::States;

#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum GameState {
	#[default]
	MainMenu,
	Connecting,
	InGame,	
}