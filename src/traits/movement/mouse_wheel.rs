use super::{Anchor, AnchoredMovement, Seconds};
use crate::resources::CameraZoomSettings;
use bevy::{input::mouse::MouseWheel, prelude::*};

impl AnchoredMovement for MouseWheel {
	type TExtra = CameraZoomSettings;

	fn anchored_movement(
		&self,
		agent: &mut Transform,
		Anchor(target): Anchor,
		Seconds(delta): Seconds,
		settings: CameraZoomSettings,
	) {
		let Some(zoom) = get_zoom(self) else {
			return;
		};

		let direction = agent.translation - target;
		let distance = direction.length();
		let zoom = zoom * distance * settings.sensitivity * delta;
		let zoomed_distance = f32::max(distance - zoom, settings.minimal_distance);

		agent.translation = target + direction.normalize() * zoomed_distance;
	}
}

fn get_zoom(wheel: &MouseWheel) -> Option<f32> {
	match wheel.y {
		y if y < 0. => Some(-1.),
		y if y > 0. => Some(1.),
		_ => None,
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use bevy::input::mouse::MouseScrollUnit;

	enum Zoom {
		None,
		In(u8),
		Out(u8),
	}

	fn mouse_wheel(zoom: Zoom) -> MouseWheel {
		MouseWheel {
			unit: MouseScrollUnit::Pixel,
			x: 0.,
			y: match zoom {
				Zoom::Out(distance) => -(distance as f32),
				Zoom::None => 0.,
				Zoom::In(distance) => distance as f32,
			},
			window: Entity::from_raw(42),
		}
	}

	#[test]
	fn zoom_out_x_2_units() {
		let mut agent = Transform::from_xyz(1., 0., 0.);
		let event = mouse_wheel(Zoom::Out(1));

		event.anchored_movement(
			&mut agent,
			Anchor(Vec3::new(0., 0., 0.)),
			Seconds(1.),
			CameraZoomSettings {
				sensitivity: 2.,
				..default()
			},
		);

		assert_eq!(Transform::from_xyz(3., 0., 0.), agent);
	}

	#[test]
	fn zoom_out_x_5_units() {
		let mut agent = Transform::from_xyz(1., 0., 0.);
		let event = mouse_wheel(Zoom::Out(1));

		event.anchored_movement(
			&mut agent,
			Anchor(Vec3::new(0., 0., 0.)),
			Seconds(1.),
			CameraZoomSettings {
				sensitivity: 5.,
				..default()
			},
		);

		assert_eq!(Transform::from_xyz(6., 0., 0.), agent);
	}

	#[test]
	fn zoom_out_y_2_units() {
		let mut agent = Transform::from_xyz(0., 1., 0.);
		let event = mouse_wheel(Zoom::Out(1));

		event.anchored_movement(
			&mut agent,
			Anchor(Vec3::new(0., 0., 0.)),
			Seconds(1.),
			CameraZoomSettings {
				sensitivity: 2.,
				..default()
			},
		);

		assert_eq!(
			Transform::from_translation(
				Vec3::new(0., 1., 0.) + Vec3::new(0., 1., 0.).normalize() * 2.
			),
			agent
		);
	}

	#[test]
	fn zoom_out_y_5_units() {
		let mut agent = Transform::from_xyz(0., 1., 0.);
		let event = mouse_wheel(Zoom::Out(1));

		event.anchored_movement(
			&mut agent,
			Anchor(Vec3::new(0., 0., 0.)),
			Seconds(1.),
			CameraZoomSettings {
				sensitivity: 5.,
				..default()
			},
		);

		assert_eq!(
			Transform::from_translation(
				Vec3::new(0., 1., 0.) + Vec3::new(0., 1., 0.).normalize() * 5.
			),
			agent
		);
	}

	#[test]
	fn zoom_out_x_units_with_anchor_offset() {
		let mut agent = Transform::from_xyz(6., 0., 0.);
		let event = mouse_wheel(Zoom::Out(1));

		event.anchored_movement(
			&mut agent,
			Anchor(Vec3::new(2., 0., 0.)),
			Seconds(1.),
			CameraZoomSettings {
				sensitivity: 1.,
				..default()
			},
		);

		assert_eq!(Transform::from_xyz(10., 0., 0.), agent);
	}

	#[test]
	fn zoom_out_scaled_by_delta() {
		let mut agent = Transform::from_xyz(4., 0., 0.);
		let event = mouse_wheel(Zoom::Out(1));

		event.anchored_movement(
			&mut agent,
			Anchor(Vec3::new(0., 0., 0.)),
			Seconds(0.5),
			CameraZoomSettings {
				sensitivity: 1.,
				..default()
			},
		);

		assert_eq!(Transform::from_xyz(6., 0., 0.), agent);
	}

	#[test]
	fn zoom_out_scaled_by_current_distance() {
		let mut agent = Transform::from_xyz(5., 0., 0.);
		let event = mouse_wheel(Zoom::Out(1));

		event.anchored_movement(
			&mut agent,
			Anchor(Vec3::new(0., 0., 0.)),
			Seconds(1.),
			CameraZoomSettings {
				sensitivity: 1.,
				..default()
			},
		);

		assert_eq!(Transform::from_xyz(10., 0., 0.), agent);
	}

	#[test]
	fn zoom_in_scaled_by_current_distance_and_delta() {
		let mut agent = Transform::from_xyz(10., 0., 0.);
		let event = mouse_wheel(Zoom::In(1));

		event.anchored_movement(
			&mut agent,
			Anchor(Vec3::new(0., 0., 0.)),
			Seconds(1.),
			CameraZoomSettings {
				sensitivity: 0.1,
				..default()
			},
		);

		assert_eq!(Transform::from_xyz(9., 0., 0.), agent);
	}

	#[test]
	fn zoom_in_is_limited() {
		let mut agent = Transform::from_xyz(10., 0., 0.);
		let event = mouse_wheel(Zoom::In(1));

		event.anchored_movement(
			&mut agent,
			Anchor(Vec3::new(0., 0., 0.)),
			Seconds(1.),
			CameraZoomSettings {
				sensitivity: 1.,
				minimal_distance: 4.,
			},
		);

		assert_eq!(Transform::from_xyz(4., 0., 0.), agent);
	}

	#[test]
	fn zoom_out_independent_from_zoom_amount() {
		let mut agent = Transform::from_xyz(5., 0., 0.);
		let event = mouse_wheel(Zoom::Out(2));

		event.anchored_movement(
			&mut agent,
			Anchor(Vec3::new(0., 0., 0.)),
			Seconds(1.),
			CameraZoomSettings {
				sensitivity: 1.,
				..default()
			},
		);

		assert_eq!(Transform::from_xyz(10., 0., 0.), agent);
	}

	#[test]
	fn zoom_in_independent_from_zoom_amount() {
		let mut agent = Transform::from_xyz(10., 0., 0.);
		let event = mouse_wheel(Zoom::In(2));

		event.anchored_movement(
			&mut agent,
			Anchor(Vec3::new(0., 0., 0.)),
			Seconds(0.1),
			CameraZoomSettings {
				sensitivity: 1.,
				..default()
			},
		);

		assert_eq!(Transform::from_xyz(9., 0., 0.), agent);
	}

	#[test]
	fn no_zoom() {
		let mut agent = Transform::from_xyz(1., 0., 0.);
		let event = mouse_wheel(Zoom::None);

		event.anchored_movement(
			&mut agent,
			Anchor(Vec3::new(0., 0., 0.)),
			Seconds(1.),
			CameraZoomSettings {
				sensitivity: 1.,
				..default()
			},
		);

		assert_eq!(Transform::from_xyz(1., 0., 0.), agent);
	}
}
