#version 330 core

in vec3 position;
in vec3 world_position;
in vec3 colour;
in vec3 tex_offsets;
in vec2 tex_coords;

uniform mat4 model;
uniform mat4 projection;
uniform mat4 view;

out vec3 colPos;
out vec2 v_tex_coords;

void main() {
    gl_Position = projection*view*model*vec4(position + world_position, 1.0);
    colPos = vec3(colour);
    v_tex_coords = tex_offsets.xy+tex_coords*tex_offsets.z;
}