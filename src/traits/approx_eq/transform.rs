use super::ApproxEq;
use bevy::prelude::Transform;

impl ApproxEq for Transform {
	fn approx_eq(self, other: Self, delta: Self) -> bool {
		self.translation
			.approx_eq(other.translation, delta.translation)
			&& self.scale.approx_eq(other.scale, delta.scale)
			&& self.rotation.approx_eq(other.rotation, delta.rotation)
	}
}
