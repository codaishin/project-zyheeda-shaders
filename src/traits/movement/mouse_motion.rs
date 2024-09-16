use super::{Anchor, AnchoredMovement, Seconds};
use bevy::{input::mouse::MouseMotion, prelude::*};

const SENSITIVITY: f32 = 0.5;

impl AnchoredMovement for MouseMotion {
	fn anchored_movement(
		&self,
		agent: &mut Transform,
		Anchor(anchor): Anchor,
		Seconds(delta): Seconds,
	) {
		let distance = (agent.translation - anchor).length();
		agent.rotate_y(-self.delta.x * SENSITIVITY * delta);
		agent.rotate_local_x(-self.delta.y * SENSITIVITY * delta);
		agent.translation = anchor - agent.forward().as_vec3() * distance;
	}
}

#[cfg(test)]
mod test {
	use super::*;
	use crate::assert_approx_eq;
	use std::f32::consts::PI;

	const TOLERANCE_F32: f32 = 0.000001;
	const TOLERANCE: Transform = Transform {
		translation: Vec3::new(TOLERANCE_F32, TOLERANCE_F32, TOLERANCE_F32),
		scale: Vec3::ZERO,
		rotation: Quat::from_array([TOLERANCE_F32; 4]),
	};
	const UP: Vec3 = Vec3::Y;

	fn radians_from_degrees(degrees: f32) -> f32 {
		degrees * PI / 180.
	}

	#[test]
	fn rotate_x_left_90_degrees() {
		let anchor = Vec3::new(0., 0., 0.);
		let mut agent = Transform::from_xyz(1., 0., 0.).looking_at(anchor, UP);
		let event = MouseMotion {
			delta: Vec2 {
				x: radians_from_degrees(180.),
				y: 0.,
			},
		};

		event.anchored_movement(&mut agent, Anchor(anchor), Seconds(1.));

		assert_approx_eq!(
			Transform::from_xyz(0., 0., 1.).looking_at(anchor, UP),
			agent,
			TOLERANCE
		);
	}

	#[test]
	fn rotate_x_left_45_degrees() {
		let anchor = Vec3::new(0., 0., 0.);
		let mut agent = Transform::from_xyz(1., 0., 0.).looking_at(anchor, UP);
		let event = MouseMotion {
			delta: Vec2 {
				x: radians_from_degrees(90.),
				y: 0.,
			},
		};

		event.anchored_movement(&mut agent, Anchor(anchor), Seconds(1.));

		assert_approx_eq!(
			Transform::from_translation(Vec3::new(1., 0., 1.).normalize()).looking_at(anchor, UP),
			agent,
			TOLERANCE
		);
	}

	#[test]
	fn rotate_x_left_90_degrees_with_offset_anchor() {
		let anchor = Vec3::new(0., 1., 0.);
		let mut agent = Transform::from_xyz(1., 0., 0.).looking_at(anchor, UP);
		let event = MouseMotion {
			delta: Vec2 {
				x: radians_from_degrees(180.),
				y: 0.,
			},
		};

		event.anchored_movement(&mut agent, Anchor(anchor), Seconds(1.));

		assert_approx_eq!(
			Transform::from_xyz(0., 0., 1.).looking_at(anchor, UP),
			agent,
			TOLERANCE
		);
	}

	#[test]
	fn rotate_x_left_45_degrees_scaled_by_delta() {
		let anchor = Vec3::new(0., 1., 0.);
		let mut agent = Transform::from_xyz(1., 0., 0.).looking_at(anchor, UP);
		let event = MouseMotion {
			delta: Vec2 {
				x: radians_from_degrees(180.),
				y: 0.,
			},
		};

		event.anchored_movement(&mut agent, Anchor(anchor), Seconds(0.5));

		assert_approx_eq!(
			Transform::from_translation(Vec3::new(1., 0., 1.).normalize()).looking_at(anchor, UP),
			agent,
			TOLERANCE
		);
	}

	#[test]
	fn rotate_y_up_45_degrees() {
		let anchor = Vec3::new(0., 0., 0.);
		let mut agent = Transform::from_xyz(1., 0., 0.).looking_at(anchor, UP);
		let event = MouseMotion {
			delta: Vec2 {
				x: 0.,
				y: radians_from_degrees(90.),
			},
		};

		event.anchored_movement(&mut agent, Anchor(anchor), Seconds(1.));

		assert_approx_eq!(
			Transform::from_translation(Vec3::new(1., 1., 0.).normalize()).looking_at(anchor, UP),
			agent,
			TOLERANCE
		);
	}

	#[test]
	fn rotate_y_up_45_degrees_scaled_by_delta() {
		let anchor = Vec3::new(0., 0., 0.);
		let mut agent = Transform::from_xyz(1., 0., 0.).looking_at(anchor, UP);
		let event = MouseMotion {
			delta: Vec2 {
				x: 0.,
				y: radians_from_degrees(45.),
			},
		};

		event.anchored_movement(&mut agent, Anchor(anchor), Seconds(2.));

		assert_approx_eq!(
			Transform::from_translation(Vec3::new(1., 1., 0.).normalize()).looking_at(anchor, UP),
			agent,
			TOLERANCE
		);
	}
}
