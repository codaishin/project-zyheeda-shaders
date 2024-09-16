mod f32;
mod quat;
mod transform;
mod vec3;

pub trait ApproxEq {
	fn approx_eq(self, other: Self, delta: Self) -> bool;
}
