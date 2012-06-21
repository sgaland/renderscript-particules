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
        
        mScript = new ScriptC_particules(rs, res, R.raw.particules);
        
        ScriptField_Particule_Point points = new ScriptField_Particule_Point(rs, particulesCount, Allocation.USAGE_SCRIPT );
        mScript.bind_points(points);
        
        Mesh.AllocationBuilder smb = new Mesh.AllocationBuilder(rs);
        smb.addVertexAllocation(points.getAllocation());
        smb.addIndexSetType(Mesh.Primitive.POINT);
        Mesh mesh = smb.create();
        mScript.set_mesh(mesh);
        
        mScript.bind_particules1(new ScriptField_Particule(rs, particulesCount, Allocation.USAGE_SCRIPT));
        mScript.bind_particules2(new ScriptField_Particule(rs, particulesCount, Allocation.USAGE_SCRIPT));
        mScript.invoke_init_particules();
        
        mScriptPhysics = new ScriptC_physics(rs, res, R.raw.physics);
        mScript.set_physics(mScriptPhysics);
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
