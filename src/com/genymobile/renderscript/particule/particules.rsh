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

typedef struct Particule {
	float2 position;
	float2 vitesse;
	float masse;
} Particule_t;


// Constantes
float2 gravity = {0.f, 9.8f};

// Paramètre de la méthode commune
long previousTime;
int frameCount;

// Méthode commune
void drawFPS() {
	long now = rsUptimeMillis();
	if ((now - previousTime) > 1000) {
		previousTime = now;
		rsDebug("Particules - fps:", frameCount);
		frameCount = 0;
	} else {
		frameCount++;
	}
}