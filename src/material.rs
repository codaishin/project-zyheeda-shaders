use crate::traits::{
	get_material_render_pass::{GetMaterialRenderPass, RenderPass},
	refresh::Refresh,
};
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

#[derive(TypePath, Clone, Default)]
pub struct WiggleFaster;

#[derive(Asset, TypePath, AsBindGroup, Clone, Default)]
pub struct CustomMaterial<T: TypePath + Sync + Send + 'static> {
	#[uniform(0)]
	pub color: LinearRgba,
	#[texture(1)]
	#[sampler(2)]
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

impl Material for CustomMaterial<WiggleFaster> {
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
		descriptor.vertex.entry_point = "vertex_faster".into();
		Ok(())
	}
}

impl<T> GetMaterialRenderPass for CustomMaterial<T>
where
	T: TypePath + Sync + Send + 'static,
{
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
