use crate::config::Settings;
use anyhow::Result;
use crossbeam_channel::Sender;
use std::path::PathBuf;

#[derive(Clone)]
pub enum TrayMsg {
    ToggleCopyMode,
    Quit,
}

pub fn init_tray(_settings: Settings, _tx: Sender<TrayMsg>, _icon_path: &PathBuf) -> Result<()> {
    println!("tray initialized (simplified version)");
    Ok(())
}
