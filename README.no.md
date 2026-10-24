<p align="center">
  <img src="assets/icon.svg" width="96" alt="LiveTranslate ghost icon" />
</p>

<h1 align="center">LiveTranslate</h1>

<p align="center"><strong>Oversett systemlyd i sanntid med Gemini Live. En liten flytende pille på 100×32, uten virtuelle lydkabler.</strong></p>

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
  <a href="README.da.md">Dansk</a> ·
  <a href="README.ja.md">日本語</a> ·
  <a href="README.pl.md">Polski</a> ·
  <a href="README.ru.md">Русский</a><br/>
  <a href="README.bs.md">Bosanski</a> ·
  <a href="README.ar.md">العربية</a> ·
  <strong>Norsk</strong> ·
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

## Oversikt

LiveTranslate tar opp lyden datamaskinen allerede spiller av, og oversetter den i sanntid med Gemini Live Translate. Den oversatte stemmen spilles av på valgt utgang, direkte fra datamaskinen og uten mellomserver.

<p align="center"><img src="docs/settings.png" width="620" alt="LiveTranslate settings" /></p>

## Funksjoner

- Sanntids tale-til-tale-oversettelse med Gemini Live Translate.
- Kompakt flytende pille (100×32 px): klikk på status for å starte/stoppe, dra i spøkelset og bruk mini-tannhjulet.
- Innebygd opptak av systemlyd på Windows (WASAPI), macOS (ScreenCaptureKit) og Linux (PulseAudio/PipeWire), uten virtuelle kabler.
- 22 UI-språk, lyst/mørkt tema, systemkurv, global hurtigtast Ctrl+Alt+T og automatisk gjenoppkobling.
- API-nøkkelen lagres i systemets nøkkelring; lyden sendes direkte til Gemini, uten konto eller telemetri.

## Installasjon

Last ned nyeste versjon fra [GitHub Releases](https://github.com/sergio-dev1-py/translate-live/releases/latest). Byggene er ikke signert; SmartScreen eller Gatekeeper kan be om bekreftelse.

| Plattform | Fil |
|---|---|
| Windows 10/11 | <code>LiveTranslate_&lt;version&gt;_x64-setup.exe</code> |
| macOS 13+ | <code>LiveTranslate_&lt;version&gt;_universal.dmg</code> |
| Linux | .AppImage / .deb |

## Hurtigstart

1. Installer og åpne LiveTranslate; pillen vises over oppgavelinjen.
2. Lag en API-nøkkel på [Google AI Studio](https://aistudio.google.com/apikey), lim den inn under Innstillinger → Generelt, test tilkoblingen og lagre.
3. Velg opptak og utgang under Lyd. Spill av lyd og klikk på <code>AUTO→ES</code>; den grønne prikken betyr live. Ctrl+Alt+T starter eller stopper også.

## Personvern

API-nøkkelen lagres bare i systemets nøkkelring og sendes aldri til frontend. Lyden går direkte til Gemini; det finnes ingen LiveTranslate-server, konto, telemetri eller opptak. Ikke commit nøkler eller .env-filer.

## Plattformer

| Plattform | Opptak | Status |
|---|---|---|
| Windows 10/11 | WASAPI loopback | Testet på Windows 10 21H2 |
| macOS 13+ | ScreenCaptureKit | Implementert; CI-bygg; manuell test gjenstår |
| Linux | PulseAudio / PipeWire | Implementert; CI-bygg; manuell test gjenstår |

## Bygg fra kildekoden

Krever Node.js 22+, stabil Rust og plattformavhengigheter. Detaljer for macOS/Linux finnes i den engelske README-en.

    git clone https://github.com/sergio-dev1-py/translate-live.git
    cd translate-live
    npm ci
    npm run tauri dev

## Feilsøking

- Ingen utgang: koble til enheten på nytt, trykk Oppdater og velg den under Lyd.
- macOS: gi tillatelse til Screen & System Audio Recording og start appen på nytt.
- Linux: installer PulseAudio eller PipeWire med pipewire-pulse og libpulse.

## Lisens

MIT — se [LICENSE](LICENSE).

> [Full dokumentasjon på engelsk](README.md).

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
  <a href="README.da.md">Dansk</a> ·
  <a href="README.ja.md">日本語</a> ·
  <a href="README.pl.md">Polski</a> ·
  <a href="README.ru.md">Русский</a><br/>
  <a href="README.bs.md">Bosanski</a> ·
  <a href="README.ar.md">العربية</a> ·
  <strong>Norsk</strong> ·
  <a href="README.pt-BR.md">Português (Brasil)</a> ·
  <a href="README.he.md">עברית</a> ·
  <a href="README.tr.md">Türkçe</a> ·
  <a href="README.uk.md">Українська</a> ·
  <a href="README.hi.md">हिन्दी</a> ·
  <a href="README.el.md">Ελληνικά</a> ·
  <a href="README.vi.md">Tiếng Việt</a>
</p>
