#version 450

layout(location = 0) in vec2 v_Uv;

layout(location = 0) out vec4 o_Target;

layout(set = 2, binding = 0) uniform vec4 CustomMaterial_color;

layout(set = 2, binding = 1) uniform texture2D CustomMaterial_texture;
layout(set = 2, binding = 2) uniform sampler CustomMaterial_sampler;

void main() {
	o_Target = CustomMaterial_color * texture(sampler2D(CustomMaterial_texture, CustomMaterial_sampler), v_Uv);
}
