use crate::{
	components::camera_label::{CameraLabel, SecondPass},
	traits::{
		override_standard_material::{GetMaterialConfig, MaterialConfig},
		refresh::Refresh,
		update_time::UpdateTime,
	},
};
use bevy::{
	prelude::*,
	render::render_resource::{AsBindGroup, ShaderRef},
};
use std::time::Duration;

#[derive(Asset, TypePath, AsBindGroup, Clone, Default)]
pub struct CustomMaterial {
	#[uniform(0)]
	pub color: LinearRgba,
	#[uniform(1)]
	pub time_secs: f32,
	#[texture(2)]
	#[sampler(3)]
	pub color_texture: Option<Handle<Image>>,
	pub alpha_mode: AlphaMode,
}

impl Material for CustomMaterial {
	fn vertex_shader() -> ShaderRef {
		"shaders/custom_material.wgsl".into()
	}

	fn fragment_shader() -> ShaderRef {
		"shaders/custom_material.wgsl".into()
	}

	fn alpha_mode(&self) -> AlphaMode {
		self.alpha_mode
	}
}

impl GetMaterialConfig for CustomMaterial {
	fn material_config() -> MaterialConfig {
		const { MaterialConfig::OverrideStandardMaterial }
	}
}

impl UpdateTime for CustomMaterial {
	fn update_time(&mut self, time: Duration) {
		self.time_secs = time.as_secs_f32();
	}
}

#[derive(Asset, TypePath, AsBindGroup, Clone, Default)]
pub struct DistortionMaterial {
	#[uniform(1)]
	pub time_secs: f32,
	#[texture(2)]
	#[sampler(3)]
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

impl GetMaterialConfig for DistortionMaterial {
	fn material_config() -> MaterialConfig {
		MaterialConfig::UseOnClonedChild {
			render_layers: Some(CameraLabel::<SecondPass>::render_layers()),
		}
	}
}

impl UpdateTime for DistortionMaterial {
	fn update_time(&mut self, time: Duration) {
		self.time_secs = time.as_secs_f32();
	}
}
