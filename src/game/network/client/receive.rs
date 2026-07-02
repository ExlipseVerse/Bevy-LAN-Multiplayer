use bevy::prelude::*;
use bevy_renet::RenetClient;
use crate::game::network::{channels::ServerChannel, messages::ServerMessage};
use super::state::LocalClient;
use super::lobby::ClientLobby;
use crate::game::player::components::{LocalPlayer, RemoteTarget};
use crate::game::player::spawn::{spawn_local_player, spawn_remote_puppet, spawn_camera};

pub fn receive_message(
    mut client: ResMut<RenetClient>,
    mut local_client: ResMut<LocalClient>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut lobby: ResMut<ClientLobby>,
    mut targets: Query<&mut RemoteTarget>,
) {
    while let Some(message) = client.receive_message(ServerChannel::ServerMessages as u8) {
        let message: ServerMessage = bincode::deserialize(&message).unwrap();
        match message {
            ServerMessage::AssignClient { id } => {
                local_client.id = Some(id);
                let player = spawn_local_player(&mut commands, &mut meshes, &mut materials);
                commands.entity(player).insert(LocalPlayer);
                spawn_camera(&mut commands, player);
                lobby.players.insert(id, player);
                println!("I am client {}", id);
            }
            ServerMessage::SpawnPlayer { id, position } => {
                if local_client.id == Some(id) { continue; }
                if lobby.players.contains_key(&id) { continue; }
                let entity = spawn_remote_puppet(&mut commands, &mut meshes, &mut materials, id, position);
                lobby.players.insert(id, entity);
            }
            ServerMessage::DespawnPlayer { id } => {
                if let Some(entity) = lobby.players.remove(&id) {
                    commands.entity(entity).despawn();
                }
            }
            ServerMessage::UpdatePosition { .. } => {} // arrives on the other channel, handled below
        }
    }

    while let Some(message) = client.receive_message(ServerChannel::PlayerUpdates as u8) {
        let message: ServerMessage = bincode::deserialize(&message).unwrap();
        if let ServerMessage::UpdatePosition { id, position } = message {
            if local_client.id == Some(id) { continue; }
            if let Some(&entity) = lobby.players.get(&id) {
                if let Ok(mut target) = targets.get_mut(entity) {
                    target.position = position;
                }
            }
        }
    }
}