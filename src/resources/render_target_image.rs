use super::window_size::WindowSize;
use crate::traits::with_texture_usage::WithTextureUsage;
use bevy::{
	asset::RenderAssetUsages,
	prelude::*,
	render::render_resource::{Extent3d, TextureDimension, TextureFormat, TextureUsages},
};

#[derive(Resource, Default, Debug, PartialEq, Clone)]
pub struct RenderTargetImage {
	pub image: Handle<Image>,
}

impl RenderTargetImage {
	pub fn initialize(mut images: ResMut<Assets<Image>>) -> Self {
		let image = Image::new_fill(
			Extent3d::default(),
			TextureDimension::D2,
			&[0, 0, 0, 255],
			TextureFormat::Bgra8UnormSrgb,
			RenderAssetUsages::default(),
		)
		.with_texture_usage(
			TextureUsages::TEXTURE_BINDING
				| TextureUsages::RENDER_ATTACHMENT
				| TextureUsages::COPY_SRC,
		);
		let image = images.add(image);

		Self { image }
	}

	pub fn update(
		mut images: ResMut<Assets<Image>>,
		render_target: Res<RenderTargetImage>,
		window_size: Res<WindowSize>,
	) {
		if !window_size.is_changed() {
			return;
		}

		let Some(image) = images.get_mut(render_target.image.id()) else {
			return;
		};
		let width = window_size.width() as u32;
		let height = window_size.height() as u32;

		if width == 0 || height == 0 {
			return;
		}

		image.resize(Extent3d {
			width,
			height,
			depth_or_array_layers: 1,
		});
	}
}
