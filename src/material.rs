use bevy::{
	asset::Asset,
	color::LinearRgba,
	pbr::Material,
	prelude::AlphaMode,
	reflect::TypePath,
	render::render_resource::{AsBindGroup, ShaderRef},
};

#[derive(Asset, TypePath, AsBindGroup, Clone, Default)]
pub struct CustomMaterial {
	#[uniform(0)]
	pub color: LinearRgba,
	#[uniform(1)]
	pub time_secs: f32,
	pub alpha_mode: AlphaMode,
}

impl Material for CustomMaterial {
	fn fragment_shader() -> ShaderRef {
		"shaders/custom_material.wgsl".into()
	}

	fn vertex_shader() -> ShaderRef {
		"shaders/custom_material.wgsl".into()
	}

	fn alpha_mode(&self) -> AlphaMode {
		self.alpha_mode
	}
}
