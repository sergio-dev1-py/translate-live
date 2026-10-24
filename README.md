<p align="center">
  <img src="assets/icon.svg" width="96" alt="LiveTranslate ghost icon" />
</p>

<h1 align="center">LiveTranslate</h1>

<p align="center">
  <strong>Real-time system-audio translation with Gemini Live.</strong><br/>
  A tiny 100×32 always-on-top pill. No virtual audio cables.
</p>

<p align="center">
  <a href="https://github.com/sergio-dev1-py/translate-live/releases/latest">
    <img src="https://img.shields.io/badge/Download-Latest-22c55e?style=for-the-badge" alt="Download the latest release" />
  </a>
</p>

<p align="center">
  <a href="https://github.com/sergio-dev1-py/translate-live/releases/latest"><img src="https://img.shields.io/github/v/release/sergio-dev1-py/translate-live?style=flat-square&label=release" alt="Release" /></a>
  <img src="https://img.shields.io/badge/platform-Windows%20%7C%20macOS%20%7C%20Linux-4c8bf5?style=flat-square" alt="Windows, macOS and Linux" />
  <img src="https://img.shields.io/badge/Tauri-2-24C8DB?logo=tauri&logoColor=white&style=flat-square" alt="Tauri 2" />
  <img src="https://img.shields.io/badge/Gemini-Live-4285F4?logo=google&logoColor=white&style=flat-square" alt="Gemini Live" />
  <a href="LICENSE"><img src="https://img.shields.io/badge/license-MIT-22c55e?style=flat-square" alt="MIT license" /></a>
</p>

<p align="center">
  <strong>English</strong> ·
  <a href="README.zh-CN.md">简体中文</a> ·
  <a href="README.zh-TW.md">繁體中文</a> ·
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

<p align="center">
  <img src="docs/pill.png" width="620" alt="LiveTranslate floating pill" />
</p>

<p align="center"><sub>100×32 px · drag the ghost · mini gear for settings · transparent around the pill</sub></p>

---

<p align="center">
  <a href="#features">Features</a> ·
  <a href="#install">Install</a> ·
  <a href="#quick-start">Quick start</a> ·
  <a href="#platform-support">Platforms</a> ·
  <a href="#build-from-source">Build</a> ·
  <a href="#architecture">Architecture</a> ·
  <a href="#license">License</a>
</p>

## A tiny pill for a big problem

LiveTranslate captures the audio your computer is already playing and translates it to another language in real time with **Gemini Live Translate**. The translated voice is played through the output device you choose.

No virtual audio cable, no browser extension and no account on a third-party server: the Rust backend talks directly to Gemini using your own Google API key.

<br/>

<p align="center"><img src="docs/settings.png" width="640" alt="LiveTranslate settings window" /></p>

## Features

- **Real-time speech-to-speech translation** with `gemini-3.5-live-translate-preview`.
- **Compact floating pill** (100 × 32 px): click the language/status area to start or stop, drag the ghost to move it, and use the mini gear to configure.
- **Transparent around the pill** — only the glass pill is visible.
- **22 UI languages**: English, 简体中文, 繁體中文, 한국어, Deutsch, Español, Français, Italiano, Dansk, 日本語, Polski, Русский, Bosanski, العربية, Norsk, Português (Brasil), עברית, Türkçe, Українська, हिन्दी, Ελληνικά, Tiếng Việt.
- **Light and dark themes**, light by default.
- **Monochrome ghost icon** in the app and in the Windows, macOS and Linux bundles.
- **Refresh device list** while the app is running, so Bluetooth/USB devices connected later appear without restarting.
- **Separate capture and output devices**: capture the speakers, play the translation in headphones, a Bluetooth device or an HDMI output.
- **Native system-audio capture** on Windows, macOS and Linux. No VB-Cable, BlackHole, VoiceMeeter or similar.
- **API key stored in the OS keychain** (Windows Credential Manager, macOS Keychain, Linux Secret Service).
- **System tray**, global shortcut (`Ctrl+Alt+T`), single instance and automatic reconnection.
- Open source under the **MIT license**.

> LiveTranslate does not include a server or proxy. Your audio goes directly from your computer to Google's Gemini API using your own API key.

## Install

Download the latest build from **[GitHub Releases](https://github.com/sergio-dev1-py/translate-live/releases/latest)**:

| Platform | Download |
|---|---|
| **Windows 10/11** | `LiveTranslate_<version>_x64-setup.exe` |
| **macOS 13+** | `LiveTranslate_<version>_universal.dmg` |
| **Linux** | `.AppImage` or `.deb` |

The builds are unsigned. Windows SmartScreen and macOS Gatekeeper may ask for confirmation the first time. On macOS, right-click the app and choose **Open**.

If you build the Windows portable executable yourself, keep `WebView2Loader.dll` next to `LiveTranslate.exe`.

## Quick start

1. **Install** LiveTranslate and open it. The pill appears above the taskbar.
2. Create a Google API key at <https://aistudio.google.com/apikey>.
3. Open the mini gear → **General**, paste the key, press **Test connection**, then **Save changes**.
4. Open **Audio** and choose the capture and output devices. Press **Refresh** after connecting Bluetooth or USB audio.
5. Play a video or meeting, click the language/status area of the pill (`AUTO → ES`), and wait for the status dot to turn green.
6. Click again to stop. Use the tray icon to open the app or quit.

The global shortcut **Ctrl+Alt+T** starts or stops the translation from anywhere.

## API key and privacy

- Create the key at <https://aistudio.google.com/apikey>.
- The key is stored in the **OS keychain**, never in the repository or the settings file.
- The frontend never receives the key.
- Audio is sent **directly** from your computer to the Gemini API. There is no LiveTranslate server, account, telemetry or analytics.
- For headless testing you can set `GEMINI_API_KEY` in the environment.

Never commit an API key, a `.env` file or a captured settings file. See [SECURITY.md](SECURITY.md).

## Audio setup and echo

LiveTranslate captures one output device and plays the translation on another. Use two independent outputs to avoid feedback:

- **Recommended**: video → Bluetooth/USB headset (muted or silent); translation → wired headphones (Realtek). Or the other way around.
- **With one physical output**: use a virtual cable such as VB-Cable as the capture sink, route the video app to **CABLE Input**, capture **CABLE Input** in LiveTranslate, and play the translation on your headphones.
- Do not capture and play on exactly the same endpoint: the translated voice will be captured again.

The app warns you when capture and output point to the same device.

## UI language vs translation language

These are two independent settings:

- **Appearance → Application language** changes the menus and texts of LiveTranslate. It asks you to restart the app.
- **General → Translation language** changes the language Gemini translates the audio into. It does not change the app UI.

The original language is detected automatically by the model; the API does not accept a source-language override.

## Platform support

| Platform | System audio capture | Status |
|---|---|---|
| **Windows 10/11** | WASAPI loopback | Fully tested on Windows 10 21H2 |
| **macOS 13+** | ScreenCaptureKit (system audio) | Implemented; CI build; manual testing pending |
| **Linux** | PulseAudio / PipeWire monitor sources | Implemented; CI build; manual testing pending |

### Platform notes

**Windows**

- No extra audio driver is required.
- The 3.5 mm jack usually switches the same Realtek endpoint between speakers and headphones. If you want to hear only the translation, use a second independent output (USB/Bluetooth/HDMI) or a virtual cable as the capture sink.
- Windows 10 21H2 is supported through WASAPI loopback.

**macOS**

- Requires macOS 13 (Ventura) or newer.
- The first time you start a translation, macOS asks for **Screen & System Audio Recording** permission. Enable it in **System Settings → Privacy & Security → Screen & System Audio Recording** and restart the app if needed.
- The app excludes its own audio from the capture (`excludesCurrentProcessAudio`), which reduces feedback.
- Builds are unsigned. Right-click the `.app`/`.dmg` and choose **Open** the first time.

**Linux**

- Requires PulseAudio or PipeWire with the PulseAudio compatibility layer (`pipewire-pulse`).
- The app lists monitor sources (for example `Monitor of Built-in Audio Analog Stereo`). Select one as the capture device.
- On Wayland, always-on-top, global shortcuts and tray behavior depend on the compositor.
- The `.deb` depends on `libpulse0`; the AppImage expects the host to provide `libpulse`.

## Build from source

### Requirements

- Node.js 22+
- Rust stable
- Platform tools listed below

```bash
git clone https://github.com/sergio-dev1-py/translate-live.git
cd translate-live
npm ci
npm run tauri dev
```

Create an installer/bundle:

```bash
npm run tauri build
```

### Windows

- WebView2 Runtime (included in current Windows 10/11).
- Visual Studio Build Tools with the **Desktop development with C++** workload (MSVC linker and Windows SDK).
- No additional audio SDK.

### macOS

```bash
xcode-select --install
rustup target add aarch64-apple-darwin x86_64-apple-darwin
npm run tauri build -- --target universal-apple-darwin --bundles dmg
```

- Minimum macOS: 13.0.
- Add `NSScreenCaptureUsageDescription` in `src-tauri/Info.plist` if you fork and change the permission text.
- Code signing/notarization are not configured; see Tauri's macOS signing guide.

### Linux (Ubuntu/Debian)

```bash
sudo apt-get update
sudo apt-get install -y \
  libwebkit2gtk-4.1-dev libgtk-3-dev libayatana-appindicator3-dev \
  librsvg2-dev patchelf libasound2-dev libpulse-dev libssl-dev \
  build-essential curl wget file
npm run tauri build -- --bundles appimage,deb
```

### Tests

Backend helpers (the debug build has a console):

```bash
cd src-tauri
$env:GEMINI_API_KEY="your-key"   # PowerShell
cargo run -- --test-gemini
cargo run -- --probe-audio
```

On Linux/macOS use `GEMINI_API_KEY=... cargo run -- --test-gemini`.

## Architecture

```text
System audio
  ├─ Windows: WASAPI loopback
  ├─ macOS:   ScreenCaptureKit
  └─ Linux:   PulseAudio / PipeWire monitor
        │
        ▼
  mono 16 kHz PCM (40 ms chunks)
        │
        ▼
  Gemini Live Translate WebSocket
        │
        ▼
  PCM 24 kHz → cpal output device
```

- **Rust backend**: capture, resampling, Gemini WebSocket, playback, settings, keychain, tray.
- **React + TypeScript frontend**: pill, settings, i18n, themes.
- Audio never passes through the WebView, so it keeps working when the settings window is hidden.
- The API key is never exposed to the frontend.

Important modules:

```text
src-tauri/src/
  audio/
    wasapi.rs       Windows WASAPI loopback
    macos.rs        macOS ScreenCaptureKit
    linux.rs        Linux PulseAudio/PipeWire monitor
    playback.rs     cpal playback + ring buffer
    processor.rs    shared resampling/chunking/level meter
    resample.rs     streaming resampler
  gemini.rs         Gemini Live WebSocket client
  commands.rs       Tauri commands
  settings.rs       settings.json
  secrets.rs        OS keychain
  tray.rs           localized tray menu
  lib.rs            windows, tray, global shortcut
src/
  Pill.tsx          compact floating pill
  Settings.tsx      settings window
  i18n/             22 locale files
  styles.css        light/dark design system
```

## Privacy

- Audio is sent **directly** from your computer to the Gemini API.
- No LiveTranslate server, account, telemetry or analytics.
- API keys are stored in the OS keychain.
- Transcripts are only printed to the debug console; they are not saved to disk.

## Limits and costs

- Gemini Live Translate sessions are limited to about **15 minutes** without compression. LiveTranslate renews the session at ~13 minutes; expect a ~1 second gap.
- Approximate price: **$0.0368 USD per minute** (~$2.21/hour) at the time of writing. Check Google's current pricing.
- Input language is auto-detected.
- Voice preservation is approximate; background music can cause artifacts.
- The model is a preview; model IDs and limits can change.

## Contributing

Issues and pull requests are welcome.

- Keep changes focused and cross-platform.
- Run `npm run build`, `cd src-tauri && cargo check`, and a real translation test before opening a PR.
- For UI translations, edit `src/i18n/locales/<code>.json`. Keep the same key set as `en.json`.
- If you change audio code, test on the affected platform and describe the device/OS version.

See [CONTRIBUTING.md](CONTRIBUTING.md) for more details.

## License

MIT — see [LICENSE](LICENSE).

The native capture approach was inspired by the architecture of [`phuc-nt/my-translator`](https://github.com/phuc-nt/my-translator) (MIT). LiveTranslate is an independent implementation focused on Gemini Live Translate and the floating pill.

---

<p align="center">
  <strong>English</strong> ·
  <a href="README.zh-CN.md">简体中文</a> ·
  <a href="README.zh-TW.md">繁體中文</a> ·
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
