#pragma version(1)
#pragma rs java_package_name(com.genymobile.renderscrip.particule)

#include "rs_graphics.rsh"
#include "particules.rsh"

typedef struct Particule_Point {
	float2 position;
} Particule_Point_t;
Particule_Point_t* points;

Controleur_t controleur;
Particule_t *out;
Particule_t *in;

Particule_t* particules1;
Particule_t* particules2;

rs_script physics;

rs_mesh mesh;

int width;
int height;
bool thread_mode;

long totalFrame;

void init () {
	thread_mode = false;
}

void init_particules() {
	int count =  rsAllocationGetDimX(rsGetAllocation(particules1));
 	
 	Particule_t* p = particules1;
 	for(int i = 0; i < count; i++) {
 	 	
 		p->position.x = rsRand(width);
 		p->position.y = rsRand(height);
 		
 		p->vitesse.x = rsRand(0.f, 20.f) - 10;
 		p->vitesse.y = rsRand(0.f, 20.f) - 10;
 		
 		p->masse = rsRand(0.f, 1.f);
 		
 		p++;
 	}
}
float2 gravity = {0.f, 9.8f};
void updatePosition() {

	int count = rsAllocationGetDimX(rsGetAllocation(in));
 	
 	Particule_t* pIn = in;
 	Particule_t* pOut = out;
 	for(int i = 0; i < count; i++) {
 	    float2 force = {0, 0};
    	float2 position = pIn->position;
    	
    	Particule_t* p = in;
    	for (int j = 0; j < count; j++) {
    		float2 position2 = p->position;
    		float2 vecteur = position - position2;
    		float2 vecteur2 = vecteur * vecteur;
    		float distance = sqrt(vecteur2.x + vecteur2.y);
    		
    		if (distance < 1) {
    			continue;
    		}
    		if (distance < 100) {
               force += (vecteur / (distance*distance)) * 50.f * p->masse * pIn->masse;
    		}
    		
    		p++;
    	}
    	
    	pOut->vitesse =  0.95f * pIn->vitesse;
    	pOut->position = pIn->position;
    	
    	pOut->vitesse += force;
 	 	pOut->vitesse += pOut->masse * gravity * 0.1f;
 		pOut->position+= pOut->vitesse;
 		
 		if (pOut->position.x > width) {
 			pOut->vitesse.x = - pOut->vitesse.x * 0.75;
 			pOut->position.x = 2 * width - pOut->position.x;
 		}
 		if (pOut->position.x < 0) {
 			pOut->vitesse.x = - pOut->vitesse.x * 0.75;
 			pOut->position.x = - pOut->position.x;
 		} 
 		
 		if (pOut->position.y > height) {
 			pOut->vitesse.y = - pOut->vitesse.y * 0.75;
 			pOut->position.y = 2 * height - pOut->position.y;
 		}
 		if (pOut->position.y < 0) {
 			pOut->vitesse.y = - pOut->vitesse.y * 0.75;
 			pOut->position.y = - pOut->position.y;
 		}
 		pIn++;
 		pOut++;
 	}
}

int root() {
    rsgClearColor(0.0f, 0.0f, 0.0f, 0.0f);
    
    if (totalFrame % 2) {
    	out = particules1;
    	in = particules2;
    } else {
    	out = particules2;
    	in = particules1;
    }
    totalFrame++;
    
	if (thread_mode) {
	    controleur.in = rsGetAllocation(in);
    	controleur.out = rsGetAllocation(out);
    	controleur.size = rsAllocationGetDimX(controleur.out);
		rsForEach(physics, controleur.in, controleur.out, &controleur, controleur.size);
	} else {
		updatePosition();
	}
	
    int count = rsAllocationGetDimX(rsGetAllocation(out));
    for (int i=0; i < count; i++) {
        points[i].position = out[i].position;
    }
    
    rsgDrawMesh(mesh);	
	drawFPS();
    
    return 1;
}