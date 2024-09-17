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

fn update_position(position: vec4<f32>) -> vec4<f32> {
    return position;
}

@vertex
fn vertex(vertex: Vertex) -> VertexOutput {
    let world = get_world_from_local(vertex.instance_index);
    let vertex_position = vec4<f32>(vertex.position, 1.0);
    let clip_position = mesh_position_local_to_clip(world, vertex_position);
    let world_position = mesh_position_local_to_world(world, vertex_position);

    var out: VertexOutput;
    out.position = update_position(clip_position);
    out.world_position = world_position;
    out.world_normal = mesh_normal_local_to_world(vertex.normal, vertex.instance_index);
    out.uv = vertex.uv;
    return out;
}

@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {
    return material_color * textureSample(material_color_texture, material_color_sampler, mesh.uv);
}
