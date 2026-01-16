use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::{Arc};
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;
use notify::{Watcher};
use tauri::Emitter;
use crate::files::pending::{handle_event, process_pending, PendingMap};
use crate::files::watch::{WatchMode, WatcherService};
use crate::jobs::JobRegistry;

pub fn start_watcher(
    watch_path: PathBuf,
    output_path: PathBuf,
    app: tauri::AppHandle,
    jobs: JobRegistry,
    mode: WatchMode,
) -> WatcherService {
    let stop = Arc::new(AtomicBool::new(false));
    let stop_thread = stop.clone();

    let handle = std::thread::spawn(move || {
        use std::sync::mpsc::channel;

        let (tx, rx) = channel();
        let mut pending: PendingMap = HashMap::new();

        let mut watcher = match notify::recommended_watcher(move |res| {
            let _ = tx.send(res);
        }) {
            Ok(w) => w,
            Err(e) => {
                let _ = app.emit("fsw:error", e.to_string());
                return;
            }
        };

        if let Err(e) = watcher.watch(&watch_path, notify::RecursiveMode::NonRecursive) {
            let _ = app.emit("fsw:error", e.to_string());
            return;
        }

        while !stop_thread.load(Ordering::SeqCst) {
            match rx.recv_timeout(Duration::from_millis(200)) {
                Ok(Ok(event)) => handle_event(
                    event,
                    &mut pending,
                    &watch_path,
                    &output_path,
                    &jobs,
                    &app,
                    &mode,
                ),
                _ => {}
            }

            process_pending(
                &mut pending,
                &watch_path,
                &output_path,
                &jobs,
                &app,
                &mode,
            );
        }
    });

    WatcherService { stop, handle }
}

