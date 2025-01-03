use crate::traits::override_standard_material::{GetMaterialConfig, MaterialConfig};
use bevy::{prelude::*, render::view::RenderLayers};

#[derive(Component, Default)]
#[require(SceneRoot, Transform, Visibility)]
pub struct ApplyMaterial<TMaterial>(pub Handle<TMaterial>)
where
	TMaterial: Material;

impl<TMaterial> ApplyMaterial<TMaterial>
where
	TMaterial: Material + GetMaterialConfig,
{
	pub fn system(
		mut commands: Commands,
		replacements: Query<&Self>,
		meshes: Query<(Entity, &Mesh3d), With<MeshMaterial3d<StandardMaterial>>>,
		parents: Query<&Parent>,
	) {
		let get_replacement = |entity| replacements.get(entity).ok();
		let find_replacement = |entity| parents.iter_ancestors(entity).find_map(get_replacement);

		for (entity, mesh) in &meshes {
			let Some(ApplyMaterial(handle)) = find_replacement(entity) else {
				continue;
			};
			let Some(mut entity) = commands.get_entity(entity) else {
				continue;
			};

			match TMaterial::material_config() {
				MaterialConfig::OverrideStandardMaterial => {
					entity.insert(MeshMaterial3d(handle.clone()));
					entity.remove::<MeshMaterial3d<StandardMaterial>>();
				}
				MaterialConfig::OverlayStandardMaterial => {
					entity.insert(MeshMaterial3d(handle.clone()));
				}
				MaterialConfig::UseOnClonedChild { render_layers } => {
					instantiate_as_child(entity, render_layers, handle, mesh);
				}
			}
		}
	}
}

fn instantiate_as_child<TMaterial>(
	mut entity: EntityCommands,
	render_layers: Option<RenderLayers>,
	material: &Handle<TMaterial>,
	mesh: &Mesh3d,
) where
	TMaterial: Material,
{
	entity.with_children(move |entity| {
		let mut child = entity.spawn((MeshMaterial3d(material.clone()), mesh.clone()));
		let Some(render_layers) = render_layers else {
			return;
		};
		child.insert(render_layers.clone());
	});
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::traits::override_standard_material::MaterialConfig;
	use bevy::{
		ecs::system::{RunSystemError, RunSystemOnce},
		render::{render_resource::AsBindGroup, view::RenderLayers},
	};
	use uuid::Uuid;

	#[derive(Asset, TypePath, AsBindGroup, Clone, Default)]
	struct _Material {}

	impl Material for _Material {}

	impl GetMaterialConfig for _Material {
		fn material_config() -> MaterialConfig {
			MaterialConfig::OverlayStandardMaterial
		}
	}

	#[derive(Asset, TypePath, AsBindGroup, Clone, Default)]
	struct _ReplaceMaterial {}

	impl Material for _ReplaceMaterial {}

	impl GetMaterialConfig for _ReplaceMaterial {
		fn material_config() -> MaterialConfig {
			MaterialConfig::OverrideStandardMaterial
		}
	}

	#[derive(Asset, TypePath, AsBindGroup, Clone, Default)]
	struct _SeparateMaterial {}

	impl Material for _SeparateMaterial {}

	impl GetMaterialConfig for _SeparateMaterial {
		fn material_config() -> MaterialConfig {
			MaterialConfig::UseOnClonedChild {
				render_layers: Some(RenderLayers::layer(11)),
			}
		}
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
		let child = app
			.world_mut()
			.spawn((material, Mesh3d(new_handle::<Mesh>())))
			.set_parent(parent)
			.id();

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
		let material = app
			.world_mut()
			.spawn((material, Mesh3d(new_handle::<Mesh>())))
			.id();

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
		let child_child = app
			.world_mut()
			.spawn((material, Mesh3d(new_handle::<Mesh>())))
			.set_parent(child)
			.id();

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
		let child = app
			.world_mut()
			.spawn((material, Mesh3d(new_handle::<Mesh>())))
			.set_parent(parent)
			.id();

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
		let child = app
			.world_mut()
			.spawn((material, Mesh3d(new_handle::<Mesh>())))
			.set_parent(parent)
			.id();

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

	#[test]
	fn set_material_of_nth_parent_when_overriding_standard_material() -> Result<(), RunSystemError>
	{
		let mut app = setup();
		let material = MeshMaterial3d(new_handle::<StandardMaterial>());
		let replacement = new_handle::<_ReplaceMaterial>();
		let parent = app
			.world_mut()
			.spawn(ApplyMaterial(replacement.clone()))
			.id();
		let child = app.world_mut().spawn_empty().set_parent(parent).id();
		let child_child = app
			.world_mut()
			.spawn((material, Mesh3d(new_handle::<Mesh>())))
			.set_parent(child)
			.id();

		app.world_mut()
			.run_system_once(ApplyMaterial::<_ReplaceMaterial>::system)?;

		assert_eq!(
			Some(&replacement),
			app.world()
				.entity(child_child)
				.get::<MeshMaterial3d<_ReplaceMaterial>>()
				.map(|MeshMaterial3d(handle)| handle)
		);
		Ok(())
	}

	fn children(app: &App, parent: Entity) -> impl Iterator<Item = EntityRef> {
		app.world().iter_entities().filter(move |entity| {
			entity
				.get::<Parent>()
				.map(|p| p.get() == parent)
				.unwrap_or(false)
		})
	}

	macro_rules! unpack {
		($count:expr, $collection:expr) => {{
			let unpack: [_; $count] = $collection
				.collect::<Vec<_>>()
				.try_into()
				.ok()
				.expect("Failed to unpack collection with correct count");

			unpack
		}};
	}

	#[test]
	fn set_material_as_nth_parent_child_with_mesh_clone_when_set_to_clone_mesh(
	) -> Result<(), RunSystemError> {
		let mut app = setup();
		let material = MeshMaterial3d(new_handle::<StandardMaterial>());
		let mesh_handle = new_handle::<Mesh>();
		let mesh = Mesh3d(mesh_handle.clone());
		let separate = new_handle::<_SeparateMaterial>();
		let parent = app.world_mut().spawn(ApplyMaterial(separate.clone())).id();
		let child = app.world_mut().spawn_empty().set_parent(parent).id();
		let child_child = app
			.world_mut()
			.spawn((material, mesh))
			.set_parent(child)
			.id();

		app.world_mut()
			.run_system_once(ApplyMaterial::<_SeparateMaterial>::system)?;

		let [child] = unpack!(1, children(&app, child_child));
		assert_eq!(
			(Some(&separate), Some(&mesh_handle)),
			(
				child
					.get::<MeshMaterial3d<_SeparateMaterial>>()
					.map(|MeshMaterial3d(handle)| handle),
				child.get::<Mesh3d>().map(|Mesh3d(handle)| handle),
			)
		);
		Ok(())
	}

	#[test]
	fn insert_render_layer_on_nth_parent_child_when_set_to_clone_mesh() -> Result<(), RunSystemError>
	{
		let mut app = setup();
		let material = MeshMaterial3d(new_handle::<StandardMaterial>());
		let mesh_handle = new_handle::<Mesh>();
		let mesh = Mesh3d(mesh_handle.clone());
		let separate = new_handle::<_SeparateMaterial>();
		let parent = app.world_mut().spawn(ApplyMaterial(separate.clone())).id();
		let child = app.world_mut().spawn_empty().set_parent(parent).id();
		let child_child = app
			.world_mut()
			.spawn((material, mesh))
			.set_parent(child)
			.id();

		app.world_mut()
			.run_system_once(ApplyMaterial::<_SeparateMaterial>::system)?;

		let [child] = unpack!(1, children(&app, child_child));
		assert_eq!(Some(&RenderLayers::layer(11)), child.get::<RenderLayers>());
		Ok(())
	}

	#[test]
	fn do_not_set_material_on_nth_parent_child_when_set_to_clone_mesh() -> Result<(), RunSystemError>
	{
		let mut app = setup();
		let material = MeshMaterial3d(new_handle::<StandardMaterial>());
		let mesh_handle = new_handle::<Mesh>();
		let mesh = Mesh3d(mesh_handle.clone());
		let separate = new_handle::<_SeparateMaterial>();
		let parent = app.world_mut().spawn(ApplyMaterial(separate.clone())).id();
		let child = app.world_mut().spawn_empty().set_parent(parent).id();
		let child_child = app
			.world_mut()
			.spawn((material, mesh))
			.set_parent(child)
			.id();

		app.world_mut()
			.run_system_once(ApplyMaterial::<_SeparateMaterial>::system)?;

		assert_eq!(
			None,
			app.world()
				.entity(child_child)
				.get::<MeshMaterial3d<_SeparateMaterial>>()
				.map(|MeshMaterial3d(handle)| handle),
		);
		Ok(())
	}
}
