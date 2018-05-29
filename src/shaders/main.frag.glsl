
#version 140
out vec4 f_color;
in vec2 asdf_position;
layout(std140) uniform;

uniform vec2 divisions;

uniform sampler2D cells;

void main() {
    f_color = texture(cells, asdf_position); 
}