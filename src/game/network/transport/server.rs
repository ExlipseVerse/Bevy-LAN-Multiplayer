use bevy::prelude::*;
use bevy_renet::RenetServer;
use bevy_renet::netcode::{ServerAuthentication, NetcodeServerTransport, ServerConfig};
use renet::ConnectionConfig;
use std::net::{UdpSocket, SocketAddr};
use std::time::SystemTime;

use super::super::channels::{ClientChannel, ServerChannel};
use super::super::config::{SERVER_ADDR, PROTOCOL_ID};

pub fn create_server(cmds: &mut Commands) {
	let server = RenetServer::new(ConnectionConfig {
		server_channels_config: ServerChannel::channel_config(),
		client_channels_config: ClientChannel::channel_config(),
		..Default::default()
	});

	let pub_address: SocketAddr = SERVER_ADDR.parse().unwrap();
	let socket = UdpSocket::bind(pub_address).unwrap();
	let curr_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();

	let server_config = ServerConfig {
		current_time: curr_time,
		max_clients: 64,
		protocol_id: PROTOCOL_ID,
		public_addresses: vec![pub_address],
		authentication: ServerAuthentication::Unsecure
	};

	let transport = NetcodeServerTransport::new(server_config, socket).unwrap();
	cmds.insert_resource(server);
	cmds.insert_resource(transport);

	println!("Server started on {}", SERVER_ADDR);
}