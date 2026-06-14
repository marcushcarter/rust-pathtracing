#version 460 core
in vec2 vUV;
out vec4 FragColor;
layout (binding = 0) uniform sampler2D uImage;
void main() {
    vec3 c = texture(uImage, vUV).rgb;
    FragColor = vec4(sqrt(max(c, vec3(0.0))), 1.0);
}