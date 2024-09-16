use super::{Anchor, AnchoredMovement, Seconds};
use bevy::{input::mouse::MouseMotion, prelude::*};

impl AnchoredMovement for MouseMotion {
	fn anchored_movement(&self, agent: &mut Transform, anchor: Anchor, delta: Seconds) {
		todo!()
	}
}
