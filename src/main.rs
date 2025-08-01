mod clipboard;
mod config;
mod tray;
mod watcher;

use anyhow::Result;
use config::Settings;
use crossbeam_channel::unbounded;
use std::path::PathBuf;
use tokio::runtime::Runtime;
// use tray::TrayMsg;

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
    let icon_path = std::path::Path::new("assets/yesclip.ico").canonicalize().unwrap_or_else(|_| {
        // fallback to current directory
        std::env::current_dir().unwrap().join("assets/yesclip.ico")
    });
    
    // set up event handlers before creating tray
    use crossbeam_channel::unbounded;
    let (tray_tx, tray_rx) = unbounded();
    let (menu_tx, menu_rx) = unbounded();
    
    // set event handlers that will forward events to our channels
    tray_icon::TrayIconEvent::set_event_handler(Some({
        let tx = tray_tx.clone();
        move |event| {
            println!("tray event handler called: {:?}", event);
            if let Err(e) = tx.send(event) {
                eprintln!("failed to send tray event: {:?}", e);
            }
        }
    }));
    
    tray_icon::menu::MenuEvent::set_event_handler(Some({
        let tx = menu_tx.clone();
        move |event| {
            println!("menu event handler called: {:?}", event);
            if let Err(e) = tx.send(event) {
                eprintln!("failed to send menu event: {:?}", e);
            }
        }
    }));

    let _tray_ctx = tray::init_tray(cfg.clone(), tx.clone(), &icon_path)?;
    rt.block_on(watcher::spawn_watcher(yesclip_dir.clone(), cfg.clone()))?;

    println!("yesclip started! watching directory: {}", yesclip_dir.display());
    println!("drop files into {} to copy them to clipboard", yesclip_dir.display());
    println!("press ctrl+c to quit");
    println!("tray event handlers set up - ready for clicks!");

    // windows message loop - required for tray events to work properly
    #[cfg(target_os = "windows")]
    {
        use windows::Win32::UI::WindowsAndMessaging::{
            DispatchMessageW, GetMessageW, TranslateMessage, MSG,
        };
        use windows::Win32::Foundation::HWND;
        
        println!("starting windows message loop for tray events...");
        
        // spawn a thread to handle our custom events while the main thread handles windows messages
        let tray_rx_clone = tray_rx.clone();
        let menu_rx_clone = menu_rx.clone();
        std::thread::spawn(move || {
            loop {
                // check for tray icon events
                match tray_rx_clone.try_recv() {
                    Ok(event) => {
                        println!(">>> tray event received via handler: {:?}", event);
                        
                        use tray_icon::TrayIconEvent;
                        match event {
                            TrayIconEvent::Click { button, .. } => {
                                println!("tray clicked with button: {:?}", button);
                                let button_str = format!("{:?}", button);
                                if button_str.contains("Left") {
                                    println!("left click detected! showing test message...");
                                }
                            }
                            _ => {
                                println!("other tray event: {:?}", event);
                            }
                        }
                    }
                    Err(_) => {} // no event
                }

                // check for menu events
                match menu_rx_clone.try_recv() {
                    Ok(event) => {
                        println!(">>> menu event received via handler: {:?}", event);
                        println!("menu item id: {}", event.id.0);
                        match event.id.0.as_str() {
                            "quit" => {
                                println!("quit menu item clicked - shutting down...");
                                std::process::exit(0);
                            }
                            _ => {
                                println!("unknown menu item clicked: {}", event.id.0);
                            }
                        }
                    }
                    Err(_) => {} // no event
                }

                std::thread::sleep(std::time::Duration::from_millis(50));
            }
        });
        
        // main thread runs the windows message loop
        loop {
            let mut msg = MSG::default();
            let result = unsafe { GetMessageW(&mut msg, HWND(0), 0, 0) };
            
            if result.0 == -1 {
                eprintln!("error in message loop");
                return Err(anyhow::anyhow!("windows message loop error"));
            } else if result.0 == 0 {
                // WM_QUIT received
                return Ok(());
            } else {
                unsafe {
                    TranslateMessage(&msg);
                    DispatchMessageW(&msg);
                }
            }
        }
    }
    
    #[cfg(not(target_os = "windows"))]
    {
        // non-windows fallback - simple event loop
        loop {
            // check for tray icon events
            match tray_rx.try_recv() {
                Ok(event) => {
                    println!(">>> tray event received via handler: {:?}", event);
                    
                    use tray_icon::TrayIconEvent;
                    match event {
                        TrayIconEvent::Click { button, .. } => {
                            println!("tray clicked with button: {:?}", button);
                            let button_str = format!("{:?}", button);
                            if button_str.contains("Left") {
                                println!("left click detected! showing test message...");
                            }
                        }
                        _ => {
                            println!("other tray event: {:?}", event);
                        }
                    }
                }
                Err(_) => {} // no event
            }

            // check for menu events
            match menu_rx.try_recv() {
                Ok(event) => {
                    println!(">>> menu event received via handler: {:?}", event);
                    println!("menu item id: {}", event.id.0);
                    match event.id.0.as_str() {
                        "quit" => {
                            println!("quit menu item clicked - shutting down...");
                            std::process::exit(0);
                        }
                        _ => {
                            println!("unknown menu item clicked: {}", event.id.0);
                        }
                    }
                }
                Err(_) => {} // no event
            }

            std::thread::sleep(std::time::Duration::from_millis(50));
        }
    }
}
