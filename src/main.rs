use bevy::{
	color::palettes::css::{DARK_CYAN, WHITE},
	input::mouse::{MouseMotion, MouseWheel},
	prelude::*,
	render::camera::RenderTarget,
};
use project_zyheeda_bevy_shaders::{
	components::{toggle_visibility::ToggleVisibility, ReplacementMaterial},
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
		.add_systems(
			Update,
			(WindowSize::update, RenderTargetImage::update).chain(),
		)
		.add_systems(Update, (ToggleVisibility::toggle, ToggleVisibility::apply))
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

	let render_image_preview = commands
		.spawn((
			ImageNode {
				image: render_target.image.clone(),
				..default()
			},
			Node {
				width: Val::Vw(100. / 3.),
				height: Val::Vh(100. / 3.),
				..default()
			},
		))
		.id();

	commands.insert_resource(render_target);

	commands
		.spawn(Node {
			position_type: PositionType::Absolute,
			left: Val::Px(0.),
			bottom: Val::Px(0.),
			margin: UiRect::all(Val::Px(5.)),
			..default()
		})
		.with_children(toggle_visibility_ui(
			render_image_preview,
			"Show pre-rendered Image",
		));
}

fn toggle_visibility_ui(entity: Entity, text: &'static str) -> impl Fn(&mut ChildBuilder) {
	move |container| {
		let mut toggles = vec![entity];
		container
			.spawn((
				Button,
				Node {
					align_items: AlignItems::Center,
					..default()
				},
			))
			.with_children(|button| {
				button
					.spawn((
						Node {
							width: Val::Px(20.),
							height: Val::Px(20.),
							align_items: AlignItems::Center,
							justify_content: JustifyContent::Center,
							border: UiRect::all(Val::Px(2.)),
							margin: UiRect::right(Val::Px(4.)),
							..default()
						},
						BorderColor::from(WHITE),
					))
					.with_children(|checkbox| {
						let is_visible = checkbox
							.spawn((
								Node {
									width: Val::Px(10.),
									height: Val::Px(10.),
									..default()
								},
								BackgroundColor::from(WHITE),
							))
							.id();

						toggles.push(is_visible);
					});

				button.spawn((
					Text::new(text),
					TextFont {
						font_size: 15.,
						..default()
					},
				));
			})
			.insert(ToggleVisibility {
				toggles,
				visible: false,
			});
	}
}
