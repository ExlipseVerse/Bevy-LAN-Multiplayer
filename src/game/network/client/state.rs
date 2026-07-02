use bevy::prelude::*;

#[derive(Resource)]
pub struct LocalClient {
	pub id: Option<u64>,
}

impl Default for LocalClient {
	fn default() -> Self {
	    Self { id: None }
	}
}