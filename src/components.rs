pub mod toggle_visibility;

use crate::material::CustomMaterial;
use bevy::prelude::*;

#[derive(Component, Default)]
#[require(SceneRoot, Transform, Visibility)]
pub struct ReplacementMaterial(pub Handle<CustomMaterial>);
