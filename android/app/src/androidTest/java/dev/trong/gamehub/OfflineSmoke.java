package dev.trong.gamehub;
import android.app.Instrumentation;
import android.content.Context;
import android.content.Intent;
import android.content.pm.PackageInfo;
import android.content.pm.PackageManager;
import android.net.Uri;
import android.os.SystemClock;
import android.webkit.WebView;
import android.view.ViewGroup;
import androidx.test.ext.junit.runners.AndroidJUnit4;
import androidx.test.platform.app.InstrumentationRegistry;
import org.junit.Test;
import org.junit.runner.RunWith;
import java.util.concurrent.ArrayBlockingQueue;
import java.util.concurrent.TimeUnit;
import static org.junit.Assert.*;

@RunWith(AndroidJUnit4.class)
public class OfflineSmoke {
    private String js(WebView web,String script) throws Exception {
        ArrayBlockingQueue<String> result=new ArrayBlockingQueue<>(1);
        web.post(()->web.evaluateJavascript(script,result::offer));
        return result.poll(5,TimeUnit.SECONDS);
    }
    private void awaitJs(WebView web,String expr) throws Exception {
        long end=SystemClock.uptimeMillis()+45000;
        while(SystemClock.uptimeMillis()<end){if("true".equals(js(web,expr)))return;SystemClock.sleep(150);}
        fail("Timed out waiting for "+expr+"; status="+js(web,"document.querySelector('#status')?.textContent"));
    }
    @Test public void bootsGameWithoutInternetPermission() throws Exception {
        Instrumentation inst=InstrumentationRegistry.getInstrumentation();
        Context app=inst.getTargetContext();
        assertEquals(PackageManager.PERMISSION_DENIED,app.checkSelfPermission("android.permission.INTERNET"));
        assertTrue(MainActivity.permits(Uri.parse("https://appassets.androidplatform.net/assets/game-hub.wasm")));
        assertFalse(MainActivity.permits(Uri.parse("https://example.org/assets/index.html")));
        assertFalse(MainActivity.permits(Uri.parse("file:///data/local/tmp/index.html")));
        Intent intent=new Intent(app,MainActivity.class).addFlags(Intent.FLAG_ACTIVITY_NEW_TASK);
        MainActivity activity=(MainActivity)inst.startActivitySync(intent);
        try {
            WebView[] holder=new WebView[1];
            inst.runOnMainSync(()->holder[0]=(WebView)((ViewGroup)activity.findViewById(android.R.id.content)).getChildAt(0));
            WebView web=holder[0];
            awaitJs(web,"!!document.querySelector('#start')");
            js(web,"document.querySelector('#start').click()");
            awaitJs(web,"document.body.dataset.scene === 'hub'");
            assertEquals("false",js(web,"!!window.Android"));
        } finally {inst.runOnMainSync(activity::finish);}
    }
}
