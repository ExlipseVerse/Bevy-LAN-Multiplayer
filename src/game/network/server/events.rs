use bevy::prelude::*;
use bevy_renet::{RenetServer, RenetServerEvent};
use bevy_renet::renet::ServerEvent;
use super::super::messages::ServerMessage;
use super::super::channels::ServerChannel;

pub fn connection_events(
    server_event: On<RenetServerEvent>,
    server: Option<ResMut<RenetServer>>,
) {
    let Some(mut server) = server else { return; };

    match **server_event {
        ServerEvent::ClientConnected { client_id } => {
            println!("Client {} connected!", client_id);

            for existing_id in server.clients_id() {
                if existing_id == client_id { continue; }
                let msg = ServerMessage::SpawnPlayer { id: existing_id, position: Vec3::new(0.0, 1.5, 0.0) };
                server.send_message(client_id, ServerChannel::ServerMessages as u8, bincode::serialize(&msg).unwrap());
            }
            let assign = ServerMessage::AssignClient { id: client_id };
            server.send_message(client_id, ServerChannel::ServerMessages as u8, bincode::serialize(&assign).unwrap());

            let spawn = ServerMessage::SpawnPlayer { id: client_id, position: Vec3::new(0.0, 1.5, 0.0) };
            server.broadcast_message(ServerChannel::ServerMessages as u8, bincode::serialize(&spawn).unwrap());
        }
        ServerEvent::ClientDisconnected { client_id, ref reason } => {
            println!("Client {} disconnected: {:?}", client_id, reason);
            let msg = ServerMessage::DespawnPlayer { id: client_id };
            server.broadcast_message(ServerChannel::ServerMessages as u8, bincode::serialize(&msg).unwrap());
        }
    }
}