use bevy::prelude::*;
use renet::{ChannelConfig, SendType};
use std::time::Duration;

#[repr(u8)]
pub enum ServerChannel {
	ServerMessages = 0,
	PlayerUpdates = 1,
}

impl ServerChannel {
	pub fn channel_config() -> Vec<ChannelConfig> {
		vec![ChannelConfig {
			channel_id: Self::ServerMessages as u8,
			max_memory_usage_bytes: 10 * 1024 * 1024,
			send_type: SendType::ReliableOrdered {
                    resend_time: Duration::from_millis(300),
                },
		},
		ChannelConfig {
                channel_id: Self::PlayerUpdates as u8,
                max_memory_usage_bytes: 10 * 1024 * 1024,
                send_type: SendType::Unreliable,
            }]
	}
}

#[repr(u8)]
pub enum ClientChannel {
    PlayerUpdate = 0
}

impl ClientChannel {
	pub fn channel_config() -> Vec<ChannelConfig> {
		vec![ChannelConfig {
            channel_id: Self::PlayerUpdate as u8,
            max_memory_usage_bytes: 10 * 1024 * 1024,
            send_type: SendType::Unreliable,
        }]
	}
}