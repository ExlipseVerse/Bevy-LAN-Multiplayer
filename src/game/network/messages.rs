use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Message)]
pub struct HostGame;

#[derive(Message)]
pub struct JoinGame;

#[derive(Debug, Serialize, Deserialize)]
pub enum ServerMessage {
	AssignClient {
		id: u64
	},

	SpawnPlayer{
		id: u64,
		position: Vec3,
	},

	DespawnPlayer {
		id: u64,
	},

	UpdatePosition { id: u64, position: Vec3 },
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ClientMessage {
	PlayerUpdate { position: Vec3 },
}