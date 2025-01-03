use bevy::{
	asset::RenderAssetUsages,
	prelude::*,
	render::{
		camera::RenderTarget,
		render_resource::{Extent3d, TextureDimension, TextureFormat, TextureUsages},
	},
};

pub fn create_render_image(mut images: ResMut<Assets<Image>>) -> RenderTarget {
	let mut image = Image::new_fill(
		Extent3d {
			width: 1024,
			height: 1024,
			depth_or_array_layers: 1,
		},
		TextureDimension::D2,
		&[0, 0, 0, 255],
		TextureFormat::Bgra8UnormSrgb,
		RenderAssetUsages::default(),
	);
	image.texture_descriptor.usage =
		TextureUsages::TEXTURE_BINDING | TextureUsages::RENDER_ATTACHMENT | TextureUsages::COPY_SRC;

	RenderTarget::Image(images.add(image))
}
