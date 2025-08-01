mod clipboard;
mod config;
mod tray;
mod watcher;

use anyhow::Result;
use config::Settings;
use std::path::PathBuf;
use tokio::runtime::Runtime;

fn main() -> Result<()> {
    let rt = Runtime::new()?;
    let yesclip_dir: PathBuf = dirs::document_dir().unwrap().join("yesclip");
    
    std::fs::create_dir_all(&yesclip_dir)?;
    let cfg = Settings::load();
    let (tx, _rx) = crossbeam_channel::unbounded();
    let icon_path = std::path::Path::new("assets/yesclip.ico").canonicalize().unwrap_or_else(|_| {
        std::env::current_dir().unwrap().join("assets/yesclip.ico")
    });
    
    let (tray_tx, tray_rx) = crossbeam_channel::unbounded();
    let (menu_tx, menu_rx) = crossbeam_channel::unbounded();
    
    tray_icon::TrayIconEvent::set_event_handler(Some({
        let tx = tray_tx.clone();
        move |event| {
            if let Err(e) = tx.send(event) {
                eprintln!("failed to send tray event: {:?}", e);
            }
        }
    }));
    
    tray_icon::menu::MenuEvent::set_event_handler(Some({
        let tx = menu_tx.clone();
        move |event| {
            if let Err(e) = tx.send(event) {
                eprintln!("failed to send menu event: {:?}", e);
            }
        }
    }));

    let _tray_ctx = tray::init_tray(cfg.clone(), tx.clone(), &icon_path)?;
    rt.block_on(watcher::spawn_watcher(yesclip_dir.clone(), cfg.clone()))?;

    println!("yesclip started! watching directory: {}", yesclip_dir.display());

    #[cfg(target_os = "windows")]
    {
        use windows::Win32::UI::WindowsAndMessaging::{
            DispatchMessageW, GetMessageW, TranslateMessage, MSG,
        };
        use windows::Win32::Foundation::HWND;
        
        let tray_rx_clone = tray_rx.clone();
        let menu_rx_clone = menu_rx.clone();
        std::thread::spawn(move || {
            loop {
                match tray_rx_clone.try_recv() {
                    Ok(event) => {
                        use tray_icon::TrayIconEvent;
                        match event {
                            TrayIconEvent::Click { .. } => {}
                            _ => {}
                        }
                    }
                    Err(_) => {}
                }

                match menu_rx_clone.try_recv() {
                    Ok(event) => {
                        match event.id.0.as_str() {
                            "quit" => std::process::exit(0),
                            _ => {}
                        }
                    }
                    Err(_) => {}
                }

                std::thread::sleep(std::time::Duration::from_millis(50));
            }
        });
        
        loop {
            let mut msg = MSG::default();
            let result = unsafe { GetMessageW(&mut msg, HWND(0), 0, 0) };
            
            if result.0 == -1 {
                return Err(anyhow::anyhow!("windows message loop error"));
            } else if result.0 == 0 {
                return Ok(());
            } else {
                unsafe {
                    let _ = TranslateMessage(&msg);
                    DispatchMessageW(&msg);
                }
            }
        }
    }
    
    #[cfg(not(target_os = "windows"))]
    {
        loop {
            match tray_rx.try_recv() {
                Ok(event) => {
                    use tray_icon::TrayIconEvent;
                    match event {
                        TrayIconEvent::Click { .. } => {}
                        _ => {}
                    }
                }
                Err(_) => {}
            }

            match menu_rx.try_recv() {
                Ok(event) => {
                    match event.id.0.as_str() {
                        "quit" => std::process::exit(0),
                        _ => {}
                    }
                }
                Err(_) => {}
            }

            std::thread::sleep(std::time::Duration::from_millis(50));
        }
    }
}