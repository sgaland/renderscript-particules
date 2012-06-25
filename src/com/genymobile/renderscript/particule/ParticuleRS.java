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

package com.genymobile.renderscript.particule;

import android.content.res.Resources;
import android.renderscript.Allocation;
import android.renderscript.Mesh;
import android.renderscript.RenderScriptGL;

import com.genymobile.renderscrip.particule.ScriptC_particules;
import com.genymobile.renderscrip.particule.ScriptField_Particule;
import com.genymobile.renderscrip.particule.ScriptField_Particule_Point;
import com.genymobile.renderscript.gas.R;

public class ParticuleRS {

    private ScriptC_particules mScript;
    private ScriptC_physics mScriptPhysics;

    public void init(RenderScriptGL rs, Resources res, int particulesCount) {
        // Création du script (note: init() sera exécutée)
        mScript = new ScriptC_particules(rs, res, R.raw.particules);

        // Allocation de la mémoire pour les particules et initialisation
        mScript.bind_particules1(new ScriptField_Particule(rs, particulesCount,
                Allocation.USAGE_SCRIPT));
        mScript.bind_particules2(new ScriptField_Particule(rs, particulesCount,
                Allocation.USAGE_SCRIPT));
        mScript.invoke_init_particules();

        // Création du script gérant la physique
        mScriptPhysics = new ScriptC_physics(rs, res, R.raw.physics);
        mScript.set_physics(mScriptPhysics);

        // Mise en place des éléments de rendu
        setupRendering(rs, res, particulesCount);
    }

    public void setupRendering(RenderScriptGL rs, Resources res, int particulesCount) {
        // Création des points qui dessinnerons les particules
        ScriptField_Particule_Point points = new ScriptField_Particule_Point(rs, particulesCount,
                Allocation.USAGE_SCRIPT);
        mScript.bind_points(points);

        // Création du mesh utilisant les points comme vertices
        Mesh.AllocationBuilder smb = new Mesh.AllocationBuilder(rs);
        smb.addVertexAllocation(points.getAllocation());
        smb.addIndexSetType(Mesh.Primitive.POINT);
        Mesh mesh = smb.create();
        mScript.set_mesh(mesh);
    }

    public void setWidthAndHeight(int width, int height) {
        mScript.set_width(width);
        mScript.set_height(height);
        mScript.invoke_init_particules();
        mScriptPhysics.set_width(width);
        mScriptPhysics.set_height(height);
    }

    public ScriptC_particules getScript() {
        return mScript;
    }

    public void setThreadMode(boolean mode) {
        if (mScript != null) {
            mScript.set_thread_mode(mode);
        }
    }
}
