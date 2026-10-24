<p align="center">
  <img src="assets/icon.svg" width="96" alt="LiveTranslate ghost icon" />
</p>

<h1 align="center">LiveTranslate</h1>

<p align="center"><strong>Oversæt systemlyd i realtid med Gemini Live. En flydende pille på 100×32 uden virtuelle lydkabler.</strong></p>

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
  <a href="README.zh-TW.md">繁體中文</a> ·
  <a href="README.ko.md">한국어</a> ·
  <a href="README.de.md">Deutsch</a> ·
  <a href="README.es.md">Español</a> ·
  <a href="README.fr.md">Français</a> ·
  <a href="README.it.md">Italiano</a> ·
  <strong>Dansk</strong> ·
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

## Overblik

LiveTranslate optager den lyd, computeren allerede afspiller, og oversætter den i realtid med Gemini Live Translate. Den oversatte stemme afspilles på den valgte udgang, direkte fra computeren og uden mellemserver.

<p align="center"><img src="docs/settings.png" width="620" alt="LiveTranslate settings" /></p>

## Funktioner

- Realtids tale-til-tale-oversættelse med Gemini Live Translate.
- Kompakt flydende pille (100×32 px): klik på status for at starte/stoppe, træk i spøgelset, og brug mini-tandhjulet.
- Indbygget systemlydoptagelse på Windows (WASAPI), macOS (ScreenCaptureKit) og Linux (PulseAudio/PipeWire), uden virtuelle kabler.
- 22 UI-sprog, lyst/mørkt tema, procesbakke, global genvej Ctrl+Alt+T og automatisk genoprettelse.
- API-nøglen gemmes i systemets nøglering; lyden sendes direkte til Gemini, uden konto eller telemetri.

## Installation

Download den nyeste version fra [GitHub Releases](https://github.com/sergio-dev1-py/translate-live/releases/latest). Builds er ikke signerede; SmartScreen eller Gatekeeper kan bede om bekræftelse.

| Platform | Fil |
|---|---|
| Windows 10/11 | <code>LiveTranslate_&lt;version&gt;_x64-setup.exe</code> |
| macOS 13+ | <code>LiveTranslate_&lt;version&gt;_universal.dmg</code> |
| Linux | .AppImage / .deb |

## Hurtig start

1. Installer og åbn LiveTranslate; pillen vises over proceslinjen.
2. Opret en API-nøgle på [Google AI Studio](https://aistudio.google.com/apikey), indsæt den under Indstillinger → Generelt, test forbindelsen og gem.
3. Vælg optagelse og udgang under Lyd. Afspil lyd, og klik på <code>AUTO→ES</code>; den grønne prik betyder live. Ctrl+Alt+T starter eller stopper også.

## Privatliv

API-nøglen gemmes kun i systemets nøglering og sendes aldrig til frontend. Lyden går direkte til Gemini; der er ingen LiveTranslate-server, konto, telemetri eller optagelse. Commit aldrig nøgler eller .env-filer.

## Platforme

| Platform | Optagelse | Status |
|---|---|---|
| Windows 10/11 | WASAPI loopback | Testet på Windows 10 21H2 |
| macOS 13+ | ScreenCaptureKit | Implementeret; CI-build; manuel test mangler |
| Linux | PulseAudio / PipeWire | Implementeret; CI-build; manuel test mangler |

## Byg fra kildekoden

Kræver Node.js 22+, stabil Rust og platformens afhængigheder. Detaljer til macOS/Linux findes i den engelske README.

    git clone https://github.com/sergio-dev1-py/translate-live.git
    cd translate-live
    npm ci
    npm run tauri dev

## Fejlfinding

- Ingen udgang: tilslut enheden igen, tryk Opdater, og vælg den under Lyd.
- macOS: giv tilladelse til Screen & System Audio Recording, og genstart appen.
- Linux: installér PulseAudio eller PipeWire med pipewire-pulse og libpulse.

## Licens

MIT — se [LICENSE](LICENSE).

> [Fuld dokumentation på engelsk](README.md).

---

<p align="center">
  <a href="README.md">English</a> ·
  <a href="README.zh-CN.md">简体中文</a> ·
  <a href="README.zh-TW.md">繁體中文</a> ·
  <a href="README.ko.md">한국어</a> ·
  <a href="README.de.md">Deutsch</a> ·
  <a href="README.es.md">Español</a> ·
  <a href="README.fr.md">Français</a> ·
  <a href="README.it.md">Italiano</a> ·
  <strong>Dansk</strong> ·
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
