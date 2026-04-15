//! ─── SENTIENT BROWSER AUTOMATION ───
//!
//! Multi-browser automation:
//! - Chrome/Chromium
//! - Firefox
//! - Safari
//! - Headless modes
//! - Stealth configuration

pub mod multi_browser;

pub use multi_browser::{
    BrowserType, BrowserEngine, BrowserConfig, BrowserSession, BrowserPool,
    BrowserError, Page, PageActions, ProxyConfig, ProxyType, StealthConfig,
};
