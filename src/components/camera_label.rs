use bevy::{
	prelude::*,
	render::{camera::RenderTarget, view::RenderLayers},
};
use std::marker::PhantomData;

#[derive(Component, Debug, PartialEq, Default)]
pub struct CameraLabel<T>(PhantomData<T>);

impl CameraLabel<FirstPass> {
	pub fn camera() -> (Self, Camera) {
		(
			Self::default(),
			Camera {
				order: 0,
				..default()
			},
		)
	}
}

impl CameraLabel<FirstPassTexture> {
	pub fn camera(first_pass: Handle<Image>) -> (Self, Camera) {
		(
			Self::default(),
			Camera {
				order: 0,
				target: RenderTarget::Image(first_pass),
				..default()
			},
		)
	}
}

impl CameraLabel<SecondPass> {
	pub fn render_layers() -> RenderLayers {
		RenderLayers::layer(1)
	}

	pub fn camera() -> (Self, Camera, RenderLayers) {
		(
			Self::default(),
			Camera {
				order: 1,
				..default()
			},
			Self::render_layers(),
		)
	}

	#[allow(clippy::type_complexity)]
	pub fn activity_based_on_visibility(
		mut cameras: Query<(&mut Camera, &Visibility), (With<Self>, Changed<Visibility>)>,
	) {
		for (mut camera, visibility) in &mut cameras {
			match visibility {
				Visibility::Hidden => {
					camera.is_active = false;
				}
				Visibility::Visible | Visibility::Inherited => {
					camera.is_active = true;
				}
			}
		}
	}
}

impl CameraLabel<Ui> {
	pub fn render_layers() -> RenderLayers {
		RenderLayers::layer(2)
	}

	pub fn camera() -> (Self, Camera, RenderLayers) {
		(
			Self::default(),
			Camera {
				order: 2,
				..default()
			},
			Self::render_layers(),
		)
	}
}

#[derive(Debug, PartialEq, Default)]
pub struct FirstPass;

#[derive(Debug, PartialEq, Default)]
pub struct FirstPassTexture;

#[derive(Debug, PartialEq, Default)]
pub struct SecondPass;

#[derive(Debug, PartialEq, Default)]
pub struct Ui;
