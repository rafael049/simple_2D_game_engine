#version 330 core

struct PointLight{
	float intensity;
	vec3  pos;
	vec3  color;
};

out vec4 FragColor;

in vec2 TexCoord;
in vec2 FragPos;

uniform bool outline;

uniform sampler2D u_difuseTexture;
uniform sampler2D u_normalTexture;

uniform float      u_ambientLight;
uniform PointLight u_pointLights[10];
uniform int        u_numPointLights;


float pointLightAtt(PointLight);
float normalMapAtt(PointLight, vec3);

void main()
{
		
    vec4 texColor = texture(u_difuseTexture, TexCoord);
    vec3 normalColor = texture(u_normalTexture, TexCoord).xyz;

	// texture alpha
 	if(texColor.a < 0.4){
 		discard;
 	}

	// Lighting and normal map
	
 	float att = u_ambientLight;
 	vec3 finalColor = vec3(0.0, 0.0, 0.0);

 	for(int i = 0; i < u_numPointLights; i++){
		PointLight light = u_pointLights[i];

		att = pointLightAtt(light)*normalMapAtt(light, normalColor);
		finalColor += att*light.color;
 	}

 	

	// Final color
 	FragColor = vec4(finalColor*texColor.xyz, 1.0);
}

// Point light
float pointLightAtt(PointLight light){
	vec3 lightPos = light.pos;
	//lightPos.y *= -1;
	
	float dist = length(lightPos - vec3(FragPos, 0.0));
	float kc = 0.4;
	float kl = 0.0001;
	float kq = 0.0001;

	float att = light.intensity/(kc + kl*dist + kq*dist*dist);

	return att;
}

// Normal Map
float normalMapAtt(PointLight light, vec3 normalColor) {
	vec3 normalMap = normalize(normalColor*2 - 1.0);
	vec3 fragPos = vec3(FragPos, 0.0);
	
	vec3 lightPos = light.pos;
	//lightPos.z *= -1;
	vec3 dir = normalize(lightPos - fragPos);
	float att = max(dot(normalMap, dir), 0.0);

	return att;
}
