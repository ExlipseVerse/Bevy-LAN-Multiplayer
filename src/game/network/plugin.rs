use bevy::prelude::*;
use bevy_renet::netcode::{NetcodeClientPlugin, NetcodeServerPlugin};
use bevy_renet::{RenetClientPlugin, RenetServerPlugin, RenetServer, RenetClient};
use super::server::{events::connection_events, receive::relay_position_updates};
use super::client::{events::client_connection, receive::receive_message, interpolate::interpolate_remote_plrs, send::send_position_update, state::LocalClient, lobby::ClientLobby};

use super::host::startup::{host_game, join_game};

use crate::game::state::GameState;
use super::messages::{HostGame, JoinGame};

pub struct NetworkPlugin;
impl Plugin for NetworkPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((RenetServerPlugin, NetcodeServerPlugin, RenetClientPlugin, NetcodeClientPlugin))
            .add_systems(Update, (host_game, join_game)
                .run_if(in_state(GameState::MainMenu))
            )
            .add_systems(Update, client_connection.run_if(in_state(GameState::Connecting)))
            .add_systems(Update,
                relay_position_updates
                    .run_if(in_state(GameState::InGame))
                    .run_if(resource_exists::<RenetServer>)
            )
            .add_systems(Update, (
                receive_message,
                send_position_update,
                interpolate_remote_plrs,
            ).run_if(in_state(GameState::InGame).and(resource_exists::<RenetClient>)))
            .add_observer(connection_events)
            .insert_resource(LocalClient::default())
            .insert_resource(ClientLobby::default())
            .add_message::<HostGame>()
            .add_message::<JoinGame>();
    }
}