# 📋 Clipboard File Copy Tool

A lightweight, standalone Windows desktop utility built with **Rust** and **Dioxus 0.7**. This tool allows you to decode Base64 strings directly from your clipboard and save them as files.

**Note: This is a hobby project created for the purpose of practicing Rust and Dioxus.**

---

## ⚠️ Disclaimer
**USE AT YOUR OWN RISK.** - This software is a learning exercise and has not been fully tested for all use cases.
- The developer is not responsible for any data loss or damages resulting from the use of this tool.

---

## ✨ Features
- **Standalone:** A single `.exe` file.
- **RDP Helper:** Designed to facilitate file transfers in environments where standard copy-paste is restricted.

## 🚀 How to Use
1. Download the latest `clipboard_copy.exe` from the [Releases] tab.
2. Copy a Base64 encoded string to your clipboard.
3. Run the app and click **"Save File from Clipboard"**.
PS. Dioxus for windows depends on webview2. If you have Edge installed, then Dioxus will work fine. If you don't have WebView2, then you can install it through [Microsoft](https://developer.microsoft.com/en-us/microsoft-edge/webview2/?form=MA13LH).

## 🛠️ Tech Stack
- **Framework:** Dioxus 0.7 (Desktop)
- **Language:** Rust
- **Styling:** Tailwind CSS (Embedded)
- **Dependencies:** `arboard`, `base64`, `rfd`, `winres`, `image`

## ⚖️ License
This project is provided under the [MIT License](LICENSE).