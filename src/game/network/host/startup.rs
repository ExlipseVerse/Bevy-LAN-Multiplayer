use bevy::prelude::*;

use super::super::transport::{client::create_client, server::create_server};

use crate::game::{state::GameState, ui::misc::resource::WaitScreenText};

use super::super::messages::{HostGame, JoinGame};


pub fn host_game(mut messages: MessageReader<HostGame>, mut commands: Commands, mut next_state: ResMut<NextState<GameState>>) {
	for _ in messages.read() {
        next_state.set(GameState::Connecting);
        commands.insert_resource(WaitScreenText("Hosting in process...".to_string()));

        create_server(&mut commands);
        create_client(&mut commands, 1);
    }
}


pub fn join_game(mut messages: MessageReader<JoinGame>, mut commands: Commands, mut next_state: ResMut<NextState<GameState>>) {
	for _ in messages.read() {
        next_state.set(GameState::Connecting);
        commands.insert_resource(WaitScreenText("Connecting to the host...".to_string()));
        create_client(&mut commands, 2);
    }
}
