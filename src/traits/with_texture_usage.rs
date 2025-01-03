use bevy::{prelude::*, render::render_resource::TextureUsages};

pub trait WithTextureUsage {
	fn with_texture_usage(self, texture_usages: TextureUsages) -> Self;
}

impl WithTextureUsage for Image {
	fn with_texture_usage(mut self, texture_usages: TextureUsages) -> Self {
		self.texture_descriptor.usage = texture_usages;
		self
	}
}
