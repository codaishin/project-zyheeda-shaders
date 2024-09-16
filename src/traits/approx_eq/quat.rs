use super::ApproxEq;
use bevy::math::Quat;

impl ApproxEq for Quat {
	fn approx_eq(self, other: Self, delta: Self) -> bool {
		self.to_array()
			.into_iter()
			.zip(other.to_array())
			.zip(delta.to_array())
			.all(|((s, o), d)| s.approx_eq(o, d))
	}
}
