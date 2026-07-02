use bevy::prelude::*;
use bevy_renet::RenetClient;
use crate::game::network::{channels::ClientChannel, messages::ClientMessage};
use crate::game::player::components::LocalPlayer;

pub fn send_position_update(
    time: Res<Time>,
    mut timer: Local<Option<Timer>>,
    mut client: ResMut<RenetClient>,
    query: Query<&Transform, With<LocalPlayer>>,
) {
    let timer = timer.get_or_insert_with(|| Timer::from_seconds(1.0 / 20.0, TimerMode::Repeating));
    timer.tick(time.delta());
    if !timer.just_finished() {
        return;
    }
    
    let Ok(transform) = query.single() else { return; };
    let msg = ClientMessage::PlayerUpdate { position: transform.translation };
    client.send_message(ClientChannel::PlayerUpdate as u8, bincode::serialize(&msg).unwrap());
}