use bevy::render::view::RenderLayers;

#[derive(Debug, PartialEq, Clone)]
pub enum MaterialConfig {
	OverrideStandardMaterial,
	OverlayStandardMaterial,
	UseOnClonedChild { render_layers: Option<RenderLayers> },
}

pub trait GetMaterialConfig {
	fn material_config() -> MaterialConfig;
}
