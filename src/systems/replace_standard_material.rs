use crate::components::ReplacementMaterial;
use bevy::prelude::*;

pub fn replace_standard_material(
	mut commands: Commands,
	replacements: Query<&ReplacementMaterial>,
	materials: Query<Entity, With<MeshMaterial3d<StandardMaterial>>>,
	parents: Query<&Parent>,
) {
	let get_replacement = |entity| replacements.get(entity).ok();
	let find_replacement = |entity| parents.iter_ancestors(entity).find_map(get_replacement);

	for entity in &materials {
		let Some(ReplacementMaterial(handle)) = find_replacement(entity) else {
			continue;
		};
		let Some(mut entity) = commands.get_entity(entity) else {
			continue;
		};

		entity.insert(MeshMaterial3d(handle.clone()));
		entity.remove::<MeshMaterial3d<StandardMaterial>>();
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::{components::ReplacementMaterial, material::CustomMaterial};
	use bevy::ecs::system::{RunSystemError, RunSystemOnce};
	use uuid::Uuid;

	fn setup() -> App {
		App::new()
	}

	fn new_handle<T: Asset>() -> Handle<T> {
		Handle::Weak(AssetId::Uuid {
			uuid: Uuid::new_v4(),
		})
	}

	#[test]
	fn set_replacement_material() -> Result<(), RunSystemError> {
		let mut app = setup();
		let material = MeshMaterial3d(new_handle::<StandardMaterial>());
		let replacement = new_handle::<CustomMaterial>();
		let parent = app
			.world_mut()
			.spawn(ReplacementMaterial(replacement.clone()))
			.id();
		let child = app.world_mut().spawn(material).set_parent(parent).id();

		app.world_mut().run_system_once(replace_standard_material)?;

		assert_eq!(
			Some(&replacement),
			app.world()
				.entity(child)
				.get::<MeshMaterial3d<CustomMaterial>>()
				.map(|MeshMaterial3d(handle)| handle)
		);
		Ok(())
	}

	#[test]
	fn do_not_set_replacement_material_when_no_standard_material() -> Result<(), RunSystemError> {
		let mut app = setup();
		let replacement = new_handle::<CustomMaterial>();
		let parent = app
			.world_mut()
			.spawn(ReplacementMaterial(replacement.clone()))
			.id();
		let child = app.world_mut().spawn_empty().set_parent(parent).id();

		app.world_mut().run_system_once(replace_standard_material)?;

		assert_eq!(
			None,
			app.world()
				.entity(child)
				.get::<MeshMaterial3d<CustomMaterial>>()
				.map(|MeshMaterial3d(handle)| handle)
		);
		Ok(())
	}

	#[test]
	fn do_not_set_replacement_material_when_not_parent() -> Result<(), RunSystemError> {
		let mut app = setup();
		let material = MeshMaterial3d(new_handle::<StandardMaterial>());
		let replacement = new_handle::<CustomMaterial>();
		app.world_mut()
			.spawn(ReplacementMaterial(replacement.clone()));
		let material = app.world_mut().spawn(material).id();

		app.world_mut().run_system_once(replace_standard_material)?;

		assert_eq!(
			None,
			app.world()
				.entity(material)
				.get::<MeshMaterial3d<CustomMaterial>>()
				.map(|MeshMaterial3d(handle)| handle)
		);
		Ok(())
	}

	#[test]
	fn set_replacement_material_of_nth_parent() -> Result<(), RunSystemError> {
		let mut app = setup();
		let material = MeshMaterial3d(new_handle::<StandardMaterial>());
		let replacement = new_handle::<CustomMaterial>();
		let parent = app
			.world_mut()
			.spawn(ReplacementMaterial(replacement.clone()))
			.id();
		let child = app.world_mut().spawn_empty().set_parent(parent).id();
		let child_child = app.world_mut().spawn(material).set_parent(child).id();

		app.world_mut().run_system_once(replace_standard_material)?;

		assert_eq!(
			Some(&replacement),
			app.world()
				.entity(child_child)
				.get::<MeshMaterial3d<CustomMaterial>>()
				.map(|MeshMaterial3d(handle)| handle)
		);
		Ok(())
	}

	#[test]
	fn remove_standard_material() -> Result<(), RunSystemError> {
		let mut app = setup();
		let material = MeshMaterial3d(new_handle::<StandardMaterial>());
		let replacement = new_handle::<CustomMaterial>();
		let parent = app
			.world_mut()
			.spawn(ReplacementMaterial(replacement.clone()))
			.id();
		let child = app.world_mut().spawn(material).set_parent(parent).id();

		app.world_mut().run_system_once(replace_standard_material)?;

		assert_eq!(
			None,
			app.world()
				.entity(child)
				.get::<MeshMaterial3d<StandardMaterial>>()
				.map(|MeshMaterial3d(handle)| handle)
		);
		Ok(())
	}
}
