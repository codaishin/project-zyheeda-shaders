pub mod render_target_image;
pub mod window_size;

use bevy::prelude::*;

#[derive(Resource, Clone, Copy)]
pub struct CameraZoomSettings {
	pub sensitivity: f32,
	pub minimal_distance: f32,
}

impl Default for CameraZoomSettings {
	fn default() -> Self {
		Self {
			sensitivity: 10.,
			minimal_distance: 3.,
		}
	}
}

#[derive(Resource, Clone, Copy)]
pub struct CameraRotationSettings {
	pub sensitivity: f32,
}

impl Default for CameraRotationSettings {
	fn default() -> Self {
		Self { sensitivity: 0.5 }
	}
}
