use bevy::prelude::*;
use bevy_renet::RenetClient;
use crate::game::{state::GameState, ui::misc::resource::WaitScreenText};

pub fn client_connection(
    client: Res<RenetClient>,
    mut next_state: ResMut<NextState<GameState>>,
    mut commands: Commands,
) {
    if client.is_connected() {
        next_state.set(GameState::InGame);
        commands.remove_resource::<WaitScreenText>();
    }
}