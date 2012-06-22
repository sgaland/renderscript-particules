
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

        SurfaceConfig sc = new SurfaceConfig();
        mRs = createRenderScriptGL(sc);

        particules = new ParticuleRS();
        particules.init(mRs, getResources(), PARTICULE_COUNT);
    }

    @Override
    protected void onSizeChanged(int w, int h, int oldw, int oldh) {
        super.onSizeChanged(w, h, oldw, oldh);
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
