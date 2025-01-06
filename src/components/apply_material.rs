use super::camera_label::{CameraLabel, SecondPass};
use crate::traits::get_material_render_pass::{GetMaterialRenderPass, RenderPass};
use bevy::prelude::*;
use std::marker::PhantomData;

#[derive(Component, Default)]
#[require(SceneRoot, Transform, Visibility)]
pub struct ApplyMaterial<TMaterial>(pub Handle<TMaterial>)
where
	TMaterial: Material;

impl<TMaterial> ApplyMaterial<TMaterial>
where
	TMaterial: Material + GetMaterialRenderPass,
{
	#[allow(clippy::type_complexity)]
	pub fn system(
		mut commands: Commands,
		replacements: Query<&Self>,
		meshes: Query<
			(Entity, &Mesh3d, Option<&SecondPassEntity>),
			(Without<Applied<TMaterial>>, Without<IsSecondPassEntity>),
		>,
		parents: Query<&Parent>,
	) {
		let get_replacement = |entity| replacements.get(entity).ok();
		let find_replacement = |entity| parents.iter_ancestors(entity).find_map(get_replacement);

		for (entity, mesh, snd_pass) in &meshes {
			let Some(ApplyMaterial(handle)) = find_replacement(entity) else {
				continue;
			};
			let Some(mut entity) = commands.get_entity(entity) else {
				continue;
			};

			entity.remove::<MeshMaterial3d<StandardMaterial>>();
			entity.insert(Applied::<TMaterial>(PhantomData));

			match TMaterial::render_pass() {
				RenderPass::FirstPass => {
					entity.insert(MeshMaterial3d(handle.clone()));
				}
				RenderPass::SecondPass => {
					insert_on_second_pass_entity(entity, snd_pass, handle, mesh);
				}
			}
		}
	}
}

#[derive(Component, Debug, PartialEq)]
pub struct IsSecondPassEntity;

#[derive(Component, Debug, PartialEq)]
pub struct SecondPassEntity(Entity);

#[derive(Component, Debug, PartialEq)]
pub struct Applied<TMaterial>(PhantomData<TMaterial>);

fn insert_on_second_pass_entity<TMaterial>(
	mut entity: EntityCommands,
	snd_pass: Option<&SecondPassEntity>,
	material: &Handle<TMaterial>,
	mesh: &Mesh3d,
) where
	TMaterial: Material,
{
	match snd_pass {
		Some(SecondPassEntity(snd_pass)) => {
			let Some(mut snd_pass) = entity.commands_mut().get_entity(*snd_pass) else {
				return;
			};
			snd_pass.insert(MeshMaterial3d(material.clone()));
		}
		None => {
			let parent_id = entity.id();
			let snd_pass = entity
				.commands_mut()
				.spawn((
					MeshMaterial3d(material.clone()),
					mesh.clone(),
					CameraLabel::<SecondPass>::render_layers(),
					IsSecondPassEntity,
				))
				.set_parent(parent_id)
				.id();
			entity.insert(SecondPassEntity(snd_pass));
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::{
		components::camera_label::{CameraLabel, SecondPass},
		traits::get_material_render_pass::RenderPass,
	};
	use bevy::{
		ecs::system::{RunSystemError, RunSystemOnce},
		render::{render_resource::AsBindGroup, view::RenderLayers},
	};
	use uuid::Uuid;

	#[derive(Asset, TypePath, AsBindGroup, Clone, Default)]
	struct _Material {}

	impl Material for _Material {}

	impl GetMaterialRenderPass for _Material {
		fn render_pass() -> RenderPass {
			const { RenderPass::FirstPass }
		}
	}

	#[derive(Asset, TypePath, AsBindGroup, Clone, Default)]
	struct _SeparateMaterialA {}

	impl Material for _SeparateMaterialA {}

	impl GetMaterialRenderPass for _SeparateMaterialA {
		fn render_pass() -> RenderPass {
			const { RenderPass::SecondPass }
		}
	}

	#[derive(Asset, TypePath, AsBindGroup, Clone, Default)]
	struct _SeparateMaterialB {}

	impl Material for _SeparateMaterialB {}

	impl GetMaterialRenderPass for _SeparateMaterialB {
		fn render_pass() -> RenderPass {
			const { RenderPass::SecondPass }
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
	fn set_material_as_nth_parent_child_with_mesh_clone_when_set_to_second_pass(
	) -> Result<(), RunSystemError> {
		let mut app = setup();
		let material = MeshMaterial3d(new_handle::<StandardMaterial>());
		let mesh_handle = new_handle::<Mesh>();
		let mesh = Mesh3d(mesh_handle.clone());
		let separate = new_handle::<_SeparateMaterialA>();
		let parent = app.world_mut().spawn(ApplyMaterial(separate.clone())).id();
		let child = app.world_mut().spawn_empty().set_parent(parent).id();
		let child_child = app
			.world_mut()
			.spawn((material, mesh))
			.set_parent(child)
			.id();

		app.world_mut()
			.run_system_once(ApplyMaterial::<_SeparateMaterialA>::system)?;

		let [child] = unpack!(1, children(&app, child_child));
		assert_eq!(
			(Some(&separate), Some(&mesh_handle)),
			(
				child
					.get::<MeshMaterial3d<_SeparateMaterialA>>()
					.map(|MeshMaterial3d(handle)| handle),
				child.get::<Mesh3d>().map(|Mesh3d(handle)| handle),
			)
		);
		Ok(())
	}

	#[test]
	fn insert_render_layer_on_nth_parent_child_when_set_to_second_pass(
	) -> Result<(), RunSystemError> {
		let mut app = setup();
		let material = MeshMaterial3d(new_handle::<StandardMaterial>());
		let mesh_handle = new_handle::<Mesh>();
		let mesh = Mesh3d(mesh_handle.clone());
		let separate = new_handle::<_SeparateMaterialA>();
		let parent = app.world_mut().spawn(ApplyMaterial(separate.clone())).id();
		let child = app.world_mut().spawn_empty().set_parent(parent).id();
		let child_child = app
			.world_mut()
			.spawn((material, mesh))
			.set_parent(child)
			.id();

		app.world_mut()
			.run_system_once(ApplyMaterial::<_SeparateMaterialA>::system)?;

		let [child] = unpack!(1, children(&app, child_child));
		assert_eq!(
			Some(&CameraLabel::<SecondPass>::render_layers()),
			child.get::<RenderLayers>()
		);
		Ok(())
	}

	#[test]
	fn reuse_clone_with_mesh_for_second_pass() -> Result<(), RunSystemError> {
		let mut app = setup();
		let material = MeshMaterial3d(new_handle::<StandardMaterial>());
		let mesh_handle = new_handle::<Mesh>();
		let mesh = Mesh3d(mesh_handle.clone());
		let separate = new_handle::<_SeparateMaterialB>();
		let parent = app
			.world_mut()
			.spawn((
				ApplyMaterial(new_handle::<_SeparateMaterialA>()),
				ApplyMaterial(separate.clone()),
			))
			.id();
		let child = app.world_mut().spawn_empty().set_parent(parent).id();
		let child_child = app
			.world_mut()
			.spawn((material, mesh))
			.set_parent(child)
			.id();

		app.world_mut()
			.run_system_once(ApplyMaterial::<_SeparateMaterialA>::system)?;
		app.world_mut()
			.run_system_once(ApplyMaterial::<_SeparateMaterialB>::system)?;

		let [child] = unpack!(1, children(&app, child_child));
		assert_eq!(
			(Some(&separate), Some(&mesh_handle)),
			(
				child
					.get::<MeshMaterial3d<_SeparateMaterialB>>()
					.map(|MeshMaterial3d(handle)| handle),
				child.get::<Mesh3d>().map(|Mesh3d(handle)| handle),
			)
		);
		Ok(())
	}

	#[test]
	fn do_not_run_again_on_separately_cloned_child() -> Result<(), RunSystemError> {
		let mut app = setup();
		let material = MeshMaterial3d(new_handle::<StandardMaterial>());
		let mesh_handle = new_handle::<Mesh>();
		let mesh = Mesh3d(mesh_handle.clone());
		let separate = new_handle::<_SeparateMaterialA>();
		let parent = app.world_mut().spawn(ApplyMaterial(separate.clone())).id();
		let child = app.world_mut().spawn_empty().set_parent(parent).id();
		let child_child = app
			.world_mut()
			.spawn((material, mesh))
			.set_parent(child)
			.id();

		app.world_mut()
			.run_system_once(ApplyMaterial::<_SeparateMaterialA>::system)?;
		let [child] = unpack!(1, children(&app, child_child).map(|e| e.id()));
		app.world_mut()
			.run_system_once(ApplyMaterial::<_SeparateMaterialA>::system)?;
		assert_eq!(0, children(&app, child).count());
		Ok(())
	}

	#[test]
	fn do_not_apply_other_material_on_separately_cloned_child() -> Result<(), RunSystemError> {
		let mut app = setup();
		let material = MeshMaterial3d(new_handle::<StandardMaterial>());
		let mesh_handle = new_handle::<Mesh>();
		let mesh = Mesh3d(mesh_handle.clone());
		let parent = app
			.world_mut()
			.spawn((
				ApplyMaterial(new_handle::<_SeparateMaterialA>()),
				ApplyMaterial(new_handle::<_Material>()),
			))
			.id();
		let child = app.world_mut().spawn_empty().set_parent(parent).id();
		let child_child = app
			.world_mut()
			.spawn((material, mesh))
			.set_parent(child)
			.id();

		app.world_mut()
			.run_system_once(ApplyMaterial::<_SeparateMaterialA>::system)?;
		app.world_mut()
			.run_system_once(ApplyMaterial::<_Material>::system)?;

		let [child] = unpack!(1, children(&app, child_child));
		assert_eq!(
			None,
			child
				.get::<MeshMaterial3d<_Material>>()
				.map(|MeshMaterial3d(handle)| handle)
		);
		Ok(())
	}

	#[test]
	fn do_not_set_material_on_nth_parent_child_when_set_to_clone_mesh() -> Result<(), RunSystemError>
	{
		let mut app = setup();
		let material = MeshMaterial3d(new_handle::<StandardMaterial>());
		let mesh_handle = new_handle::<Mesh>();
		let mesh = Mesh3d(mesh_handle.clone());
		let separate = new_handle::<_SeparateMaterialA>();
		let parent = app.world_mut().spawn(ApplyMaterial(separate.clone())).id();
		let child = app.world_mut().spawn_empty().set_parent(parent).id();
		let child_child = app
			.world_mut()
			.spawn((material, mesh))
			.set_parent(child)
			.id();

		app.world_mut()
			.run_system_once(ApplyMaterial::<_SeparateMaterialA>::system)?;

		assert_eq!(
			None,
			app.world()
				.entity(child_child)
				.get::<MeshMaterial3d<_SeparateMaterialA>>()
				.map(|MeshMaterial3d(handle)| handle),
		);
		Ok(())
	}

	#[test]
	fn remove_standard_material_when_using_on_original_mesh() -> Result<(), RunSystemError> {
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
			None,
			app.world()
				.entity(child)
				.get::<MeshMaterial3d<StandardMaterial>>()
				.map(|MeshMaterial3d(handle)| handle)
		);
		Ok(())
	}

	#[test]
	fn remove_standard_material_when_using_on_copied_mesh() -> Result<(), RunSystemError> {
		let mut app = setup();
		let material = MeshMaterial3d(new_handle::<StandardMaterial>());
		let replacement = new_handle::<_SeparateMaterialA>();
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
			.run_system_once(ApplyMaterial::<_SeparateMaterialA>::system)?;

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
	fn run_even_when_standard_material_missing() -> Result<(), RunSystemError> {
		let mut app = setup();
		let replacement = new_handle::<_Material>();
		let parent = app
			.world_mut()
			.spawn(ApplyMaterial(replacement.clone()))
			.id();
		let child = app
			.world_mut()
			.spawn(Mesh3d(new_handle::<Mesh>()))
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
	fn run_only_once() -> Result<(), RunSystemError> {
		let mut app = setup();
		let parent = app
			.world_mut()
			.spawn(ApplyMaterial(new_handle::<_Material>()))
			.id();
		let child = app
			.world_mut()
			.spawn(Mesh3d(new_handle::<Mesh>()))
			.set_parent(parent)
			.id();

		app.world_mut()
			.run_system_once(ApplyMaterial::<_Material>::system)?;
		app.world_mut()
			.entity_mut(child)
			.remove::<MeshMaterial3d<_Material>>();
		app.world_mut()
			.run_system_once(ApplyMaterial::<_Material>::system)?;

		assert_eq!(
			None,
			app.world()
				.entity(child)
				.get::<MeshMaterial3d<_Material>>()
				.map(|MeshMaterial3d(handle)| handle)
		);
		Ok(())
	}
}
