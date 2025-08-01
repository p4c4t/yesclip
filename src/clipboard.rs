use anyhow::Result;
use std::path::Path;

pub fn copy_file(path: &Path) -> Result<()> {
    #[cfg(target_os = "windows")]
    {
        use std::ffi::OsStr;
        use std::mem::size_of;
        use std::os::windows::ffi::OsStrExt;

        use windows::Win32::{
            Foundation::HWND,
            System::{
                DataExchange::{OpenClipboard, CloseClipboard, EmptyClipboard, SetClipboardData},
                Memory::{GlobalAlloc, GlobalLock, GlobalUnlock, GMEM_MOVEABLE},
                Ole::CF_HDROP,
            },
            UI::Shell::DROPFILES,
        };

        // build the double-null-terminated utf-16 filename list
        let mut wide: Vec<u16> = OsStr::new(path)
            .encode_wide()
            .chain(std::iter::once(0))
            .chain(std::iter::once(0)) // list terminator
            .collect();

        let bytes_needed = size_of::<DROPFILES>() + wide.len() * 2;

        unsafe {
            // open clipboard
            OpenClipboard(HWND(0))?;
            EmptyClipboard()?;

            // allocate and fill global memory
            let h_mem = GlobalAlloc(GMEM_MOVEABLE, bytes_needed)?;
            
            let ptr = GlobalLock(h_mem) as *mut u8;
            if ptr.is_null() {
                CloseClipboard();
                return Err(anyhow::anyhow!("GlobalLock failed"));
            }

            // header
            let df = &mut *(ptr as *mut DROPFILES);
            df.pFiles = size_of::<DROPFILES>() as u32;
            df.fWide = windows::Win32::Foundation::TRUE;

            // filename data right after header
            let data_ptr = ptr.add(size_of::<DROPFILES>());
            std::ptr::copy_nonoverlapping(wide.as_ptr() as *const u8, data_ptr, wide.len() * 2);

            let _ = GlobalUnlock(h_mem);

            // put it in the clipboard
            use windows::Win32::Foundation::HANDLE;
            SetClipboardData(CF_HDROP.0 as u32, HANDLE(h_mem.0 as isize))?;
            CloseClipboard()?;
        }
        Ok(())
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

