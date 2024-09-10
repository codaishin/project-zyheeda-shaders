use bevy::{
	color::palettes::css::{DARK_CYAN, WHITE},
	input::mouse::MouseMotion,
	math::vec3,
	pbr::{MaterialPipeline, MaterialPipelineKey},
	prelude::*,
	reflect::TypePath,
	render::{
		mesh::MeshVertexBufferLayoutRef,
		render_resource::{
			AsBindGroup,
			RenderPipelineDescriptor,
			ShaderRef,
			SpecializedMeshPipelineError,
		},
	},
};

fn main() {
	App::new()
		.add_plugins((DefaultPlugins, MaterialPlugin::<CustomMaterial>::default()))
		.add_systems(Startup, setup)
		.add_systems(Update, move_camera)
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
		color_texture: Some(asset_server.load("branding/icon.png")),
		alpha_mode: AlphaMode::Blend,
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

fn move_camera(
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

#[derive(Asset, TypePath, AsBindGroup, Clone)]
struct CustomMaterial {
	#[uniform(0)]
	color: LinearRgba,
	#[texture(1)]
	#[sampler(2)]
	color_texture: Option<Handle<Image>>,
	alpha_mode: AlphaMode,
}

impl Material for CustomMaterial {
	fn vertex_shader() -> ShaderRef {
		"shaders/custom_material.vert".into()
	}

	fn fragment_shader() -> ShaderRef {
		"shaders/custom_material.frag".into()
	}

	fn alpha_mode(&self) -> AlphaMode {
		self.alpha_mode
	}

	fn specialize(
		_: &MaterialPipeline<Self>,
		descriptor: &mut RenderPipelineDescriptor,
		_: &MeshVertexBufferLayoutRef,
		_: MaterialPipelineKey<Self>,
	) -> Result<(), SpecializedMeshPipelineError> {
		let vert = &mut descriptor.vertex;
		vert.entry_point = "main".into();

		if let Some(frag) = descriptor.fragment.as_mut() {
			frag.entry_point = "main".into();
		}

		Ok(())
	}
}
