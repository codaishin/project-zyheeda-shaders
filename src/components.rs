use crate::material::CustomMaterial;
use bevy::prelude::*;

#[derive(Component, Default)]
pub struct ReplacementMaterial<T: TypePath + Sync + Send + 'static>(pub Handle<CustomMaterial<T>>);
