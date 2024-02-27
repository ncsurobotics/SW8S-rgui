#![allow(non_snake_case)]

use const_format::concatcp;
use dioxus::prelude::*;
use dioxus_desktop::window;
use dioxus_desktop::wry::WebViewExtUnix;
use log::{info, LevelFilter};
use webkit2gtk::{SettingsExt, WebViewExt};

const VIDEO_BASE: &str = "http://localhost:8888/";
const VIDEO_FRONT: &str = concatcp!(VIDEO_BASE, "mystream");
const VIDEO_BOTTOM: &str = concatcp!(VIDEO_BASE, "mystream");

fn main() {
    // Init debug
    dioxus_logger::init(LevelFilter::Info).expect("failed to init logger");

    launch_desktop(app);
}

fn app() -> Element {
    info!("Started app");

    #[cfg(target_family = "unix")]
    {
        let webview = window().webview.webview();
        if let Some(settings) = webview.settings() {
            enable_web_features(&settings);

            #[cfg(target_os = "linux")]
            allow_all_permissions(&webview);
        }
    }

    rsx! {
        h1 { "SW8S-rgui" }
        iframe { width : 320, height : 240, scrolling : false, src : VIDEO_FRONT, allow : "*" }
        iframe { width : 320, height : 240, scrolling : false, src : VIDEO_BOTTOM, allow : "*" }
    }
}

fn enable_web_features(settings: &webkit2gtk::Settings) {
    info!("enabling webrtc");
    settings.set_enable_webrtc(true);
    settings.set_enable_media_stream(true);
    settings.set_enable_mediasource(true);
    settings.set_enable_media(true);
    settings.set_enable_media_capabilities(true);
    settings.set_enable_encrypted_media(true);
    // settings.set_enable_mock_capture_devices(true);
    settings.set_media_playback_requires_user_gesture(false);
    settings.set_media_playback_allows_inline(true);
    settings.set_media_content_types_requiring_hardware_support(Some("video/m3u8"));
}

#[cfg(target_os = "linux")]
fn allow_all_permissions(webview: &webkit2gtk::WebView) {
    info!("giving all permissions");
    use webkit2gtk::PermissionRequestExt;
    // Allow all permission requests for debugging
    let _ = webview.connect_permission_request(move |_, request| {
        request.allow();
        true
    });
}
