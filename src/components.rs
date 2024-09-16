use crate::material::CustomMaterial;
use bevy::prelude::*;

#[derive(Component, Default)]
pub struct ReplacementMaterial(pub Handle<CustomMaterial>);
