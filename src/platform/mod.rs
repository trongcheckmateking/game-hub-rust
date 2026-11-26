//! The Rust games run unchanged on desktop, in a browser and in Android's bundled WebView.
#[cfg(target_arch = "wasm32")]
pub mod web;
