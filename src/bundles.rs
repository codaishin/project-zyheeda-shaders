use crate::components::ReplacementMaterial;
use bevy::prelude::*;

#[derive(Bundle, Default)]
pub struct MaterialAssetBundle {
	pub asset: Handle<Scene>,
	pub material: ReplacementMaterial,
	pub transform: Transform,
	pub global_transform: GlobalTransform,
	pub visibility: Visibility,
	pub inherited_visibility: InheritedVisibility,
	pub view_visibility: ViewVisibility,
}
