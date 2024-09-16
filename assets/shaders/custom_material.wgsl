#import bevy_pbr::mesh_functions::get_world_from_local
#import bevy_pbr::mesh_functions::mesh_position_local_to_clip
#import bevy_pbr::mesh_functions::mesh_position_local_to_world
#import bevy_pbr::mesh_functions::mesh_normal_local_to_world
#import bevy_pbr::forward_io::Vertex
#import bevy_pbr::mesh_view_bindings::view
#import bevy_pbr::utils::interleaved_gradient_noise

@group(2) @binding(0) var<uniform> material_color: vec4<f32>;
@group(2) @binding(1) var<uniform> time_secs: f32;
@group(2) @binding(2) var material_color_texture: texture_2d<f32>;
@group(2) @binding(3) var material_color_sampler: sampler;

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) world_position: vec4<f32>,
    @location(1) world_normal: vec3<f32>,
    @location(2) uvw: vec3<f32>,
};

// Vertex shader

fn wobble(position: vec4<f32>, speed: f32, waves: f32, amplitude: f32) -> vec4<f32> {
    var result = position;

    result.x += sin(position.y * waves + time_secs * speed) * amplitude;
    result.x += sin(position.z * waves + time_secs * speed) * amplitude;

    return result;
}

@vertex
fn vertex(vertex: Vertex) -> VertexOutput {
    let world = get_world_from_local(vertex.instance_index);

    var vertex_position = vec4(vertex.position, 1.0);
    vertex_position = wobble(vertex_position, 9.0, 30., 0.004);

    var out: VertexOutput;
    out.position = mesh_position_local_to_clip(world, vertex_position);
    out.world_position = mesh_position_local_to_world(world, vertex_position);
    out.world_normal = mesh_normal_local_to_world(vertex.normal, vertex.instance_index);
    out.uvw = vertex.position;
    return out;
}

// Fragment shader

const RAND_DISTORTION_AXIS_X = vec3(127.1, 311.7, 513.7);
const RAND_DISTORTION_AXIS_Y = vec3(269.5, 183.3, 396.5);
const RAND_DISTORTION_AXIS_Z = vec3(421.3, 314.1, 119.7);
const RAND_AMPLITUDE_FACTOR = 43758.5453123;

fn random_distortion(xyz: vec3<f32>) -> vec3<f32> {
    return vec3(
        dot(xyz, RAND_DISTORTION_AXIS_X),
        dot(xyz, RAND_DISTORTION_AXIS_Y),
        dot(xyz, RAND_DISTORTION_AXIS_Z),
    );
}

fn random_from_seed_3D(xyz: vec3<f32>, min: vec3<f32>, max: vec3<f32>) -> vec3<f32> {
    let sin_offset = random_distortion(xyz);
    let amplitude = sin(sin_offset) * RAND_AMPLITUDE_FACTOR;

    return min + (max - min) * fract(amplitude);
}

fn randomize_corner_gradients(cell_index: vec3<f32>, cell_fract: vec3<f32>) -> array<array<f32,4>,2> {
    return array(
        array(
            randomize_corner_gradient(cell_index, cell_fract, vec3(0.0, 0.0, 0.0)),
            randomize_corner_gradient(cell_index, cell_fract, vec3(1.0, 0.0, 0.0)),
            randomize_corner_gradient(cell_index, cell_fract, vec3(0.0, 1.0, 0.0)),
            randomize_corner_gradient(cell_index, cell_fract, vec3(1.0, 1.0, 0.0)),
        ),
        array(
            randomize_corner_gradient(cell_index, cell_fract, vec3(0.0, 0.0, 1.0)),
            randomize_corner_gradient(cell_index, cell_fract, vec3(1.0, 0.0, 1.0)),
            randomize_corner_gradient(cell_index, cell_fract, vec3(0.0, 1.0, 1.0)),
            randomize_corner_gradient(cell_index, cell_fract, vec3(1.0, 1.0, 1.0)),
        ),
    );
}

fn randomize_corner_gradient(cell_index: vec3<f32>, cell_fract: vec3<f32>, corner: vec3<f32>) -> f32 {
    let corner_offset = cell_index + corner;
    let pixel = random_from_seed_3D(corner_offset, vec3(-1.0), vec3(1.0));
    let gradient_vector = cell_fract - corner;
    return dot(pixel, gradient_vector);
}

fn mix_corners_2D(corners: array<f32, 4>, cell_fract: vec2<f32>) -> f32 {
    return mix(
        mix(corners[0], corners[1], cell_fract.x),
        mix(corners[2], corners[3], cell_fract.x),
        cell_fract.y,
    );
}

fn mix_corners_3D(corners: array<array<f32, 4>, 2>, cell_fract: vec3<f32>) -> f32 {
    return mix(
        mix_corners_2D(corners[0], cell_fract.xy),
        mix_corners_2D(corners[1], cell_fract.xy),
        cell_fract.z
    );
}

fn fresnel(mesh: VertexOutput) -> f32 {
    // concept taken from fresnel example in https://github.com/rust-adventure/bevy-examples
    let normal = normalize(mesh.world_normal);
    let view_vector = normalize(view.world_position.xyz - mesh.world_position.xyz);
    let normalized_angle = dot(normal, view_vector);
    return clamp(1.0 - normalized_angle, 0.0, 1.0);
}

fn perlin_noise_3D(mesh: VertexOutput, cells: u32, offset: f32) -> f32 {
    // concept taken from https://github.com/gegamongy/3DPerlinNoiseGodot
    let uvw = mesh.uvw * f32(cells) + offset;
    let cell_index = floor(uvw);
    let cell_fract = smoothstep(vec3(0.0), vec3(1.0), fract(uvw));
    let cell_corners = randomize_corner_gradients(cell_index, cell_fract);
    return 0.5 + mix_corners_3D(cell_corners, cell_fract);
}

fn distort(value: f32, falloff: f32, intensity: f32) -> f32 {
    return pow(value, falloff) * intensity;
}

fn sharpen(value: f32, threshold: f32) -> f32 {
    if value < threshold {
        return 0.0;
    }
    return 1.0;
}


@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {
    let min_alpha_per_effect = 0.3;
    var fresnel = fresnel(mesh);
    var noise = perlin_noise_3D(mesh, u32(6), time_secs);

    fresnel = distort(fresnel, 3.0, 8.0);
    noise = distort(noise, 15.0, 10.0);

    let alpha = max(noise, min_alpha_per_effect) * max(fresnel, min_alpha_per_effect);

    return vec4(material_color.rgb, alpha);
}
