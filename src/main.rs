use bevy::{
	color::palettes::css::{DARK_CYAN, WHITE},
	input::mouse::{MouseMotion, MouseWheel},
	prelude::*,
};
use project_zyheeda_bevy_shaders::{
	bundles::MaterialAssetBundle,
	components::ReplacementMaterial,
	material::CustomMaterial,
	resources::{CameraRotationSettings, CameraZoomSettings},
	systems::{
		cam_movement::cam_movement,
		holding_button::holding_button,
		replace_standard_material::replace_standard_material,
		set_material_time::set_material_time,
	},
};

fn main() {
	App::new()
		.add_plugins((DefaultPlugins, MaterialPlugin::<CustomMaterial>::default()))
		.init_resource::<CameraRotationSettings>()
		.init_resource::<CameraZoomSettings>()
		.add_systems(Startup, setup)
		.add_systems(
			Update,
			cam_movement::<MouseMotion>.run_if(holding_button(MouseButton::Right)),
		)
		.add_systems(Update, cam_movement::<MouseWheel>)
		.add_systems(Update, replace_standard_material)
		.add_systems(Update, set_material_time)
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
		color_texture: Some(asset_server.load("textures/bubbles.png")),
		..default()
	});

	commands.spawn(MaterialAssetBundle {
		asset: asset_server.load("models/shield.glb#Scene0"),
		transform: Transform::from_translation(rotation_center - Vec3::X * 1.),
		material: ReplacementMaterial(material.clone()),
		..default()
	});

	commands.spawn(MaterialAssetBundle {
		asset: asset_server.load("models/sphere.glb#Scene0"),
		transform: Transform::from_translation(rotation_center + Vec3::X * 1.),
		material: ReplacementMaterial(material.clone()),
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
