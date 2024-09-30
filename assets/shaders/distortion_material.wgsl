#import bevy_pbr::forward_io::VertexOutput
#import bevy_pbr::mesh_view_bindings::view
#import bevy_pbr::view_transformations::position_world_to_ndc
#import bevy_pbr::view_transformations::ndc_to_uv
#import bevy_pbr::mesh_view_bindings
#import bevy_render::globals::Globals

@group(0) @binding(11) var<uniform> globals: Globals;

@group(2) @binding(0) var first_pass: texture_2d<f32>;
@group(2) @binding(1) var first_sampler: sampler;

struct PulseParams {
    speed: f32,
    frequency: f32,
}

struct FresnelParams {
    power: f32,
}

@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {
    var fresnel_params: FresnelParams;
    fresnel_params.power = .5;

    var pulse_params: PulseParams;
    pulse_params.speed = .3;
    pulse_params.frequency = 10.;

    let view_normal = normalize(view.world_position - mesh.world_position.xyz);
    let fresnel = pulse_inwards(pulse_params, fresnel(fresnel_params, mesh, view_normal));
    let uv = offset_uv_position(mesh, fresnel, view_normal);

    return textureSample(first_pass, first_sampler, uv);
}

fn pulse_inwards(params: PulseParams, value: f32) -> f32 {
    // I am sure there is a better way to do this, but this get's the job done.
    let offset = params.frequency * (globals.time * params.speed + value);
    return 1. - abs(sin(offset));
}

fn fresnel(params: FresnelParams, mesh: VertexOutput, view_normal: vec3<f32>) -> f32 {
    // concept taken from fresnel example in https://github.com/rust-adventure/bevy-examples
    let normal = normalize(mesh.world_normal);  // after testing, world normals seen not to be normalized
    let angle = dot(normal, view_normal);
    return pow(1. - angle, params.power);
}

fn offset_uv_position(mesh: VertexOutput, offset: f32, view_normal: vec3<f32>) -> vec2<f32> {
    let v_offset = normalize(project_onto_plane(view_normal, mesh.world_normal));
    return position_world_to_uv(mesh.world_position.xyz + v_offset * offset);
}

fn project_onto_plane(view_normal: vec3<f32>, v: vec3<f32>) -> vec3<f32> {
    let v_onto_view_plane_normal = dot(v, view_normal) * view_normal;
    return v - v_onto_view_plane_normal;
}

fn position_world_to_uv(world_position: vec3<f32>) -> vec2<f32> {
    let ndc = position_world_to_ndc(world_position).xy;
    return ndc_to_uv(ndc);
}
