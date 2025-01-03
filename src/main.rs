use bevy::{
	color::palettes::css::{DARK_CYAN, WHITE},
	input::mouse::{MouseMotion, MouseWheel},
	prelude::*,
	render::camera::RenderTarget,
};
use project_zyheeda_bevy_shaders::{
	components::ReplacementMaterial,
	material::CustomMaterial,
	resources::{
		render_target_image::RenderTargetImage,
		window_size::WindowSize,
		CameraRotationSettings,
		CameraZoomSettings,
	},
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
		.add_systems(
			Startup,
			(
				WindowSize::initialize,
				RenderTargetImage::initialize.pipe(setup),
			),
		)
		.add_systems(Update, (WindowSize::update, RenderTargetImage::update))
		.add_systems(
			Update,
			(
				cam_movement::<MouseMotion>.run_if(holding_button(MouseButton::Right)),
				cam_movement::<MouseWheel>,
			),
		)
		.add_systems(Update, replace_standard_material)
		.add_systems(Update, set_material_time)
		.run();
}

fn setup(
	In(render_target): In<RenderTargetImage>,
	mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	mut standard_materials: ResMut<Assets<StandardMaterial>>,
	mut custom_materials: ResMut<Assets<CustomMaterial>>,
	asset_server: Res<AssetServer>,
) {
	let rotation_center = Vec3::new(0.0, 0.5, 0.0);
	let custom_material = custom_materials.add(CustomMaterial {
		color: DARK_CYAN.into(),
		alpha_mode: AlphaMode::Blend,
		color_texture: Some(asset_server.load("textures/grid.png")),
		..default()
	});
	let cam_transform = Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(rotation_center, Vec3::Y);

	commands.spawn((
		Mesh3d(meshes.add(Plane3d::new(Vec3::Y, Vec2::new(5., 5.)))),
		MeshMaterial3d(standard_materials.add(StandardMaterial {
			base_color: Color::WHITE,
			..default()
		})),
	));

	commands.spawn((
		SceneRoot(asset_server.load("models/shield.glb#Scene0")),
		ReplacementMaterial(custom_material.clone()),
		Transform::from_translation(rotation_center - Vec3::X * 1.),
	));

	commands.spawn((
		SceneRoot(asset_server.load("models/sphere.glb#Scene0")),
		ReplacementMaterial(custom_material.clone()),
		Transform::from_translation(rotation_center + Vec3::X * 1.),
	));

	commands.spawn((
		PointLight {
			color: WHITE.into(),
			shadows_enabled: false,
			intensity: 2_000_000.0,
			..default()
		},
		Transform::from_xyz(5., 5., 5.),
	));

	commands.spawn((Camera3d::default(), cam_transform));

	commands.spawn((
		Camera3d::default(),
		cam_transform,
		Camera {
			target: RenderTarget::Image(render_target.image.clone()),
			..default()
		},
	));

	// verify image is rendered correctly
	commands.spawn((
		ImageNode {
			image: render_target.image.clone(),
			..default()
		},
		Node {
			width: Val::Vw(50.),
			height: Val::Vh(50.),
			..default()
		},
	));

	commands.insert_resource(render_target);
}
