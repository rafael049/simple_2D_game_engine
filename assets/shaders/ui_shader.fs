#version 330 core
in vec2 TexCoord;
out vec4 FragColor;

uniform sampler2D u_difuseTexture;

void main()
{
    FragColor = texture(u_difuseTexture, TexCoord);
    FragColor = vec4(1.0, 0.0, 0.0, 1.0);
}
