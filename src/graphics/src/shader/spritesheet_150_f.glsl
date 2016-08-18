#version 150 core

in vec2 v_Uv;

uniform sampler2D t_Texture;

uniform b_TextureData {
    vec4 u_Tint;
    vec4 u_SpritesheetRect;
    vec2 u_SpritesheetSize;
    bool u_Mirror;
};

out vec4 Target0;

void main() {
    vec4 scaledRect =  vec4(u_SpritesheetRect.xy, u_SpritesheetRect.zw + u_SpritesheetRect.xy) / u_SpritesheetSize.xyxy;

    if (u_Mirror) {
        Target0 = texture(t_Texture, vec2(1.0 - v_Uv.x, v_Uv.y) * (scaledRect.zw - scaledRect.xy) + scaledRect.xy) * u_Tint;
    } else {
        Target0 = texture(t_Texture, v_Uv * (scaledRect.zw - scaledRect.xy) + scaledRect.xy) * u_Tint;
    }
}
