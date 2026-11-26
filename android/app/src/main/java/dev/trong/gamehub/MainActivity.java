package dev.trong.gamehub;

import android.app.Activity;
import android.net.Uri;
import android.os.Bundle;
import android.view.View;
import android.webkit.CookieManager;
import android.webkit.PermissionRequest;
import android.webkit.WebChromeClient;
import android.webkit.WebResourceRequest;
import android.webkit.WebResourceResponse;
import android.webkit.WebSettings;
import android.webkit.WebView;
import android.webkit.WebViewClient;
import androidx.webkit.WebViewAssetLoader;
import java.io.ByteArrayInputStream;
import java.util.Collections;
import java.util.Set;
import java.util.HashSet;
import java.util.Arrays;

public final class MainActivity extends Activity {
    private static final String DOMAIN = WebViewAssetLoader.DEFAULT_DOMAIN;
    private static final String INDEX = "https://" + DOMAIN + "/assets/index.html";
    private static final Set<String> FILES = Collections.unmodifiableSet(new HashSet<>(Arrays.asList(
        "/assets/index.html", "/assets/game-hub.wasm", "/assets/host.js",
        "/assets/vendor/mq_js_bundle.js", "/assets/hub.css"
    )));
    private WebView game;

    static boolean permits(Uri uri) {
        return "https".equals(uri.getScheme()) && DOMAIN.equals(uri.getHost())
            && uri.getPort() == -1 && uri.getUserInfo() == null
            && uri.getQuery() == null && FILES.contains(uri.getEncodedPath());
    }

    @Override public void onCreate(Bundle state) {
        super.onCreate(state);
        game = new WebView(this);
        setContentView(game);
        game.setOnApplyWindowInsetsListener((view, insets) -> {
            view.setPadding(insets.getSystemWindowInsetLeft(), insets.getSystemWindowInsetTop(),
                insets.getSystemWindowInsetRight(), insets.getSystemWindowInsetBottom());
            return insets;
        });
        game.requestApplyInsets();
        WebView.setWebContentsDebuggingEnabled(BuildConfig.DEBUG);
        WebSettings settings = game.getSettings();
        settings.setJavaScriptEnabled(true);
        settings.setDomStorageEnabled(true);
        settings.setAllowFileAccess(false);
        settings.setAllowContentAccess(false);
        settings.setAllowFileAccessFromFileURLs(false);
        settings.setAllowUniversalAccessFromFileURLs(false);
        settings.setMixedContentMode(WebSettings.MIXED_CONTENT_NEVER_ALLOW);
        settings.setBlockNetworkLoads(true);
        settings.setJavaScriptCanOpenWindowsAutomatically(false);
        settings.setSupportMultipleWindows(false);
        CookieManager.getInstance().setAcceptCookie(false);
        CookieManager.getInstance().setAcceptThirdPartyCookies(game, false);
        WebViewAssetLoader loader = new WebViewAssetLoader.Builder()
            .addPathHandler("/assets/", new WebViewAssetLoader.AssetsPathHandler(this)).build();
        game.setWebViewClient(new WebViewClient() {
            @Override public boolean shouldOverrideUrlLoading(WebView view, WebResourceRequest request) {
                return !INDEX.equals(request.getUrl().toString());
            }
            @Override public WebResourceResponse shouldInterceptRequest(WebView view, WebResourceRequest request) {
                Uri u = request.getUrl();
                if ("GET".equals(request.getMethod()) && permits(u)) {
                    WebResourceResponse result = loader.shouldInterceptRequest(u);
                    if (result != null) {
                        String path = u.getPath();
                        if (path.endsWith(".wasm")) result.setMimeType("application/wasm");
                        else if (path.endsWith(".js")) result.setMimeType("application/javascript");
                        else if (path.endsWith(".css")) result.setMimeType("text/css");
                        return result;
                    }
                }
                return new WebResourceResponse("text/plain", "UTF-8", 403, "Blocked",
                    Collections.emptyMap(), new ByteArrayInputStream(new byte[0]));
            }
        });
        game.setWebChromeClient(new WebChromeClient() {
            @Override public void onPermissionRequest(PermissionRequest request) { request.deny(); }
        });
        game.loadUrl(INDEX);
    }

    @Override public void onBackPressed() {
        game.evaluateJavascript("typeof window.gameHubBack === 'function' && window.gameHubBack()",
            handled -> { if (!"true".equals(handled)) finish(); });
    }
    @Override protected void onPause() { super.onPause(); game.onPause(); }
    @Override protected void onResume() { super.onResume(); game.onResume(); }
    @Override protected void onDestroy() {
        if (game != null) { game.stopLoading(); ((android.view.ViewGroup)game.getParent()).removeView(game); game.destroy(); }
        super.onDestroy();
    }
}
