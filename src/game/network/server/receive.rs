use bevy::prelude::*;
use bevy_renet::RenetServer;
use super::super::channels::{ServerChannel, ClientChannel};
use super::super::messages::{ClientMessage, ServerMessage};

pub fn relay_position_updates(mut server: ResMut<RenetServer>) {
    for client_id in server.clients_id() {
        while let Some(message) = server.receive_message(client_id, ClientChannel::PlayerUpdate as u8) {
            if let Ok(ClientMessage::PlayerUpdate { position }) = bincode::deserialize(&message) {
                let update = ServerMessage::UpdatePosition { id: client_id, position };
                server.broadcast_message(ServerChannel::PlayerUpdates as u8, bincode::serialize(&update).unwrap());
            }
        }
    }
}