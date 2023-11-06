use std::ops::Sub;
use bevy::math::Vec3Swizzles;
use bevy::prelude::*;
use bevy::sprite::collide_aabb::collide;
use crate::game::buffers::PosBuffer;
use crate::game::components::{Collider, Health};
use crate::game::player::LocalPlayer;
use crate::net;

const COLLISION_SHOVE_DIST: f32 = 4.0;

pub fn lerp_pos(
    tick_time: Res<FixedTime>,
    tick: Res<net::TickNum>,
    mut query: Query<(&mut Transform, &PosBuffer), Without<LocalPlayer>>,
) {
    for (mut tf, bp) in &mut query {
        let next_state = bp.0.get(tick.0.wrapping_sub(net::DELAY));
        let prev_state = bp.0.get(tick.0.wrapping_sub(net::DELAY + 1));
        let percent: f32 = tick_time.accumulated().as_secs_f32() / tick_time.period.as_secs_f32();
        let new_state = prev_state.lerp(*next_state, percent);
        tf.translation.x = new_state.x;
        tf.translation.y = new_state.y;
    }
}

/// Runs on fixedupdate schedule after other movement-related operations and information received over network.
/// We check to see if things are colliding and if they are we stop them from doing so.
pub fn resolve_collisions(
    tick: Res<net::TickNum>,
    mut colliders: Query<(&mut PosBuffer, &Health, &Collider)>,
) {
    let mut iter = colliders.iter_combinations_mut();
    while let Some([(mut pb1, hp1, collider1), (mut pb2, hp2, collider2)]) = iter.fetch_next() {
        if hp1.current <= 0 || hp2.current <= 0 { continue }
        let a_pos = pb1.0.get(tick.0);
        let a_pos = Vec3::new(a_pos.x, a_pos.y, 0.0);
        let b_pos = pb2.0.get(tick.0);
        let b_pos = Vec3::new(b_pos.x, b_pos.y, 0.0);
        if collide(a_pos, collider1.0, b_pos, collider2.0).is_some() {
            let new_a_pos = a_pos + (a_pos.sub(b_pos)).clamp_length_max(COLLISION_SHOVE_DIST);
            let new_b_pos = b_pos + (b_pos.sub(a_pos)).clamp_length_max(COLLISION_SHOVE_DIST);
            pb1.0.set(tick.0, new_a_pos.xy());
            pb2.0.set(tick.0, new_b_pos.xy());
        }
    }
}