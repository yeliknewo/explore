#version 150 core

in vec2 v_Uv;

uniform sampler2D t_Texture;

uniform b_TextureData {
    vec4 u_Tint;
    vec2 u_TilesheetRect;
}

out vec4 Target0;

void main() {
    ivec2 TextureSize = textureSize(t_Texture, 0);

    Target0 = texture(t_Texture, u_TilesheetPos + v_Uv) * u_Tint;
}
