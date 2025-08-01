mod clipboard;
mod config;
mod tray;
mod watcher;

use anyhow::Result;
use config::Settings;
use crossbeam_channel::unbounded;
use std::path::PathBuf;
use tokio::runtime::Runtime;
use tray::TrayMsg;

fn main() -> Result<()> {
    let rt = Runtime::new()?;
    let yesclip_dir: PathBuf = dirs::document_dir().unwrap().join("yesclip");
    
    match std::fs::create_dir_all(&yesclip_dir) {
        Ok(_) => {},
        Err(e) => {
            eprintln!("error: failed to create directory: {:?}", e);
            return Err(e.into());
        }
    }

    let cfg = Settings::load();
    let (tx, _rx) = unbounded();
    let icon_path = std::env::current_exe()?.with_file_name("assets/yesclip.ico");
    
    tray::init_tray(cfg.clone(), tx.clone(), &icon_path)?;
    rt.block_on(watcher::spawn_watcher(yesclip_dir.clone(), cfg.clone()))?;

    println!("yesclip started! watching directory: {}", yesclip_dir.display());
    println!("drop files into {} to copy them to clipboard", yesclip_dir.display());
    println!("press ctrl+c to quit");

    loop {
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}
