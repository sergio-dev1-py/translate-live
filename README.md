<p align="center">
  <img src="assets/icon.svg" width="112" alt="LiveTranslate ghost icon" />
</p>

<h1 align="center">LiveTranslate</h1>

<p align="center">
  Real-time system-audio translation with <strong>Gemini Live Translate</strong>.<br/>
  A tiny always-on-top pill, a clean settings window, and no virtual audio cable.
</p>

<p align="center">
  <a href="#features">Features</a> ·
  <a href="#install">Install</a> ·
  <a href="#usage">Usage</a> ·
  <a href="#build-from-source">Build</a> ·
  <a href="#architecture">Architecture</a> ·
  <a href="#license">License</a>
</p>

---

## Features

- **Real-time speech-to-speech translation** with `gemini-3.5-live-translate-preview`.
- **Compact floating pill** (240 × 52 px): click to start/stop, drag to move, gear to configure.
- **Transparent around the pill** — only the glass pill is visible.
- **22 UI languages**: English, 简体中文, 繁體中文, 한국어, Deutsch, Español, Français, Italiano, Dansk, 日本語, Polski, Русский, Bosanski, العربية, Norsk, Português (Brasil), עברית, Türkçe, Українська, हिन्दी, Ελληνικά, Tiếng Việt.
- **Light and dark themes**, light by default.
- **Monochrome ghost icon** in the app and in the Windows/macOS/Linux bundles.
- **Refresh device list** while the app is running, so Bluetooth/USB devices connected later appear without restarting.
- **Separate capture and output devices**: capture the speakers, play the translation in headphones or a Bluetooth device.
- **Native system-audio capture** on Windows, macOS and Linux. No VB-Cable, BlackHole, VoiceMeeter or similar.
- API key stored in the OS keychain (Windows Credential Manager, macOS Keychain, Linux Secret Service).
- System tray, global shortcut (`Ctrl+Alt+T`), single instance and auto-reconnect.
- Open source under the MIT license.

> LiveTranslate does not include a server or proxy. Your audio goes directly from your computer to Google's Gemini API using your own API key.

---

## Platform support

| Platform | System audio capture | Status |
|---|---|---|
| **Windows 10/11** | WASAPI loopback | Fully tested on Windows 10 21H2 |
| **macOS 13+** | ScreenCaptureKit (system audio) | Implemented; CI build; manual testing pending |
| **Linux** | PulseAudio / PipeWire monitor sources | Implemented; CI build; manual testing pending |

### Platform notes

**Windows**
- No extra audio driver is required.
- The 3.5 mm jack usually switches the same Realtek endpoint between speakers and headphones. If you want to hear only the translation, use a second independent output (USB/Bluetooth/HDMI) or use a virtual cable such as VB-Cable as the capture sink.
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

---

## Install

Download the latest build from **GitHub Releases**:

- Windows: `LiveTranslate_<version>_x64-setup.exe`
- macOS: `LiveTranslate_<version>_universal.dmg`
- Linux: `.AppImage` or `.deb`

The builds are unsigned. SmartScreen/Gatekeeper may ask for confirmation.

---

## Usage

1. Open a video, meeting or any app with audio.
2. Click the pill (`AUTO → ES`).
3. The pill changes to **LIVE** and the translated voice starts playing.
4. Click again to stop.
5. Use the gear to configure the API key, devices, appearance and language.

### API key

1. Create a key at <https://aistudio.google.com/apikey>.
2. Open **Settings → General**.
3. Paste the key and press **Test connection**.
4. Save.

The key is stored in the OS keychain, never in the source code. You can also set `GEMINI_API_KEY` in the environment for headless testing.

### Audio setup and echo

LiveTranslate captures one output device and plays the translation on another. To avoid hearing the original video and to prevent the model from hearing its own translation:

- **Best**: use two independent outputs. For example:
  - Video → Bluetooth headset or USB adapter (muted or silent)
  - Translation → wired headphones (Realtek)
  - or the other way around.
- **With one physical output**: use a virtual cable such as VB-Cable as the capture sink:
  - Route Chrome/Edge to **CABLE Input**
  - Capture **CABLE Input** in LiveTranslate
  - Play the translation on your headphones
- Do not capture and play on exactly the same endpoint: the translation will be captured again.

The app warns you when capture and output point to the same device.

### UI language vs translation language

These are two independent settings:

- **Appearance → Application language** changes only the menus and texts of LiveTranslate. It asks you to restart the app.
- **General → Translation language** changes the language Gemini translates the audio into. It does not change the app UI.

The original language is detected automatically by the model; the API does not accept a source-language override.

### Devices refresh

Connect Bluetooth or USB audio after opening the app, then press **Refresh** in **Settings → Audio** (or simply reopen/focus the settings window). New devices appear in both capture and output lists without restarting.

---

## Build from source

### Requirements

- Node.js 22+
- Rust stable (1.90+ recommended)
- Platform tools listed below

```bash
git clone <your-repository-url>
cd live-translate
npm install
npm run tauri dev
```

Create an installer/bundle:

```bash
npm run tauri build
```

### Windows

- WebView2 Runtime (included in current Windows 10/11).
- No additional SDK beyond the Rust MSVC toolchain.

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

Backend helpers (debug build has a console):

```bash
cd src-tauri
$env:GEMINI_API_KEY="your-key"   # PowerShell
cargo run -- --test-gemini
cargo run -- --probe-audio
```

On Linux/macOS use `GEMINI_API_KEY=... cargo run -- --test-gemini`.

---

## Architecture

```
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

```
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

---

## Privacy

- Audio is sent **directly** from your computer to the Gemini API.
- No LiveTranslate server, account, telemetry or analytics.
- API keys are stored in the OS keychain.
- Transcripts are only printed to the debug console; they are not saved to disk.

---

## Limits and costs

- Gemini Live Translate sessions are limited to about **15 minutes** without compression. LiveTranslate renews the session at ~13 minutes; expect a ~1 second gap.
- Approximate price: **$0.0368 USD per minute** (~$2.21/hour) at the time of writing. Check Google's current pricing.
- Input language is auto-detected.
- Voice preservation is approximate; background music can cause artifacts.
- The model is a preview; model IDs and limits can change.

---

## Contributing

Issues and pull requests are welcome.

- Keep changes focused and cross-platform.
- Run `npm run build`, `cd src-tauri && cargo check`, and a real translation test before opening a PR.
- For UI translations, edit `src/i18n/locales/<code>.json`. Keep the same key set as `en.json`.
- If you change audio code, test on the affected platform and describe the device/OS version.

See `CONTRIBUTING.md` for more details.

---

## License

MIT — see [LICENSE](LICENSE).

The native capture approach was inspired by the architecture of [`phuc-nt/my-translator`](https://github.com/phuc-nt/my-translator) (MIT). LiveTranslate is an independent implementation focused on Gemini Live Translate and the floating pill.
