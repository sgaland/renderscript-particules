package com.genymobile.renderscript.particule;

import android.app.Activity;
import android.os.Bundle;
import android.view.Menu;
import android.view.MenuItem;
import android.widget.Toast;

public class ParticuleActivity extends Activity {
    
    private ParticuleView view;
    private boolean mThreadMode;
    
    @Override
    public void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        mThreadMode = false;
        view = new ParticuleView(this);
        setContentView(view);
        view.setThreadMode(mThreadMode);
    }
    
    private void changeMode(boolean threadMode) {
        mThreadMode = threadMode;
        view.setThreadMode(mThreadMode);
        Toast.makeText(this, "Thread Mode : " + mThreadMode, Toast.LENGTH_SHORT).show();
    }
    
    @Override
    public boolean onCreateOptionsMenu(Menu menu) {
        menu.add("Change Mode");
        return true;
    }

    @Override
    public boolean onMenuItemSelected(int featureId, MenuItem item) {
        changeMode(!mThreadMode);
        return super.onMenuItemSelected(featureId, item);
    }
}