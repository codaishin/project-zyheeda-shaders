#[cfg(test)]
pub mod test_tools {
	use bevy::prelude::*;
	use std::time::Duration;

	#[macro_export]
	macro_rules! assert_approx_eq {
	($a:expr, $b:expr, $d:expr) => {
		assert!(
			$crate::traits::approx_eq::ApproxEq::approx_eq($a, $b, $d),
			"Equal approximation assert failed\n     left: {:?}\n    right: {:?}\ntolerance: {:?}",
			$a,
			$b,
			$d
		)
	};
}
	pub use assert_approx_eq;

	pub fn tick_time(app: &mut App, delta: Duration) {
		let mut time = app.world_mut().resource_mut::<Time<Real>>();
		time.advance_by(delta);
	}
}
