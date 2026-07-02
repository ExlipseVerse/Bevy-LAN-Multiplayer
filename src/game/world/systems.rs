use bevy::prelude::*;
use avian3d::prelude::*;

pub fn setup_gravity(mut commands: Commands) {
	commands.insert_resource(Gravity(vec3(0.0, -20.0, 0.0)));
}