use crate::{
	components::ReplacementMaterial,
	material::{WiggleFast, WiggleSlow},
};
use bevy::prelude::*;

#[derive(Bundle, Default)]
pub struct MaterialAssetBundle {
	pub asset: Handle<Scene>,
	pub material_slow: ReplacementMaterial<WiggleSlow>,
	pub material_fast: ReplacementMaterial<WiggleFast>,
	pub transform: Transform,
	pub global_transform: GlobalTransform,
	pub visibility: Visibility,
	pub inherited_visibility: InheritedVisibility,
	pub view_visibility: ViewVisibility,
}
