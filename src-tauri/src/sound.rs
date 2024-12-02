use rodio::{Decoder, OutputStream, Sink};
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

static LAST_PLAY: AtomicU64 = AtomicU64::new(0);

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

pub fn play_sound_async_debounced(path: String, debounce_ms: u64) {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as u64;

    let last = LAST_PLAY.load(Ordering::Relaxed);
    if now - last > debounce_ms {
        LAST_PLAY.store(now, Ordering::Relaxed);
        play_sound_async(path);
    }
}
