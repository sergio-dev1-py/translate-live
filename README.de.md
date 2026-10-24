<p align="center">
  <img src="assets/icon.svg" width="96" alt="LiveTranslate ghost icon" />
</p>

<h1 align="center">LiveTranslate</h1>

<p align="center"><strong>Systemaudio in Echtzeit mit Gemini Live übersetzen. Eine schwebende 100×32-Pille, ohne virtuelle Audiokabel.</strong></p>

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
  <strong>Deutsch</strong> ·
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

## Überblick

LiveTranslate erfasst den Ton, den dein Computer bereits abspielt, und übersetzt ihn in Echtzeit mit Gemini Live Translate. Die übersetzte Stimme wird über das gewählte Ausgabegerät wiedergegeben – ohne virtuelles Audiokabel und ohne Zwischenserver.

<p align="center"><img src="docs/settings.png" width="620" alt="LiveTranslate settings" /></p>

## Funktionen

- Echtzeit-Sprach-zu-Sprach-Übersetzung mit Gemini Live Translate.
- Kompakte schwebende Pille (100×32 px): Statusbereich zum Starten/Stoppen, Geist zum Ziehen, Mini-Zahnrad für Einstellungen.
- Native Systemaudio-Aufnahme unter Windows (WASAPI), macOS (ScreenCaptureKit) und Linux (PulseAudio/PipeWire), ohne virtuelle Kabel.
- 22 UI-Sprachen, helles/dunkles Design, Infobereich, globales Tastenkürzel Ctrl+Alt+T und automatische Wiederverbindung.
- Der API-Schlüssel liegt im Systemschlüsselbund; Audio geht direkt an Gemini, ohne Konto oder Telemetrie.

## Installation

Lade die neueste Version von [GitHub Releases](https://github.com/sergio-dev1-py/translate-live/releases/latest) herunter. Die Builds sind nicht signiert; SmartScreen oder Gatekeeper können eine Bestätigung verlangen.

| Plattform | Datei |
|---|---|
| Windows 10/11 | <code>LiveTranslate_&lt;version&gt;_x64-setup.exe</code> |
| macOS 13+ | <code>LiveTranslate_&lt;version&gt;_universal.dmg</code> |
| Linux | .AppImage / .deb |

## Schnellstart

1. LiveTranslate installieren und öffnen; die Pille erscheint über der Taskleiste.
2. API-Schlüssel in [Google AI Studio](https://aistudio.google.com/apikey) erstellen und unter Einstellungen → Allgemein einfügen, Verbindung testen und speichern.
3. Unter Audio Aufnahme- und Ausgabegerät wählen. Audio abspielen und im Statusbereich <code>AUTO→ES</code> anklicken; der grüne Punkt bedeutet live. Strg+Alt+T startet oder stoppt ebenfalls.

## Datenschutz

Der API-Schlüssel wird nur im Systemschlüsselbund gespeichert und nie an das Frontend gesendet. Audio geht direkt an Gemini; es gibt keinen LiveTranslate-Server, keine Konten, Telemetrie oder Aufzeichnungen. Keine Schlüssel oder .env-Dateien committen.

## Plattformen

| Plattform | Aufnahme | Status |
|---|---|---|
| Windows 10/11 | WASAPI-Loopback | Unter Windows 10 21H2 getestet |
| macOS 13+ | ScreenCaptureKit | Implementiert; CI-Build; manueller Test ausstehend |
| Linux | PulseAudio / PipeWire | Implementiert; CI-Build; manueller Test ausstehend |

## Aus dem Quellcode bauen

Benötigt Node.js 22+, stabiles Rust und die plattformspezifischen Abhängigkeiten. Details für macOS/Linux stehen im englischen README.

    git clone https://github.com/sergio-dev1-py/translate-live.git
    cd translate-live
    npm ci
    npm run tauri dev

## Problemlösung

- Kein Ausgabegerät: Gerät neu verbinden, Aktualisieren drücken und unter Audio auswählen.
- macOS: Berechtigung „Screen & System Audio Recording“ erteilen und die App neu starten.
- Linux: PulseAudio oder PipeWire mit pipewire-pulse und libpulse installieren.

## Lizenz

MIT — siehe [LICENSE](LICENSE).

> [Vollständige Dokumentation auf Englisch](README.md).

---

<p align="center">
  <a href="README.md">English</a> ·
  <a href="README.zh-CN.md">简体中文</a> ·
  <a href="README.zh-TW.md">繁體中文</a> ·
  <a href="README.ko.md">한국어</a> ·
  <strong>Deutsch</strong> ·
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
