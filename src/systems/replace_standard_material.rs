use crate::components::ReplacementMaterial;
use bevy::prelude::*;

pub fn replace_standard_material<T: TypePath + Sync + Send + 'static>(
	mut commands: Commands,
	replacements: Query<&ReplacementMaterial<T>>,
	materials: Query<Entity, With<Handle<StandardMaterial>>>,
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

		entity.insert(handle.clone());
		entity.remove::<Handle<StandardMaterial>>();
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::{components::ReplacementMaterial, material::CustomMaterial};
	use bevy::{app::App, ecs::system::RunSystemOnce};
	use uuid::Uuid;

	#[derive(TypePath, Clone, Default)]
	struct _T;

	fn setup() -> App {
		App::new()
	}

	fn new_handle<T: Asset>() -> Handle<T> {
		Handle::Weak(AssetId::Uuid {
			uuid: Uuid::new_v4(),
		})
	}

	#[test]
	fn set_replacement_material() {
		let mut app = setup();
		let material = new_handle::<StandardMaterial>();
		let replacement = new_handle::<CustomMaterial<_T>>();
		let parent = app
			.world_mut()
			.spawn(ReplacementMaterial(replacement.clone()))
			.id();
		let child = app.world_mut().spawn(material).set_parent(parent).id();

		app.world_mut()
			.run_system_once(replace_standard_material::<_T>);

		let child = app.world().entity(child);
		assert_eq!(
			Some(&replacement),
			child.get::<Handle<CustomMaterial<_T>>>()
		)
	}

	#[test]
	fn do_not_set_replacement_material_when_no_standard_material() {
		let mut app = setup();
		let replacement = new_handle::<CustomMaterial<_T>>();
		let parent = app
			.world_mut()
			.spawn(ReplacementMaterial(replacement.clone()))
			.id();
		let child = app.world_mut().spawn_empty().set_parent(parent).id();

		app.world_mut()
			.run_system_once(replace_standard_material::<_T>);

		let child = app.world().entity(child);
		assert_eq!(None, child.get::<Handle<CustomMaterial::<_T>>>())
	}

	#[test]
	fn do_not_set_replacement_material_when_not_parent() {
		let mut app = setup();
		let material = new_handle::<StandardMaterial>();
		let replacement = new_handle::<CustomMaterial<_T>>();
		app.world_mut()
			.spawn(ReplacementMaterial(replacement.clone()));
		let material = app.world_mut().spawn(material).id();

		app.world_mut()
			.run_system_once(replace_standard_material::<_T>);

		let material = app.world().entity(material);
		assert_eq!(None, material.get::<Handle<CustomMaterial<_T>>>())
	}

	#[test]
	fn set_replacement_material_of_nth_parent() {
		let mut app = setup();
		let material = new_handle::<StandardMaterial>();
		let replacement = new_handle::<CustomMaterial<_T>>();
		let parent = app
			.world_mut()
			.spawn(ReplacementMaterial(replacement.clone()))
			.id();
		let child = app.world_mut().spawn_empty().set_parent(parent).id();
		let child_child = app.world_mut().spawn(material).set_parent(child).id();

		app.world_mut()
			.run_system_once(replace_standard_material::<_T>);

		let child_child = app.world().entity(child_child);
		assert_eq!(
			Some(&replacement),
			child_child.get::<Handle<CustomMaterial<_T>>>()
		)
	}

	#[test]
	fn remove_standard_material() {
		let mut app = setup();
		let material = new_handle::<StandardMaterial>();
		let replacement = new_handle::<CustomMaterial<_T>>();
		let parent = app
			.world_mut()
			.spawn(ReplacementMaterial(replacement.clone()))
			.id();
		let child = app.world_mut().spawn(material).set_parent(parent).id();

		app.world_mut()
			.run_system_once(replace_standard_material::<_T>);

		let child = app.world().entity(child);
		assert_eq!(None, child.get::<Handle<StandardMaterial>>())
	}
}
