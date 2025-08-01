# yesclip 📋✨

> drop files, get clipboard magic

a minimalist rust tray app that turns file drops into instant clipboard copies. built for developers who live in terminals and love efficient workflows.

## what it does

drop any file into `~/Documents/yesclip/` → instantly available in your clipboard for `ctrl+v` anywhere.

- **images** → paste directly into slack, discord, figma
- **documents** → drop into file explorers, emails, anywhere  
- **text files** → copies content as plain text (configurable)
- **zero friction** → runs silently in system tray

## quick start

```bash
git clone https://github.com/your-username/yesclip
cd yesclip
cargo build --release
./target/release/yesclip
```

that's it. files dropped into `~/Documents/yesclip/` are now clipboard-ready.

## features

- 🪶 **lightweight** - minimal memory footprint, rust performance
- 🎯 **focused** - does one thing exceptionally well  
- 🔧 **configurable** - smart text file handling
- 🖥️ **cross-platform** - windows (primary), linux/mac support incoming
- 🔄 **real-time** - instant file watching with `notify`
- 🎨 **clean** - no ui clutter, just pure functionality

## tech stack

- **rust** - blazing fast, memory safe
- **tokio** - async file watching
- **windows api** - native clipboard integration (`CF_HDROP`)
- **tray-icon** - system tray presence

## why yesclip?

because copying files shouldn't require right-clicks, context menus, or breaking your flow. just drop and paste.

## inspired by

- lack of "share" in windows
- "the art of bodge" by tom scott

---

*built with ❤️ for rust and 😤 for windows. because file management should be effortless.*