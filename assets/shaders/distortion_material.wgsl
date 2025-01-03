@group(2) @binding(2) var first_pass: texture_2d<f32>;
@group(2) @binding(3) var first_sampler: sampler;

@fragment
fn fragment(@builtin(position) position: vec4<f32>) -> @location(0) vec4<f32> {
    return textureSample(first_pass, first_sampler, position.xy);
}
