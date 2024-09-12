use bevy::input::mouse::MouseWheel;
use std::ops::Deref;

pub struct ZoomChange(f32);

pub struct NoChange;

enum Direction {
	Unchanged,
	AwayFromCam,
	TowardsCam,
}

impl ZoomChange {
	pub fn scaled_by(self, scale: f32) -> Self {
		let Self(change) = self;
		Self(change * scale)
	}

	fn direction(MouseWheel { y, .. }: &MouseWheel) -> Direction {
		match y {
			y if y > &0. => Direction::AwayFromCam,
			y if y < &0. => Direction::TowardsCam,
			_ => Direction::Unchanged,
		}
	}
}

impl Deref for ZoomChange {
	type Target = f32;

	fn deref(&self) -> &Self::Target {
		let Self(change) = self;
		change
	}
}

impl TryFrom<&MouseWheel> for ZoomChange {
	type Error = NoChange;

	fn try_from(mouse_wheel: &MouseWheel) -> Result<Self, NoChange> {
		match ZoomChange::direction(mouse_wheel) {
			Direction::TowardsCam => Ok(ZoomChange(1.)),
			Direction::AwayFromCam => Ok(ZoomChange(-1.)),
			Direction::Unchanged => Err(NoChange),
		}
	}
}
