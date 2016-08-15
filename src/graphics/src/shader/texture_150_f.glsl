#version 150 core

in vec2 v_Uv;

uniform sampler2D t_Texture;

uniform b_TextureData {
    vec4 u_Tint;
};

out vec4 Target0;

void main() {
    Target0 = texture(t_Texture, v_Uv) * u_Tint;
}
