use crate::traits::update_time::UpdateTime;
use bevy::prelude::*;

pub fn set_material_time<TMaterial>(
	time: Res<Time<Real>>,
	materials: Query<&MeshMaterial3d<TMaterial>>,
	mut custom_materials: ResMut<Assets<TMaterial>>,
) where
	TMaterial: Material + UpdateTime,
{
	for handle in &materials {
		let Some(material) = custom_materials.get_mut(handle) else {
			continue;
		};
		material.update_time(time.elapsed());
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::tools::test_tools::tick_time;
	use bevy::{
		ecs::system::{RunSystemError, RunSystemOnce},
		render::render_resource::AsBindGroup,
	};
	use std::time::Duration;

	#[derive(Asset, TypePath, Clone, AsBindGroup, Default)]
	struct _Material {
		time: Duration,
	}

	impl Material for _Material {}

	impl UpdateTime for _Material {
		fn update_time(&mut self, time: Duration) {
			self.time = time;
		}
	}

	fn setup() -> App {
		let mut app = App::new();
		app.init_resource::<Assets<_Material>>();
		app.init_resource::<Time<Real>>();

		tick_time(&mut app, Duration::ZERO);
		app
	}

	#[test]
	fn set_elapsed_time() -> Result<(), RunSystemError> {
		let mut app = setup();
		let mut materials = app.world_mut().resource_mut::<Assets<_Material>>();
		let material = MeshMaterial3d(materials.add(_Material::default()));
		app.world_mut().spawn(material.clone());

		tick_time(&mut app, Duration::from_secs(1));
		tick_time(&mut app, Duration::from_secs(2));
		tick_time(&mut app, Duration::from_secs(3));
		app.world_mut()
			.run_system_once(set_material_time::<_Material>)?;

		let materials = app.world_mut().resource::<Assets<_Material>>();
		let material = materials.get(material.id()).unwrap();
		assert_eq!(Duration::from_secs(6), material.time);
		Ok(())
	}
}
