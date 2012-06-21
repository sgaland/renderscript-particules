typedef struct Particule {
	float2 position;
	float2 vitesse;
	float masse;
} Particule_t;

typedef struct Controleur {
	rs_allocation in;
	rs_allocation out;
	int size;
} Controleur_t;

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