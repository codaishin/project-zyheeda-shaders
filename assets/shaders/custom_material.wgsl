#import bevy_pbr::mesh_functions::get_world_from_local
#import bevy_pbr::mesh_functions::mesh_position_local_to_clip
#import bevy_pbr::mesh_functions::mesh_position_local_to_world
#import bevy_pbr::mesh_functions::mesh_normal_local_to_world
#import bevy_pbr::forward_io::Vertex
#import bevy_pbr::forward_io::VertexOutput

@group(2) @binding(0) var<uniform> material_color: vec4<f32>;
@group(2) @binding(1) var<uniform> time_secs: f32;
@group(2) @binding(2) var material_color_texture: texture_2d<f32>;
@group(2) @binding(3) var material_color_sampler: sampler;

fn wiggle(position: vec4<f32>, waves: f32, amplitude: f32, speed: f32) -> vec4<f32> {
    var result = position;
    result.x += sin(time_secs * speed + position.y * waves) * amplitude;
    return result;
}

fn vertex(vertex: Vertex) -> VertexOutput {
    let world = get_world_from_local(vertex.instance_index);
    let vertex_position = vec4<f32>(vertex.position, 1.0);

    var out: VertexOutput;
    out.position = mesh_position_local_to_clip(world, vertex_position);
    out.world_position = mesh_position_local_to_world(world, vertex_position);
    out.world_normal = mesh_normal_local_to_world(vertex.normal, vertex.instance_index);
    out.uv = vertex.uv;
    return out;
}

@vertex
fn vertex_slow(vertex: Vertex) -> VertexOutput {
    var out = vertex(vertex);
    out.position = wiggle(out.position, 10., 0.1, 1.);
    return out;
}

@vertex
fn vertex_fast(vertex: Vertex) -> VertexOutput {
    var out = vertex(vertex);
    out.position = wiggle(out.position, 10., 0.1, 10.);
    return out;
}

@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {
    return material_color * textureSample(material_color_texture, material_color_sampler, mesh.uv);
}
