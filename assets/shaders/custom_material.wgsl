#import bevy_pbr::mesh_functions::{get_world_from_local, mesh_position_local_to_clip}

@group(2) @binding(0) var<uniform> material_color: vec4<f32>;
@group(2) @binding(1) var<uniform> time_secs: f32;
@group(2) @binding(2) var material_color_texture: texture_2d<f32>;
@group(2) @binding(3) var material_color_sampler: sampler;

struct Vertex {
    @builtin(instance_index) instance_index: u32,
    @location(0) position: vec3<f32>,
    @location(1) blend_color: vec4<f32>,
    @location(2) uv: vec2<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) blend_color: vec4<f32>,
    @location(1) uv: vec2<f32>,
};

fn update_position(position: vec4<f32>) -> vec4<f32> {
    return position;
}

@vertex
fn vertex(vertex: Vertex) -> VertexOutput {
    var out: VertexOutput;
    var world = get_world_from_local(vertex.instance_index);
    var vertex_position = vec4<f32>(vertex.position, 1.0);
    var clip_position = mesh_position_local_to_clip(world, vertex_position);
    out.clip_position = update_position(clip_position);
    out.blend_color = vertex.blend_color;
    out.uv = vertex.uv;
    return out;
}

@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {
    return material_color * vec4(1., 1., 1., 0.5);
}
