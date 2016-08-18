#version 150 core

in vec2 v_Uv;

uniform sampler2D t_Texture;

uniform b_TextureData {
    vec4 u_Tint;
    vec4 u_SpritesheetRect;
    vec2 u_SpritesheetSize;
};

out vec4 Target0;

void main() {
    vec4 scaledRect =  vec4(u_SpritesheetRect.xy, u_SpritesheetRect.zw + u_SpritesheetRect.xy) / u_SpritesheetSize.xyxy;

    Target0 = texture(t_Texture, v_Uv * (scaledRect.zw - scaledRect.xy) + scaledRect.xy) * u_Tint;
}
