/*
 *   Copyright 2012 - Genymobile
 *
 *   Licensed under the Apache License, Version 2.0 (the "License");
 *   you may not use this file except in compliance with the License.
 *   You may obtain a copy of the License at
 *
 *       http://www.apache.org/licenses/LICENSE-2.0
 *
 *   Unless required by applicable law or agreed to in writing, software
 *   distributed under the License is distributed on an "AS IS" BASIS,
 *   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 *   See the License for the specific language governing permissions and
 *   limitations under the License.
 */
 
#pragma version(1)
#pragma rs java_package_name(com.genymobile.renderscrip.particule)

#include "rs_graphics.rsh"
#include "particules.rsh"

// Partie rendu graphique
typedef struct Particule_Point {
	float2 position;
} Particule_Point_t;
Particule_Point_t* points;
rs_mesh mesh;
long totalFrame;

// Allocations de particules
Particule_t* particules1;
Particule_t* particules2;

// Pointeurs vers les allocations d'entrée et de sortie
Particule_t *aout;
Particule_t *ain;

// Script avec gestion de la physique
rs_script physics;

// Paramètres du script
int count;
int width;
int height;
bool thread_mode;

// Méthode exécutée à l'initialisation du script
void init () {
	thread_mode = false;
}

void init_particules() {
	// Récupération du nombre de particules
	count =  rsAllocationGetDimX(rsGetAllocation(particules1));
	
	// Initialisation des particules
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

// fonctions appliquant les différents paramètres modifiant la position des particules
void updatePosition() {
 	
 	Particule_t* in = ain;
 	Particule_t* out = aout;
 	
 	// Pour chaque particule...
 	for(int i = 0; i < count; i++) {
 	    float2 force = {0, 0};
    	float2 position = in->position;
    	
    	// Calcul de la répulsivité
    	Particule_t* p = ain;
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
    	
    	out->vitesse =  0.95f * in->vitesse;
    	out->position = in->position;
    	
    	out->vitesse += (force + out->masse * gravity * 0.1f);
 		out->position+= out->vitesse;
 		
 		// Rebonds sur les parois
 		if (out->position.x > width) {
 			out->vitesse.x = - out->vitesse.x * 0.95;
 			out->position.x = 2 * width - out->position.x;
 		}
 		if (out->position.x < 0) {
 			out->vitesse.x = - out->vitesse.x * 0.95;
 			out->position.x = - out->position.x;
 		} 
 		
 		if (out->position.y > height) {
 			out->vitesse.y = - out->vitesse.y * 0.95;
 			out->position.y = 2 * height - out->position.y;
 		}
 		if (out->position.y < 0) {
 			out->vitesse.y = - out->vitesse.y * 0.95;
 			out->position.y = - out->position.y;
 		}
 		in++;
 		out++;
 	}
}

void draw() {

	// Récupération des positions
    for (int i=0; i < count; i++) {	
        points[i].position = aout[i].position;
    }

	// Nettoyage de l'écran en noir
    rsgClearColor(0.0f, 0.0f, 0.0f, 0.0f);
    
    // Dessin et log
    rsgDrawMesh(mesh);
	drawFPS();
}

int root() {

	// Changement des paramètres d'entrée et de sortie.
    if (totalFrame % 2) {
    	aout = particules1;
    	ain = particules2;
    } else {
    	aout = particules2;
    	ain = particules1;
    }
    totalFrame++;
    
    // Utilisation du mode threadé ou non
	if (thread_mode) {
    	rs_allocation in = rsGetAllocation(ain);
    	rs_allocation out = rsGetAllocation(aout);
		rsForEach(physics, in, out, &in, count);
	} else {
		updatePosition();
	}

	draw();
	    
    return 1; // On désire être appelé le plus souvent possible (minimum toutes les millisondes ou au mieu)
}