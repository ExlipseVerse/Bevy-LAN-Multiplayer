use bevy::prelude::*;
use avian3d::prelude::*;

pub fn spawn_world(
	mut cmds: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<StandardMaterial>>,
) {

	cmds.spawn((
		DirectionalLight::default(),
		Transform::from_rotation(Quat::from_euler(EulerRot::XYZ, -1.0, -1.0, 0.0))
	));

	cmds.spawn((
		RigidBody::Static,
		Collider::cuboid(20.0, 0.1, 20.0),
		Mesh3d(meshes.add(Plane3d::default().mesh().size(20.0, 20.0))),
		MeshMaterial3d(materials.add(Color::srgb(0.5,0.5,0.5))),
        Transform::default()
	));

}