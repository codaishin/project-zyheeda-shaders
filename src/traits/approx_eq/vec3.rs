use super::ApproxEq;
use bevy::math::Vec3;

impl ApproxEq for Vec3 {
	fn approx_eq(self, other: Self, delta: Self) -> bool {
		self.x.approx_eq(other.x, delta.x)
			&& self.y.approx_eq(other.y, delta.y)
			&& self.z.approx_eq(other.z, delta.z)
	}
}
