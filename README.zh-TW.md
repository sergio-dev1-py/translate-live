<p align="center">
  <img src="assets/icon.svg" width="96" alt="LiveTranslate ghost icon" />
</p>

<h1 align="center">LiveTranslate</h1>

<p align="center"><strong>使用 Gemini Live 即時翻譯系統音訊。100×32 浮動膠囊，無需虛擬音訊線。</strong></p>

<p align="center">
  <a href="https://github.com/sergio-dev1-py/translate-live/releases/latest"><img src="https://img.shields.io/badge/Download-Latest-22c55e?style=for-the-badge" alt="Download" /></a>
</p>

<p align="center">
  <a href="https://github.com/sergio-dev1-py/translate-live/releases/latest"><img src="https://img.shields.io/github/v/release/sergio-dev1-py/translate-live?style=flat-square&label=release" alt="Release" /></a>
  <img src="https://img.shields.io/badge/platform-Windows%20%7C%20macOS%20%7C%20Linux-4c8bf5?style=flat-square" alt="Platforms" />
  <img src="https://img.shields.io/badge/Tauri-2-24C8DB?logo=tauri&logoColor=white&style=flat-square" alt="Tauri 2" />
  <img src="https://img.shields.io/badge/Gemini-Live-4285F4?logo=google&logoColor=white&style=flat-square" alt="Gemini Live" />
  <a href="LICENSE"><img src="https://img.shields.io/badge/license-MIT-22c55e?style=flat-square" alt="MIT" /></a>
</p>

<p align="center">
  <a href="README.md">English</a> ·
  <a href="README.zh-CN.md">简体中文</a> ·
  <strong>繁體中文</strong> ·
  <a href="README.ko.md">한국어</a> ·
  <a href="README.de.md">Deutsch</a> ·
  <a href="README.es.md">Español</a> ·
  <a href="README.fr.md">Français</a> ·
  <a href="README.it.md">Italiano</a> ·
  <a href="README.da.md">Dansk</a> ·
  <a href="README.ja.md">日本語</a> ·
  <a href="README.pl.md">Polski</a> ·
  <a href="README.ru.md">Русский</a><br/>
  <a href="README.bs.md">Bosanski</a> ·
  <a href="README.ar.md">العربية</a> ·
  <a href="README.no.md">Norsk</a> ·
  <a href="README.pt-BR.md">Português (Brasil)</a> ·
  <a href="README.he.md">עברית</a> ·
  <a href="README.tr.md">Türkçe</a> ·
  <a href="README.uk.md">Українська</a> ·
  <a href="README.hi.md">हिन्दी</a> ·
  <a href="README.el.md">Ελληνικά</a> ·
  <a href="README.vi.md">Tiếng Việt</a>
</p>

<p align="center"><img src="docs/pill.png" width="620" alt="LiveTranslate floating pill" /></p>

<p align="center"><sub>100×32 · LiveTranslate</sub></p>

---

## 簡介

LiveTranslate 擷取電腦正在播放的音訊，並透過 Gemini Live Translate 即時翻譯。翻譯後的聲音會從你選擇的輸出裝置播放，不需要虛擬音訊線，也沒有中間伺服器。

<p align="center"><img src="docs/settings.png" width="620" alt="LiveTranslate settings" /></p>

## 功能

- 使用 Gemini Live Translate 進行即時語音到語音翻譯。
- 100×32 浮動膠囊：點擊狀態區開始/停止，拖曳幽靈圖示，使用迷你齒輪開啟設定。
- Windows（WASAPI）、macOS（ScreenCaptureKit）與 Linux（PulseAudio/PipeWire）原生系統音訊擷取，無需虛擬音訊線。
- 22 種介面語言、淺色/深色主題、系統匣、全域快速鍵 Ctrl+Alt+T 與自動重新連線。
- API key 儲存在系統鑰匙圈中；音訊直接傳送到 Gemini，沒有帳號或遙測。

## 安裝

從 [GitHub Releases](https://github.com/sergio-dev1-py/translate-live/releases/latest) 下載最新版本。建置未簽署，SmartScreen 或 Gatekeeper 可能會要求確認。

| 平台 | 檔案 |
|---|---|
| Windows 10/11 | <code>LiveTranslate_&lt;version&gt;_x64-setup.exe</code> |
| macOS 13+ | <code>LiveTranslate_&lt;version&gt;_universal.dmg</code> |
| Linux | .AppImage / .deb |

## 快速開始

1. 安裝並開啟 LiveTranslate，膠囊會出現在工作列上方。
2. 在 [Google AI Studio](https://aistudio.google.com/apikey) 建立 API key，貼到「設定 → 一般」，測試連線並儲存。
3. 在「音訊」中選擇擷取與輸出裝置。播放音訊並點擊 <code>AUTO→ES</code>；綠點表示正在即時翻譯。Ctrl+Alt+T 也可開始或停止。

## 隱私

API key 只儲存在系統鑰匙圈中，前端永遠無法讀取。音訊直接傳送到 Gemini；沒有 LiveTranslate 伺服器、帳號、遙測或錄音。請勿提交金鑰或 .env 檔案。

## 平台支援

| 平台 | 擷取 | 狀態 |
|---|---|---|
| Windows 10/11 | WASAPI loopback | 已在 Windows 10 21H2 測試 |
| macOS 13+ | ScreenCaptureKit | 已實作；CI 建置；待手動測試 |
| Linux | PulseAudio / PipeWire | 已實作；CI 建置；待手動測試 |

## 從原始碼建置

需要 Node.js 22+、穩定版 Rust 與系統相依套件。macOS/Linux 的詳細步驟請見英文 README。

    git clone https://github.com/sergio-dev1-py/translate-live.git
    cd translate-live
    npm ci
    npm run tauri dev

## 常見問題

- 找不到輸出裝置：重新連接裝置、點擊「重新整理」，然後在「音訊」中選擇它。
- macOS：授予 Screen & System Audio Recording 權限並重新啟動應用程式。
- Linux：安裝 PulseAudio 或含 pipewire-pulse 的 PipeWire，並安裝 libpulse。

## 授權

MIT — 請見 [LICENSE](LICENSE)。

> [完整的英文文件](README.md)。

---

<p align="center">
  <a href="README.md">English</a> ·
  <a href="README.zh-CN.md">简体中文</a> ·
  <strong>繁體中文</strong> ·
  <a href="README.ko.md">한국어</a> ·
  <a href="README.de.md">Deutsch</a> ·
  <a href="README.es.md">Español</a> ·
  <a href="README.fr.md">Français</a> ·
  <a href="README.it.md">Italiano</a> ·
  <a href="README.da.md">Dansk</a> ·
  <a href="README.ja.md">日本語</a> ·
  <a href="README.pl.md">Polski</a> ·
  <a href="README.ru.md">Русский</a><br/>
  <a href="README.bs.md">Bosanski</a> ·
  <a href="README.ar.md">العربية</a> ·
  <a href="README.no.md">Norsk</a> ·
  <a href="README.pt-BR.md">Português (Brasil)</a> ·
  <a href="README.he.md">עברית</a> ·
  <a href="README.tr.md">Türkçe</a> ·
  <a href="README.uk.md">Українська</a> ·
  <a href="README.hi.md">हिन्दी</a> ·
  <a href="README.el.md">Ελληνικά</a> ·
  <a href="README.vi.md">Tiếng Việt</a>
</p>
