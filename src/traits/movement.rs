use bevy::{math::Vec3, prelude::*};

pub struct Seconds(pub f32);
pub struct Anchor(pub Vec3);

pub trait AnchoredMovement {
	fn anchored_movement(&self, agent: &mut Transform, around: Anchor, delta: Seconds);
}
