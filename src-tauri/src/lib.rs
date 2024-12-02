use rand::Rng;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::time::Duration;
use tauri::{
    menu::{Menu, MenuItem},
    path::BaseDirectory,
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    Manager,
    AppHandle,
};
use tauri_plugin_autostart::ManagerExt;

mod command;
mod sound;

static VINE_BOOM_ENABLED: AtomicBool = AtomicBool::new(false);
static MIN_INTERVAL: AtomicU64 = AtomicU64::new(1);
static MAX_INTERVAL: AtomicU64 = AtomicU64::new(30);

fn show_main_window(app_handle: &AppHandle) {
    if let Some(window) = app_handle.get_webview_window("main") {
        if !window.is_visible().unwrap_or(false) {
            window.show().unwrap();
        }
        let _ = window.set_focus();
        if cfg!(target_os = "macos") {
            window.unminimize().unwrap();
        }
    }
}

pub fn run() {
    let mut builder = tauri::Builder::default();
    
    #[cfg(desktop)]
    {
        builder = builder.plugin(tauri_plugin_single_instance::init(move |app_handle, _argv, _cwd| {
            show_main_window(&app_handle);
        }));

        builder = builder.plugin(tauri_plugin_autostart::init(
            tauri_plugin_autostart::MacosLauncher::LaunchAgent,
            None,
        ));
    }

    builder
        .invoke_handler(tauri::generate_handler![
            command::get_status,
            command::toggle_status,
            command::set_interval,
        ])
        .setup(|app| {
            create_tray_icon(app)?;
            let window = app.get_webview_window("main").unwrap();
            let window_clone = window.clone();
            
            window.on_window_event(move |event| {
                if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                    api.prevent_close();
                    window_clone.hide().unwrap();
                }
            });
            
            // Enable autostart by default
            let autostart_manager = app.autolaunch();
            let _ = autostart_manager.enable();
            
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

pub fn create_tray_icon(app: &tauri::App) -> Result<(), tauri::Error> {
    let enable_i = MenuItem::with_id(app, "enable", "Enable Vine Boom", true, None::<&str>)?;
    let disable_i = MenuItem::with_id(app, "disable", "Disable Vine Boom", true, None::<&str>)?;
    let settings_i = MenuItem::with_id(app, "settings", "Settings", true, None::<&str>)?;
    let quit_i = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
    let menu = Menu::with_items(app, &[&enable_i, &disable_i, &settings_i, &quit_i])?;

    let app_handle = app.handle().clone();
    std::thread::spawn(move || {
        let mut rng = rand::thread_rng();
        loop {
            if VINE_BOOM_ENABLED.load(Ordering::Relaxed) {
                let min = MIN_INTERVAL.load(Ordering::Relaxed);
                let max = MAX_INTERVAL.load(Ordering::Relaxed);
                let delay = rng.gen_range(min..=max);

                std::thread::sleep(Duration::from_secs(delay));

                if VINE_BOOM_ENABLED.load(Ordering::Relaxed) {
                    if let Ok(path) = app_handle
                        .path()
                        .resolve("assets/vine.mp3", BaseDirectory::Resource)
                    {
                        sound::play_sound_async(path.to_string_lossy().into_owned());
                    }
                }
            } else {
                std::thread::sleep(Duration::from_secs(1));
            }
        }
    });

    let _tray = TrayIconBuilder::new()
        .icon(app.default_window_icon().unwrap().clone())
        .menu(&menu)
        .on_menu_event(|app, event| match event.id.as_ref() {
            "quit" => {
                app.exit(0);
            }
            "enable" => {
                VINE_BOOM_ENABLED.store(true, Ordering::Relaxed);
            }
            "disable" => {
                VINE_BOOM_ENABLED.store(false, Ordering::Relaxed);
            }
            "settings" => {
                println!("settings menu item was clicked");
            }
            _ => {}
        })
        .on_tray_icon_event(|tray, event| match event {
            TrayIconEvent::Click {
                button: MouseButton::Left,
                button_state: MouseButtonState::Up,
                ..
            } => {
                println!("left click pressed and released");
                let app = tray.app_handle();
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.show();
                    let _ = window.set_focus();
                }
            }
            TrayIconEvent::Enter { .. } => {
                sound::play_sound_async_debounced(
                    tray.app_handle()
                        .path()
                        .resolve("assets/vine.mp3", BaseDirectory::Resource)
                        .expect("failed to resolve resource")
                        .to_string_lossy()
                        .into_owned(),
                    100,
                );
            }
            _ => {}
        })
        .build(app)?;

    Ok(())
}
