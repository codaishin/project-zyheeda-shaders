#import bevy_pbr::forward_io::VertexOutput
#import bevy_pbr::mesh_view_bindings::view
#import bevy_pbr::mesh_view_bindings
#import bevy_render::globals::Globals

@group(0) @binding(11) var<uniform> globals: Globals;

@group(2) @binding(0) var first_pass: texture_2d<f32>;
@group(2) @binding(1) var first_sampler: sampler;

@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {
    var view_distance = length(view.world_position - mesh.world_position.xyz);
    var view_size = view.viewport.zw;
    var uv = mesh.position.xy / view_size;

    // arbitrary distortion as proof of concept
    uv.y += (200. * sin(globals.time)) / view_size.y / view_distance;
    return textureSample(first_pass, first_sampler, uv);
}
