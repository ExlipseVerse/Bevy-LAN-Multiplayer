use bevy::prelude::*;
use avian3d::prelude::*;

use super::super::components::{LocalPlayer, Grounded};

pub fn ground_check(
    spatial_query: SpatialQuery,
    mut player: Query<(Entity, &Transform, &Collider, &mut Grounded), With<LocalPlayer>>,
) {
    // let (entity, transform, collider, mut grounded) = player.single_mut().unwrap();

    let Ok((entity, transform, collider, mut grounded)) = player.single_mut() else {
        return;
    };
    let filter = SpatialQueryFilter::from_mask(0b1111).with_excluded_entities([entity]);

    let origin = transform.translation;
    let max_distance = 0.15;

    let hit = spatial_query.cast_shape(
                collider,
                origin,
                transform.rotation,
                Dir3::NEG_Y,
                &ShapeCastConfig { max_distance: max_distance, target_distance: 0.0, compute_contact_on_penetration: true, ignore_origin_penetration: false },
                &filter
            );

    grounded.0 = hit.is_some();
}