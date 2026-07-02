use bevy::prelude::*;
use bevy_renet::RenetClient;
use bevy_renet::netcode::{ClientAuthentication, NetcodeClientTransport};
use renet::ConnectionConfig;
use std::net::UdpSocket;
use std::time::SystemTime;

use crate::game::state::GameState;

use super::super::channels::{ServerChannel, ClientChannel};
use super::super::config::{SERVER_ADDR, PROTOCOL_ID};

pub fn create_client(commands: &mut Commands, client_id: u64) {
	let client = RenetClient::new(ConnectionConfig {
	        server_channels_config: ServerChannel::channel_config(),
            client_channels_config: ClientChannel::channel_config(),
	        ..Default::default()
	    });

	let server_addr = SERVER_ADDR.parse().unwrap();
    let socket = UdpSocket::bind("127.0.0.1:0").unwrap();
    let current_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();

    let authentication = ClientAuthentication::Unsecure {
        server_addr,
        client_id,
        user_data: None,
        protocol_id: PROTOCOL_ID,
    };

    let transport = NetcodeClientTransport::new(current_time, authentication, socket).unwrap();

    commands.insert_resource(client);
    commands.insert_resource(transport);

    println!("Client {} trying to connect...", client_id);
}