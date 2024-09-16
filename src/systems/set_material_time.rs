use crate::material::CustomMaterial;
use bevy::prelude::*;

pub fn set_material_time(
	time: Res<Time<Real>>,
	materials: Query<&Handle<CustomMaterial>>,
	mut custom_materials: ResMut<Assets<CustomMaterial>>,
) {
	for handle in &materials {
		let Some(material) = custom_materials.get_mut(handle) else {
			continue;
		};
		material.time_secs = time.elapsed_seconds();
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::{material::CustomMaterial, tools::test_tools::tick_time};
	use bevy::{
		app::App,
		asset::Assets,
		ecs::system::RunSystemOnce,
		time::{Real, Time},
	};
	use std::time::Duration;

	fn setup() -> App {
		let mut app = App::new();
		app.init_resource::<Assets<CustomMaterial>>();
		app.init_resource::<Time<Real>>();

		tick_time(&mut app, Duration::ZERO);
		app
	}

	#[test]
	fn set_elapsed_time() {
		let mut app = setup();
		let mut materials = app.world_mut().resource_mut::<Assets<CustomMaterial>>();
		let material = materials.add(CustomMaterial::default());
		app.world_mut().spawn(material.clone());

		tick_time(&mut app, Duration::from_secs(1));
		tick_time(&mut app, Duration::from_secs(2));
		tick_time(&mut app, Duration::from_secs(3));
		app.world_mut().run_system_once(set_material_time);

		let materials = app.world_mut().resource::<Assets<CustomMaterial>>();
		let material = materials.get(material.id()).unwrap();
		assert_eq!(6., material.time_secs);
	}
}
