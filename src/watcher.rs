use crate::{clipboard, config::Settings};
use anyhow::Result;
use notify::{EventKind, RecursiveMode, Watcher};
use std::{path::PathBuf, sync::mpsc::channel};
use tokio::task;

pub async fn spawn_watcher(watch_dir: PathBuf, cfg: Settings) -> Result<()> {
    task::spawn_blocking(move || {
        let (tx, rx) = channel();

        let mut watcher = match notify::recommended_watcher(tx) {
            Ok(w) => w,
            Err(e) => {
                eprintln!("error: failed to create watcher: {:?}", e);
                return;
            }
        };

        match watcher.watch(&watch_dir, RecursiveMode::NonRecursive) {
            Ok(_) => {},
            Err(e) => {
                eprintln!("error: failed to watch directory: {:?}", e);
                return;
            }
        }

        loop {
            let evt = match rx.recv() {
                Ok(e) => e,
                Err(e) => {
                    eprintln!("error: failed to receive event: {:?}", e);
                    continue;
                }
            };

            match evt {
                Ok(event) => {
                    match event.kind {
                        EventKind::Create(_) => {
                            for p in event.paths {
                                if p.is_file() {
                                    if let Err(e) = handle_new_file(&p, &cfg) {
                                        eprintln!("error: failed to handle file: {e:?}");
                                    }
                                }
                            }
                        },
                        EventKind::Modify(_) => {
                            for p in event.paths {
                                if p.is_file() {
                                    if let Err(e) = handle_new_file(&p, &cfg) {
                                        eprintln!("error: failed to handle modified file: {e:?}");
                                    }
                                }
                            }
                        },
                        _ => {}
                    }
                },
                Err(e) => {
                    eprintln!("error: file watcher error: {:?}", e);
                }
            }
        }
    });
    Ok(())
}

fn handle_new_file(src: &PathBuf, cfg: &Settings) -> Result<()> {
    let ext = src.extension().and_then(|e| e.to_str()).unwrap_or_default().to_ascii_lowercase();

    if cfg.copy_text_files_as_plain && (ext == "txt" || ext == "md") {
        let text = std::fs::read_to_string(src)?;
        clipboard::copy_text(&text)?;
        println!("copied text content to clipboard");
    } else {
        clipboard::copy_file(src)?;
        println!("copied file to clipboard (cf_hdrop): {}", src.display());
    }

    Ok(())
}