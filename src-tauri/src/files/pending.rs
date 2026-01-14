use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::time::Instant;
use std::time::Duration;
use crate::crypto::api::jobs::{try_start_decrypt, try_start_encrypt, JobRegistry};
use crate::files::watch::WatchMode;

pub type PendingMap = HashMap<PathBuf, Instant>;

const QUIET_PERIOD: Duration = Duration::from_secs(2);
const STABILITY_SAMPLES: usize = 4;
const STABILITY_INTERVAL: Duration = Duration::from_millis(200);

fn is_stable(path: &Path) -> std::io::Result<bool> {
    let mut last = None;

    for _ in 0..STABILITY_SAMPLES {
        let meta = std::fs::metadata(path)?;
        let snapshot = (meta.len(), meta.modified()?);

        if let Some(prev) = last {
            if prev != snapshot {
                return Ok(false);
            }
        }

        last = Some(snapshot);
        std::thread::sleep(STABILITY_INTERVAL);
    }

    Ok(true)
}

fn should_skip(path: &Path) -> bool {
    matches!(
        path.extension().and_then(|s| s.to_str()),
        Some("enc" | "tmp")
    )
}

fn job_exists(jobs: &JobRegistry, filename: &str) -> bool {
    jobs.lock()
        .map(|map| map.contains_key(filename))
        .unwrap_or(true)
}

fn try_start_job(
    path: PathBuf,
    watch_path: &Path,
    output_path: &Path,
    pending: &mut PendingMap,
    jobs: &JobRegistry,
    app: &tauri::AppHandle,
    mode: &WatchMode,
) {
    if should_skip(&path) {
        pending.remove(&path);
        return;
    }

    let filename = match path.file_name().and_then(|s| s.to_str()) {
        Some(f) => f.to_string(),
        None => {
            pending.remove(&path);
            return;
        }
    };

    if job_exists(jobs, &filename) {
        pending.remove(&path);
        return;
    }

    match mode {
        WatchMode::Encrypt(req) => {
            let _ = try_start_encrypt(
                app.clone(),
                jobs.clone(),
                watch_path.to_path_buf(),
                output_path.to_path_buf(),
                filename,
                req.clone(),
            );
        }
        WatchMode::Decrypt(key) => {
            if path.extension().and_then(|s| s.to_str()) == Some("enc") {
                let _ = try_start_decrypt(
                    app.clone(),
                    jobs.clone(),
                    watch_path.to_path_buf(),
                    output_path.to_path_buf(),
                    filename,
                    key.clone(),
                );
            }
        }
    }

    pending.remove(&path);
}

pub fn handle_event(
    event: notify::Event,
    pending: &mut PendingMap,
    watch_path: &Path,
    output_path: &Path,
    jobs: &JobRegistry,
    app: &tauri::AppHandle,
    mode: &WatchMode,
) {
    use notify::EventKind;

    match event.kind {
        EventKind::Create(_) | EventKind::Modify(_) => {
            for path in event.paths {
                pending.insert(path, Instant::now());
            }
        }

        _ => {}
    }
}

pub fn process_pending(
    pending: &mut PendingMap,
    watch_path: &Path,
    output_path: &Path,
    jobs: &JobRegistry,
    app: &tauri::AppHandle,
    mode: &WatchMode,
) {
    let now = Instant::now();

    let ready: Vec<PathBuf> = pending
        .iter()
        .filter_map(|(path, last)| {
            if now.duration_since(*last) >= QUIET_PERIOD {
                Some(path.clone())
            } else {
                None
            }
        })
        .collect();

    for path in ready {
        if is_stable(&path).unwrap_or(false) {
            try_start_job(
                path,
                watch_path,
                output_path,
                pending,
                jobs,
                app,
                mode,
            );
        }
    }
}
