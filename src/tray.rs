use crate::config::Settings;
use anyhow::Result;
use crossbeam_channel::Sender;
use std::path::PathBuf;

#[derive(Clone)]
pub enum TrayMsg {}

pub struct TrayContext {
    _tray: tray_icon::TrayIcon,
    _menu: tray_icon::menu::Menu,
    _toggle: tray_icon::menu::CheckMenuItem,
    _quit: tray_icon::menu::MenuItem,
}

pub fn init_tray(settings: Settings, _tx: Sender<TrayMsg>, icon_path: &PathBuf)
    -> Result<TrayContext>
{
    use tray_icon::{
        menu::{CheckMenuItem, Menu, MenuId, MenuItem},
        Icon, TrayIconBuilder,
    };

    let icon = Icon::from_path(icon_path, None)?;

    let menu = Menu::new();
    let toggle_item = CheckMenuItem::new(
        "copy .md/.txt as plain text",
        true,
        settings.copy_text_files_as_plain,
        None,
    );
    let quit_item = MenuItem::with_id(MenuId::new("quit"), "quit", true, None);

    menu.append(&toggle_item)?;
    menu.append(&quit_item)?;

    let tray = TrayIconBuilder::new()
        .with_tooltip("yesclip")
        .with_icon(icon)
        .with_menu(Box::new(menu.clone()))
        .build()?;

    Ok(TrayContext {
        _tray: tray,
        _menu: menu,
        _toggle: toggle_item,
        _quit: quit_item,
    })
}