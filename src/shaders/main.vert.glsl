
#version 140
in vec2 position;
out vec2 asdf_position;
void main() {
    gl_Position = vec4(position, 0.0, 1.0);
    asdf_position = position / 2 + vec2(0.5, 0.5);
}
