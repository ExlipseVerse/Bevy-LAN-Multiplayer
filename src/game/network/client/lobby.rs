use std::collections::HashMap;
use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct ClientLobby {
    pub players: HashMap<u64, Entity>,
}