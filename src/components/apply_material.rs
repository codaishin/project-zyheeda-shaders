use crate::traits::override_standard_material::OverrideStandardMaterial;
use bevy::{prelude::*, render::view::RenderLayers};

#[derive(Component, Default)]
#[require(SceneRoot, Transform, Visibility)]
pub struct ApplyMaterial<TMaterial>(pub Handle<TMaterial>)
where
	TMaterial: Material;

impl<TMaterial> ApplyMaterial<TMaterial>
where
	TMaterial: Material + OverrideStandardMaterial,
{
	pub fn system(
		mut commands: Commands,
		replacements: Query<&Self>,
		materials: Query<Entity, With<MeshMaterial3d<StandardMaterial>>>,
		parents: Query<&Parent>,
	) {
		let get_replacement = |entity| replacements.get(entity).ok();
		let find_replacement = |entity| parents.iter_ancestors(entity).find_map(get_replacement);

		for entity in &materials {
			let Some(ApplyMaterial(handle)) = find_replacement(entity) else {
				continue;
			};
			let Some(mut entity) = commands.get_entity(entity) else {
				continue;
			};

			entity.insert(MeshMaterial3d(handle.clone()));

			if !TMaterial::OVERRIDE_STANDARD_MATERIAL {
				continue;
			}

			entity.remove::<MeshMaterial3d<StandardMaterial>>();
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use bevy::{
		ecs::system::{RunSystemError, RunSystemOnce},
		render::render_resource::AsBindGroup,
	};
	use uuid::Uuid;

	#[derive(Asset, TypePath, AsBindGroup, Clone, Default)]
	struct _Material {}

	impl Material for _Material {}

	impl OverrideStandardMaterial for _Material {
		const OVERRIDE_STANDARD_MATERIAL: bool = false;
	}

	#[derive(Asset, TypePath, AsBindGroup, Clone, Default)]
	struct _ReplaceMaterial {}

	impl Material for _ReplaceMaterial {}

	impl OverrideStandardMaterial for _ReplaceMaterial {
		const OVERRIDE_STANDARD_MATERIAL: bool = true;
	}

	fn setup() -> App {
		App::new()
	}

	fn new_handle<T: Asset>() -> Handle<T> {
		Handle::Weak(AssetId::Uuid {
			uuid: Uuid::new_v4(),
		})
	}

	#[test]
	fn set_material() -> Result<(), RunSystemError> {
		let mut app = setup();
		let material = MeshMaterial3d(new_handle::<StandardMaterial>());
		let replacement = new_handle::<_Material>();
		let parent = app
			.world_mut()
			.spawn(ApplyMaterial(replacement.clone()))
			.id();
		let child = app.world_mut().spawn(material).set_parent(parent).id();

		app.world_mut()
			.run_system_once(ApplyMaterial::<_Material>::system)?;

		assert_eq!(
			Some(&replacement),
			app.world()
				.entity(child)
				.get::<MeshMaterial3d<_Material>>()
				.map(|MeshMaterial3d(handle)| handle)
		);
		Ok(())
	}

	#[test]
	fn do_not_set_material_when_not_parent() -> Result<(), RunSystemError> {
		let mut app = setup();
		let material = MeshMaterial3d(new_handle::<StandardMaterial>());
		let replacement = new_handle::<_Material>();
		app.world_mut().spawn(ApplyMaterial(replacement.clone()));
		let material = app.world_mut().spawn(material).id();

		app.world_mut()
			.run_system_once(ApplyMaterial::<_Material>::system)?;

		assert_eq!(
			None,
			app.world()
				.entity(material)
				.get::<MeshMaterial3d<_Material>>()
				.map(|MeshMaterial3d(handle)| handle)
		);
		Ok(())
	}

	#[test]
	fn set_material_of_nth_parent() -> Result<(), RunSystemError> {
		let mut app = setup();
		let material = MeshMaterial3d(new_handle::<StandardMaterial>());
		let replacement = new_handle::<_Material>();
		let parent = app
			.world_mut()
			.spawn(ApplyMaterial(replacement.clone()))
			.id();
		let child = app.world_mut().spawn_empty().set_parent(parent).id();
		let child_child = app.world_mut().spawn(material).set_parent(child).id();

		app.world_mut()
			.run_system_once(ApplyMaterial::<_Material>::system)?;

		assert_eq!(
			Some(&replacement),
			app.world()
				.entity(child_child)
				.get::<MeshMaterial3d<_Material>>()
				.map(|MeshMaterial3d(handle)| handle)
		);
		Ok(())
	}

	#[test]
	fn remove_standard_material() -> Result<(), RunSystemError> {
		let mut app = setup();
		let material = MeshMaterial3d(new_handle::<StandardMaterial>());
		let replacement = new_handle::<_ReplaceMaterial>();
		let parent = app
			.world_mut()
			.spawn(ApplyMaterial(replacement.clone()))
			.id();
		let child = app.world_mut().spawn(material).set_parent(parent).id();

		app.world_mut()
			.run_system_once(ApplyMaterial::<_ReplaceMaterial>::system)?;

		assert_eq!(
			None,
			app.world()
				.entity(child)
				.get::<MeshMaterial3d<StandardMaterial>>()
				.map(|MeshMaterial3d(handle)| handle)
		);
		Ok(())
	}

	#[test]
	fn do_not_remove_standard_material_when_not_overriding_standard_material(
	) -> Result<(), RunSystemError> {
		let mut app = setup();
		let handle = new_handle::<StandardMaterial>();
		let material = MeshMaterial3d(handle.clone());
		let replacement = new_handle::<_Material>();
		let parent = app
			.world_mut()
			.spawn(ApplyMaterial(replacement.clone()))
			.id();
		let child = app.world_mut().spawn(material).set_parent(parent).id();

		app.world_mut()
			.run_system_once(ApplyMaterial::<_Material>::system)?;

		assert_eq!(
			Some(&handle),
			app.world()
				.entity(child)
				.get::<MeshMaterial3d<StandardMaterial>>()
				.map(|MeshMaterial3d(handle)| handle)
		);
		Ok(())
	}
}
