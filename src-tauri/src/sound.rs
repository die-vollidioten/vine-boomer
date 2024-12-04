use rand::Rng;
use rodio::{Decoder, OutputStream, Sink};
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};
use crate::statistics;
use tauri::AppHandle;

static LAST_PLAY: AtomicU64 = AtomicU64::new(0);
const RARE_SOUND_CHANCE: f64 = 0.10; // 10% chance

pub fn play_sound(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let (_stream, stream_handle) = OutputStream::try_default()?;
    let sink = Sink::try_new(&stream_handle)?;
    let file = BufReader::new(File::open(Path::new(path))?);
    let source = Decoder::new(file)?;
    sink.append(source);
    sink.sleep_until_end();
    Ok(())
}

pub fn play_sound_async(path: String) {
    std::thread::spawn(move || {
        if let Err(e) = play_sound(&path) {
            eprintln!("Error playing sound: {}", e);
        }
    });
}

pub fn play_sound_async_debounced(path: String, debounce_ms: u64, allow_rare: bool, app: &AppHandle) {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as u64;

    let last = LAST_PLAY.load(Ordering::Relaxed);
    if now - last > debounce_ms {
        LAST_PLAY.store(now, Ordering::Relaxed);
        
        if allow_rare {
            let mut rng = rand::thread_rng();
            if rng.gen_bool(RARE_SOUND_CHANCE) {
                let rare_sounds = ["vine-bass.mp3", "vine-sirene.mp3", "vine-surprise.mp3"];
                let chosen = rare_sounds[rng.gen_range(0..rare_sounds.len())];
                
                let base_path = Path::new(&path).parent().unwrap_or(Path::new(""));
                let rare_path = base_path.join(chosen);
                
                if rare_path.exists() {
                    let rare_path_str = rare_path.to_string_lossy().into_owned();
                    statistics::record_boom(app, true, rare_path_str.clone());
                    play_sound_async(rare_path_str);
                    return;
                }
            }
        }
        
        statistics::record_boom(app, false, path.clone());
        play_sound_async(path);
    }
}
