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

import android.content.Context;
import android.renderscript.RSTextureView;
import android.renderscript.RenderScriptGL;
import android.renderscript.RenderScriptGL.SurfaceConfig;
import android.util.AttributeSet;

public class ParticuleView extends RSTextureView {

    private static final int PARTICULE_COUNT = 750;

    private RenderScriptGL mRs;
    private ParticuleRS particules;

    public ParticuleView(Context context) {
        super(context);
    }

    public ParticuleView(Context context, AttributeSet set) {
        super(context, set);
    }

    @Override
    protected void onAttachedToWindow() {
        super.onAttachedToWindow();

        // Création du contexte RenderScript
        SurfaceConfig sc = new SurfaceConfig();
        mRs = createRenderScriptGL(sc);

        // Création et initialisation de notre script
        particules = new ParticuleRS();
        particules.init(mRs, getResources(), PARTICULE_COUNT);
    }

    @Override
    protected void onSizeChanged(int w, int h, int oldw, int oldh) {
        super.onSizeChanged(w, h, oldw, oldh);

        // Définition des dimensions et bind
        particules.setWidthAndHeight(w, h);
        mRs.bindRootScript(particules.getScript());
    }

    @Override
    protected void onDetachedFromWindow() {
        super.onDetachedFromWindow();
        if (mRs != null) {
            mRs = null;
            destroyRenderScriptGL();
        }
    }

    public void setThreadMode(boolean mode) {
        if (particules != null) {
            particules.setThreadMode(mode);
        }
    }
}
