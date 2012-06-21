#pragma version(1)
#pragma rs java_package_name(com.genymobile.renderscript.particule)

#include "particules.rsh"

int width;
int height;

int count;


float2 gravity = {0.f, 9.8f};

void root(const Particule_t *in, Particule_t *out, const Controleur_t *ctl, uint32_t x ) {

    float2 force = {0, 0};
   	float2 position = in->position;
   	
   	count = ctl->size;
   	const Particule_t* p = rsGetElementAt(ctl->in, 0);;
   	
   	for (int j = 0; j < count; j++) {
   		float2 position2 = p->position;
   		float2 vecteur = position - position2;
   		float2 vecteur2 = vecteur * vecteur;
   		float distance = sqrt(vecteur2.x + vecteur2.y);
   		
   		if (distance < 1) {
   			continue;
   		}
   		if (distance < 100) {
              force += (vecteur / (distance*distance)) * 50.f * p->masse * in->masse;
   		}
   		p++;
   	}

   	out->vitesse = 0.95f * in->vitesse;
   	out->position = in->position;
    	
   	out->vitesse += force;
 	out->vitesse += out->masse * gravity * 0.1f;
	out->position+= out->vitesse;
 		
	if (out->position.x > width) {
		out->vitesse.x = - out->vitesse.x * 0.75;
		out->position.x = 2 * width - out->position.x;
	}
	if (out->position.x < 0) {
		out->vitesse.x = - out->vitesse.x * 0.75;
		out->position.x = - out->position.x;
	} 
	
	if (out->position.y > height) {
		out->vitesse.y = - out->vitesse.y * 0.75;
		out->position.y = 2 * height - out->position.y;
	}
	if (out->position.y < 0) {
		out->vitesse.y = - out->vitesse.y * 0.75;
		out->position.y = - out->position.y;
	} 	
}
