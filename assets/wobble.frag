// Wobble fragment shader for Macroquad
// Save as assets/wobble.frag

#version 100
precision mediump float;

varying vec2 uv;
uniform sampler2D Texture;
uniform vec4 _ScreenSize;
uniform vec4 _Time;

void main() {
    float strength = 0.015;
    float speed = 2.0;
    float freq = 12.0;
    float time = _Time.x;
    // Correct for aspect ratio
    vec2 aspect = vec2(_ScreenSize.x / _ScreenSize.y, 1.0);
    vec2 uv_centered = (uv - 0.5) * aspect + 0.5;
    vec2 wobble = vec2(
        sin(uv_centered.y * freq + time * speed),
        cos(uv_centered.x * freq + time * speed)
    ) * strength;
    vec2 uv_wobble = uv_centered + wobble;
    // Undo aspect correction
    uv_wobble = (uv_wobble - 0.5) / aspect + 0.5;
    gl_FragColor = texture2D(Texture, uv_wobble);
}
