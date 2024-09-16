use bevy::{
	color::palettes::css::{DARK_CYAN, WHITE},
	input::mouse::{MouseMotion, MouseWheel},
	prelude::*,
};
use project_zyheeda_bevy_shaders::{
	bundles::MaterialAssetBundle,
	components::ReplacementMaterial,
	material::CustomMaterial,
	systems::cam_movement::cam_movement,
};

fn main() {
	App::new()
		.add_plugins((DefaultPlugins, MaterialPlugin::<CustomMaterial>::default()))
		.add_systems(Startup, setup)
		.add_systems(Update, replace_standard_material)
		.add_systems(Update, cam_movement::<MouseMotion>)
		.add_systems(Update, cam_movement::<MouseWheel>)
		.add_systems(Update, material_time)
		.run();
}

fn setup(
	mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	mut standard_materials: ResMut<Assets<StandardMaterial>>,
	mut custom_materials: ResMut<Assets<CustomMaterial>>,
	asset_server: Res<AssetServer>,
) {
	commands.spawn(MaterialMeshBundle {
		mesh: meshes.add(Plane3d::new(Vec3::Y, Vec2::new(5., 5.))),
		transform: Transform::from_xyz(0.0, 0.0, 0.0),
		material: standard_materials.add(StandardMaterial {
			base_color: Color::WHITE,
			..default()
		}),
		..default()
	});

	let rotation_center = Vec3::new(0.0, 0.5, 0.0);
	let material = custom_materials.add(CustomMaterial {
		color: DARK_CYAN.into(),
		alpha_mode: AlphaMode::Blend,
		..default()
	});

	commands.spawn(MaterialAssetBundle {
		asset: asset_server.load("models/shield.glb#Scene0"),
		transform: Transform::from_translation(rotation_center - Vec3::X * 1.),
		material: ReplacementMaterial(material.clone()),
		..default()
	});

	commands.spawn(MaterialMeshBundle {
		mesh: meshes.add(Sphere::default()),
		transform: Transform::from_translation(rotation_center + Vec3::X * 1.),
		material: material.clone(),
		..default()
	});

	commands.spawn(Camera3dBundle {
		transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(rotation_center, Vec3::Y),
		..default()
	});

	commands.spawn(PointLightBundle {
		transform: Transform::from_xyz(5., 5., 5.),
		point_light: PointLight {
			color: WHITE.into(),
			shadows_enabled: false,
			intensity: 2_000_000.0,
			..default()
		},
		..default()
	});
}

fn replace_standard_material(
	mut commands: Commands,
	replacements: Query<&ReplacementMaterial>,
	materials: Query<Entity, Added<Handle<StandardMaterial>>>,
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

fn material_time(
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
