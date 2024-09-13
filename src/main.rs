use bevy::{
	color::palettes::css::{DARK_CYAN, WHITE},
	input::mouse::{MouseMotion, MouseWheel},
	math::vec3,
	prelude::*,
};
use project_zyheeda_bevy_shaders::{material::CustomMaterial, zoom_change::ZoomChange};

fn main() {
	App::new()
		.add_plugins((DefaultPlugins, MaterialPlugin::<CustomMaterial>::default()))
		.add_systems(Startup, setup)
		.add_systems(Update, rotate_camera)
		.add_systems(Update, zoom_camera)
		.add_systems(Update, material_time)
		.run();
}

fn setup(
	mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	mut standard_materials: ResMut<Assets<StandardMaterial>>,
	mut custom_materials: ResMut<Assets<CustomMaterial>>,
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

	commands.spawn(MaterialMeshBundle {
		mesh: meshes.add(Cuboid::default()),
		transform: Transform::from_translation(rotation_center - Vec3::X * 1.),
		material: material.clone(),
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

fn rotate_camera(
	time: Res<Time<Real>>,
	mut cams: Query<&mut Transform, With<Camera>>,
	mut mouse_motion: EventReader<MouseMotion>,
	mouse_input: Res<ButtonInput<MouseButton>>,
) {
	let Ok(mut cam) = cams.get_single_mut() else {
		return;
	};
	let holding_right = mouse_input.pressed(MouseButton::Right);
	let center = vec3(0.0, 0.5, 0.0);

	for event in mouse_motion.read() {
		if !holding_right {
			continue;
		}

		let distance = (cam.translation - center).length();
		cam.rotate_y(-event.delta.x * time.delta_seconds() * 0.5);
		cam.rotate_local_x(-event.delta.y * time.delta_seconds() * 0.5);
		cam.translation = center - cam.forward().as_vec3() * distance;
	}
}

fn zoom_camera(
	time: Res<Time<Real>>,
	mut cams: Query<&mut Transform, With<Camera>>,
	mut mouse_wheel: EventReader<MouseWheel>,
) {
	let Ok(mut cam) = cams.get_single_mut() else {
		return;
	};
	let center = vec3(0.0, 0.5, 0.0);

	for event in mouse_wheel.read() {
		let Ok(change) = ZoomChange::try_from(event) else {
			continue;
		};

		let distance = (cam.translation - center).length();
		let change = *change
			.scaled_by(10.)
			.scaled_by(time.delta_seconds())
			.scaled_by(distance);

		let zoomed_distance = f32::max(3., distance + change);
		cam.translation = center - cam.forward().as_vec3() * zoomed_distance;
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
