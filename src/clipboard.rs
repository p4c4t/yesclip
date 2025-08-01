use anyhow::Result;
use std::path::Path;

pub fn copy_file(path: &Path) -> Result<()> {
    #[cfg(target_os = "windows")]
    {
        use clipboard_win::set_clipboard_string;
        let path_str = path.to_string_lossy();
        
        match set_clipboard_string(&path_str) {
            Ok(_) => Ok(()),
            Err(e) => {
                eprintln!("error: failed to copy to clipboard: {:?}", e);
                Err(anyhow::anyhow!("clipboard error: {:?}", e))
            }
        }
    }
    #[cfg(not(target_os = "windows"))]
    {
        #[cfg(feature = "unix_clip")]
        arboard::Clipboard::new()?.set_text(path.to_string_lossy())?;
        Ok(())
    }
}

pub fn copy_text(text: &str) -> Result<()> {
    #[cfg(target_os = "windows")]
    {
        use clipboard_win::set_clipboard_string;
        match set_clipboard_string(text) {
            Ok(_) => Ok(()),
            Err(e) => {
                eprintln!("error: failed to copy text to clipboard: {:?}", e);
                Err(anyhow::anyhow!("clipboard error: {:?}", e))
            }
        }
    }
    #[cfg(not(target_os = "windows"))]
    {
        #[cfg(feature = "unix_clip")]
        arboard::Clipboard::new()?.set_text(text.to_owned())?;
        Ok(())
    }
}

