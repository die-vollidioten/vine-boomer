use rand::Rng;
use std::{
    sync::atomic::{AtomicBool, AtomicU64, Ordering},
    time::Duration,
};
use tauri::{
    menu::{Menu, MenuItem},
    path::BaseDirectory,
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    AppHandle, Manager,
};
use tauri_plugin_autostart::ManagerExt;
mod command;
mod sound;
mod storage;
use tauri_plugin_updater::UpdaterExt;


static VINE_BOOM_ENABLED: AtomicBool = AtomicBool::new(false);
static GENERATION: AtomicU64 = AtomicU64::new(0);

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
    let mut builder = tauri::Builder::default()
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_store::Builder::new().build());

    #[cfg(desktop)]
    {
        builder = builder.plugin(tauri_plugin_single_instance::init(
            move |app_handle, _argv, _cwd| {
                show_main_window(&app_handle);
            },
        ));

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
            command::is_autostart_enabled,
            command::enable_autostart,
            command::disable_autostart,
            command::set_start_enabled,
        ])
        .setup(|app| {
            let handle = app.handle().clone();
            tauri::async_runtime::spawn(async move {
                update(handle).await.unwrap();
              });
            create_tray_icon(app)?;
            storage::initialize_store(&app.handle())?;
            
            // Set initial state based on start_enabled setting
            let start_enabled = storage::get_start_enabled(&app.handle());
            VINE_BOOM_ENABLED.store(start_enabled, Ordering::Relaxed);
            
            let window = app.get_webview_window("main").unwrap();
            let window_clone = window.clone();

            window.on_window_event(move |event| {
                if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                    api.prevent_close();
                    window_clone.hide().unwrap();
                }
            });

            let autostart_manager = app.autolaunch();
            let _ = autostart_manager.enable();

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

async fn update(app: tauri::AppHandle) -> tauri_plugin_updater::Result<()> {
    if let Some(update) = app.updater()?.check().await? {
      let mut downloaded = 0;
  
      update
        .download_and_install(
          |chunk_length, content_length| {
            downloaded += chunk_length;
            println!("downloaded {downloaded} from {content_length:?}");
          },
          || {
            println!("download finished");
          },
        )
        .await?;
  
      println!("update installed");
      app.restart();
    }
  
    Ok(())
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
        let mut current_generation = GENERATION.load(Ordering::Relaxed);
        
        loop {
            if VINE_BOOM_ENABLED.load(Ordering::Relaxed) {
                let min = storage::get_min_interval(&app_handle);
                let max = storage::get_max_interval(&app_handle);
                let delay = rng.gen_range(min..=max);
                
                let start_time = std::time::Instant::now();
                while start_time.elapsed().as_secs() < delay {
                    if current_generation != GENERATION.load(Ordering::Relaxed) 
                        || !VINE_BOOM_ENABLED.load(Ordering::Relaxed) {
                        break;
                    }
                    std::thread::sleep(Duration::from_millis(100));
                }

                current_generation = GENERATION.load(Ordering::Relaxed);

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
                current_generation = GENERATION.load(Ordering::Relaxed);
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

pub fn is_autostart_enabled(app_handle: &AppHandle) -> bool {
    #[cfg(desktop)]
    {
        app_handle.autolaunch().is_enabled().unwrap_or(false)
    }
    #[cfg(not(desktop))]
    false
}

pub fn enable_autostart(app_handle: &AppHandle) -> Result<(), String> {
    #[cfg(desktop)]
    {
        app_handle
            .autolaunch()
            .enable()
            .map_err(|e| e.to_string())?;
    }
    Ok(())
}

pub fn disable_autostart(app_handle: &AppHandle) -> Result<(), String> {
    #[cfg(desktop)]
    {
        app_handle
            .autolaunch()
            .disable()
            .map_err(|e| e.to_string())?;
    }
    Ok(())
}
