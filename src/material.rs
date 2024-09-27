use bevy::{
	pbr::{MaterialPipeline, MaterialPipelineKey},
	prelude::*,
	render::{
		mesh::MeshVertexBufferLayoutRef,
		render_resource::{
			AsBindGroup,
			RenderPipelineDescriptor,
			ShaderRef,
			SpecializedMeshPipelineError,
		},
	},
};
use std::marker::PhantomData;

#[derive(TypePath, Clone, Default)]
pub struct WiggleSlow;

#[derive(TypePath, Clone, Default)]
pub struct WiggleFast;

#[derive(Asset, TypePath, AsBindGroup, Clone, Default)]
pub struct CustomMaterial<T: TypePath + Sync + Send + 'static> {
	#[uniform(0)]
	pub color: LinearRgba,
	#[uniform(1)]
	pub time_secs: f32,
	#[texture(2)]
	#[sampler(3)]
	pub color_texture: Option<Handle<Image>>,
	pub alpha_mode: AlphaMode,
	pub phantom_data: PhantomData<T>,
}

impl Material for CustomMaterial<WiggleSlow> {
	fn vertex_shader() -> ShaderRef {
		"shaders/custom_material.wgsl".into()
	}

	fn fragment_shader() -> ShaderRef {
		"shaders/custom_material.wgsl".into()
	}

	fn alpha_mode(&self) -> AlphaMode {
		self.alpha_mode
	}

	fn specialize(
		_: &MaterialPipeline<Self>,
		descriptor: &mut RenderPipelineDescriptor,
		_: &MeshVertexBufferLayoutRef,
		_: MaterialPipelineKey<Self>,
	) -> Result<(), SpecializedMeshPipelineError> {
		descriptor.vertex.entry_point = "vertex_slow".into();
		Ok(())
	}
}

impl Material for CustomMaterial<WiggleFast> {
	fn vertex_shader() -> ShaderRef {
		"shaders/custom_material.wgsl".into()
	}

	fn fragment_shader() -> ShaderRef {
		"shaders/custom_material.wgsl".into()
	}

	fn alpha_mode(&self) -> AlphaMode {
		self.alpha_mode
	}

	fn specialize(
		_: &MaterialPipeline<Self>,
		descriptor: &mut RenderPipelineDescriptor,
		_: &MeshVertexBufferLayoutRef,
		_: MaterialPipelineKey<Self>,
	) -> Result<(), SpecializedMeshPipelineError> {
		descriptor.vertex.entry_point = "vertex_fast".into();
		Ok(())
	}
}
