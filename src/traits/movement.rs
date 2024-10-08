mod mouse_motion;
mod mouse_wheel;

use bevy::{math::Vec3, prelude::*};

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Seconds(pub f32);

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Anchor(pub Vec3);

pub trait AnchoredMovement {
	type TExtra;
	fn anchored_movement(
		&self,
		agent: &mut Transform,
		around: Anchor,
		delta: Seconds,
		extra: Self::TExtra,
	);
}
