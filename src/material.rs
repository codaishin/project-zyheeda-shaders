use crate::traits::{
	get_material_render_pass::{GetMaterialRenderPass, RenderPass},
	refresh::Refresh,
};
use bevy::{
	prelude::*,
	render::render_resource::{AsBindGroup, ShaderRef},
};

#[derive(Asset, TypePath, AsBindGroup, Clone, Default)]
pub struct CustomMaterial {
	#[uniform(0)]
	pub material_color: LinearRgba,
	#[texture(1)]
	#[sampler(2)]
	pub color_texture: Option<Handle<Image>>,
	#[uniform(3)]
	pub fill_color: LinearRgba,
	#[uniform(4)]
	pub shine: LinearRgba,
	pub alpha_mode: AlphaMode,
}

impl Material for CustomMaterial {
	fn fragment_shader() -> ShaderRef {
		"shaders/custom_material.wgsl".into()
	}

	fn alpha_mode(&self) -> AlphaMode {
		self.alpha_mode
	}
}

impl GetMaterialRenderPass for CustomMaterial {
	fn render_pass() -> RenderPass {
		const { RenderPass::FirstPass }
	}
}

#[derive(Asset, TypePath, AsBindGroup, Clone, Default)]
pub struct DistortionMaterial {
	#[texture(0)]
	#[sampler(1)]
	pub first_pass: Handle<Image>,
}

impl DistortionMaterial {
	pub fn refresh(
		entities: Query<&MeshMaterial3d<DistortionMaterial>>,
		mut materials: ResMut<Assets<DistortionMaterial>>,
	) {
		for MeshMaterial3d(handle) in &entities {
			materials.refresh(handle.id());
		}
	}
}

impl Material for DistortionMaterial {
	fn fragment_shader() -> ShaderRef {
		"shaders/distortion_material.wgsl".into()
	}
}

impl GetMaterialRenderPass for DistortionMaterial {
	fn render_pass() -> RenderPass {
		const { RenderPass::SecondPass }
	}
}
