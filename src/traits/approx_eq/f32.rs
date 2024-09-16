use super::ApproxEq;

impl ApproxEq for f32 {
	fn approx_eq(self, other: Self, delta: Self) -> bool {
		if delta.is_sign_negative() {
			return false;
		}

		(self - other).abs() <= delta
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn not_approx_equal() {
		assert!(!(5.).approx_eq(4., 0.9));
	}

	#[test]
	fn approx_equal_1() {
		assert!((5.).approx_eq(4., 1.));
	}

	#[test]
	fn approx_equal_2() {
		assert!((5.).approx_eq(4., 1.1));
	}

	#[test]
	fn not_approx_equal_reversed() {
		assert!(!(4.).approx_eq(5., 0.9));
	}

	#[test]
	fn approx_equal_1_reversed() {
		assert!((4.).approx_eq(5., 1.));
	}

	#[test]
	fn approx_equal_2_reversed() {
		assert!((4.).approx_eq(5., 1.1));
	}

	#[test]
	fn neg_delta_always_false() {
		assert!(!(4.).approx_eq(5., -1.));
	}
}
