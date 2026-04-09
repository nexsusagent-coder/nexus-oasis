//! ─── System Tray ───

use tauri::{
    menu::{Menu, MenuItem},
    tray::{TrayIcon, TrayIconBuilder},
    AppHandle, Manager, Runtime,
};

pub fn setup_tray<R: Runtime>(app: &AppHandle<R>) -> Result<(), Box<dyn std::error::Error>> {
    let quit = MenuItem::with_id(app, "quit", "Çıkış", true, None::<&str>)?;
    let show = MenuItem::with_id(app, "show", "Göster", true, None::<&str>)?;
    let hide = MenuItem::with_id(app, "hide", "Gizle", true, None::<&str>)?;
    let voice_toggle = MenuItem::with_id(app, "voice", "🎤 Ses", true, None::<&str>)?;
    
    let menu = Menu::with_items(app, &[&show, &hide, &voice_toggle, &quit])?;
    
    let _tray = TrayIconBuilder::new()
        .menu(&menu)
        .menu_on_left_click(true)
        .on_menu_event(|app, event| match event.id.as_ref() {
            "quit" => {
                app.exit(0);
            }
            "show" => {
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.show();
                    let _ = window.set_focus();
                }
            }
            "hide" => {
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.hide();
                }
            }
            "voice" => {
                // Toggle voice
            }
            _ => {}
        })
        .build(app)?;
    
    Ok(())
}
