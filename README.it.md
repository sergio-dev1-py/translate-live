<p align="center">
  <img src="assets/icon.svg" width="96" alt="LiveTranslate ghost icon" />
</p>

<h1 align="center">LiveTranslate</h1>

<p align="center"><strong>Traduci l’audio di sistema in tempo reale con Gemini Live. Una pillola flottante da 100×32, senza cavi audio virtuali.</strong></p>

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
  <strong>Italiano</strong> ·
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

## Panoramica

LiveTranslate cattura l’audio già in riproduzione sul computer e lo traduce in tempo reale con Gemini Live Translate. La voce tradotta viene riprodotta sull’uscita scelta, direttamente dal computer e senza server intermedi.

<p align="center"><img src="docs/settings.png" width="620" alt="LiveTranslate settings" /></p>

## Funzionalità

- Traduzione voce-voce in tempo reale con Gemini Live Translate.
- Pillola flottante compatta (100×32 px): clicca lo stato per avviare/fermare, trascina il fantasma e usa l’ingranaggio mini.
- Cattura audio di sistema nativa su Windows (WASAPI), macOS (ScreenCaptureKit) e Linux (PulseAudio/PipeWire), senza cavi virtuali.
- 22 lingue dell’interfaccia, temi chiaro/scuro, area di notifica, scorciatoia globale Ctrl+Alt+T e riconnessione automatica.
- La chiave API resta nel portachiavi di sistema; l’audio va direttamente a Gemini, senza account o telemetria.

## Installazione

Scarica l’ultima versione da [GitHub Releases](https://github.com/sergio-dev1-py/translate-live/releases/latest). Le build non sono firmate; SmartScreen o Gatekeeper potrebbero chiedere conferma.

| Piattaforma | File |
|---|---|
| Windows 10/11 | <code>LiveTranslate_&lt;version&gt;_x64-setup.exe</code> |
| macOS 13+ | <code>LiveTranslate_&lt;version&gt;_universal.dmg</code> |
| Linux | .AppImage / .deb |

## Avvio rapido

1. Installa e apri LiveTranslate; la pillola appare sopra la barra delle applicazioni.
2. Crea una chiave API su [Google AI Studio](https://aistudio.google.com/apikey), incollala in Impostazioni → Generale, prova la connessione e salva.
3. In Audio scegli cattura e uscita. Riproduci audio e clicca <code>AUTO→ES</code>; il punto verde indica il live. Ctrl+Alt+T avvia o ferma.

## Privacy

La chiave API è salvata solo nel portachiavi di sistema e non viene mai inviata al frontend. L’audio va direttamente a Gemini; nessun server LiveTranslate, account, telemetria o registrazione. Non committare chiavi o file .env.

## Piattaforme

| Piattaforma | Cattura | Stato |
|---|---|---|
| Windows 10/11 | WASAPI loopback | Testato su Windows 10 21H2 |
| macOS 13+ | ScreenCaptureKit | Implementato; build CI; test manuale in sospeso |
| Linux | PulseAudio / PipeWire | Implementato; build CI; test manuale in sospeso |

## Compila dai sorgenti

Richiede Node.js 22+, Rust stabile e le dipendenze della piattaforma. I dettagli macOS/Linux sono nel README inglese.

    git clone https://github.com/sergio-dev1-py/translate-live.git
    cd translate-live
    npm ci
    npm run tauri dev

## Risoluzione problemi

- Nessuna uscita: ricollega il dispositivo, premi Aggiorna e selezionalo in Audio.
- macOS: concedi l’autorizzazione Screen & System Audio Recording e riavvia l’app.
- Linux: installa PulseAudio o PipeWire con pipewire-pulse e libpulse.

## Licenza

MIT — vedi [LICENSE](LICENSE).

> [Documentazione completa in inglese](README.md).

---

<p align="center">
  <a href="README.md">English</a> ·
  <a href="README.zh-CN.md">简体中文</a> ·
  <a href="README.zh-TW.md">繁體中文</a> ·
  <a href="README.ko.md">한국어</a> ·
  <a href="README.de.md">Deutsch</a> ·
  <a href="README.es.md">Español</a> ·
  <a href="README.fr.md">Français</a> ·
  <strong>Italiano</strong> ·
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
