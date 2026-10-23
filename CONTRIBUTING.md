# Contributing to LiveTranslate

Thanks for helping! LiveTranslate is a Tauri 2 + React + Rust desktop app for real-time system-audio translation with Gemini Live Translate.

## Development setup

1. Install Node.js 22+ and Rust stable.
2. Install platform dependencies:
   - **Windows:** WebView2 Runtime (usually already present) and the MSVC Rust toolchain.
   - **macOS:** `xcode-select --install`.
   - **Linux:** `libwebkit2gtk-4.1-dev libgtk-3-dev libayatana-appindicator3-dev librsvg2-dev patchelf libasound2-dev libpulse-dev libssl-dev build-essential`.
3. `npm install`
4. `npm run tauri dev`

## Before opening a PR

- `npm run build`
- `cd src-tauri && cargo check`
- Test a real translation on the platform(s) you touched.
- If you change the UI, check both light and dark themes and at least one RTL language (Arabic or Hebrew).
- If you add a translation key, add it to **all** files in `src/i18n/locales/` (or keep English fallback and explain why).

## Code structure

- `src/Pill.tsx` — compact floating pill.
- `src/Settings.tsx` — settings window.
- `src/i18n/` — language list, provider and 22 locale JSON files.
- `src-tauri/src/audio/` — platform capture + shared resampler/playback.
- `src-tauri/src/gemini.rs` — Gemini Live WebSocket client.
- `src-tauri/src/tray.rs` — localized tray menu.
- `src-tauri/src/secrets.rs` — OS keychain.

## Platform-specific audio

- **Windows:** WASAPI loopback (`audio/wasapi.rs`).
- **macOS:** ScreenCaptureKit (`audio/macos.rs`), macOS 13+, Screen Recording permission.
- **Linux:** PulseAudio/PipeWire monitor sources via cpal's PulseAudio backend (`audio/linux.rs`).

Please do not introduce a virtual audio cable as a runtime requirement. Virtual cables can be an optional user workaround, but capture must work with the OS APIs.

## Translations

Locale files are flat JSON dictionaries. Use `en.json` as the source of truth. Keep placeholders such as `{language}`, `{code}`, `{rms}`, `{peak}`, `{version}` unchanged.

## License

By contributing, you agree that your contributions are licensed under the MIT License.
