#version 450
layout(std430, set = 0, binding = 0) readonly buffer Frame { vec4 pixels[]; } frame;
layout(location = 0) out vec4 color;
void main() {
    uint index = uint(gl_FragCoord.y) * 640u + uint(gl_FragCoord.x);
    // The guest supplies linear colors. Vulkan does exposure and display gamma.
    vec3 linear = max(frame.pixels[index].rgb, vec3(0.0));
    color = vec4(sqrt(clamp(linear, 0.0, 1.0)), 1.0);
}
