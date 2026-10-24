<p align="center">
  <img src="assets/icon.svg" width="96" alt="LiveTranslate ghost icon" />
</p>

<h1 align="center">LiveTranslate</h1>

<p align="center"><strong>Prevod sistemskog zvuka u realnom vremenu uz Gemini Live. Mala plutajuća pilula 100×32, bez virtuelnih audio kablova.</strong></p>

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
  <strong>Bosanski</strong> ·
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

## Pregled

LiveTranslate snima zvuk koji računar već reprodukuje i prevodi ga u realnom vremenu pomoću Gemini Live Translate. Prevedeni glas se reprodukuje na izlazu koji izaberete, direktno sa računara i bez posredničkog servera.

<p align="center"><img src="docs/settings.png" width="620" alt="LiveTranslate settings" /></p>

## Mogućnosti

- Prevođenje govora u govor u realnom vremenu uz Gemini Live Translate.
- Kompaktna plutajuća pilula (100×32 px): kliknite status za početak/zaustavljanje, prevucite duha i koristite mini zupčanik.
- Nativno snimanje sistemskog zvuka na Windowsu (WASAPI), macOS-u (ScreenCaptureKit) i Linuxu (PulseAudio/PipeWire), bez virtuelnih kablova.
- 22 jezika interfejsa, svijetla/tamna tema, sistemska traka, globalna prečica Ctrl+Alt+T i automatsko ponovno povezivanje.
- API ključ se čuva u sistemskom privjesku ključeva; zvuk ide direktno u Gemini, bez naloga ili telemetrije.

## Instalacija

Preuzmite najnoviju verziju sa [GitHub Releases](https://github.com/sergio-dev1-py/translate-live/releases/latest). Buildovi nisu potpisani; SmartScreen ili Gatekeeper mogu zatražiti potvrdu.

| Platforma | Datoteka |
|---|---|
| Windows 10/11 | <code>LiveTranslate_&lt;version&gt;_x64-setup.exe</code> |
| macOS 13+ | <code>LiveTranslate_&lt;version&gt;_universal.dmg</code> |
| Linux | .AppImage / .deb |

## Brzi početak

1. Instalirajte i otvorite LiveTranslate; pilula se pojavljuje iznad trake zadataka.
2. Napravite API ključ na [Google AI Studio](https://aistudio.google.com/apikey), zalijepite ga u Postavke → Opšte, testirajte vezu i sačuvajte.
3. U Audio odaberite uređaje za snimanje i izlaz. Pustite zvuk i kliknite <code>AUTO→ES</code>; zelena tačka označava prijenos uživo. Ctrl+Alt+T takođe pokreće ili zaustavlja.

## Privatnost

API ključ se čuva samo u sistemskom privjesku ključeva i nikada se ne šalje frontendu. Zvuk ide direktno u Gemini; nema LiveTranslate servera, naloga, telemetrije ni snimanja. Ne commitujte ključeve ni .env datoteke.

## Platforme

| Platforma | Snimanje | Status |
|---|---|---|
| Windows 10/11 | WASAPI loopback | Testirano na Windowsu 10 21H2 |
| macOS 13+ | ScreenCaptureKit | Implementirano; CI build; ručno testiranje čeka |
| Linux | PulseAudio / PipeWire | Implementirano; CI build; ručno testiranje čeka |

## Izgradnja iz izvornog koda

Potrebni su Node.js 22+, stabilni Rust i zavisnosti platforme. Detalji za macOS/Linux su u engleskom README-u.

    git clone https://github.com/sergio-dev1-py/translate-live.git
    cd translate-live
    npm ci
    npm run tauri dev

## Rješavanje problema

- Nema izlaznog uređaja: ponovo povežite uređaj, kliknite Osvježi i odaberite ga u Audio.
- macOS: odobrite Screen & System Audio Recording i ponovo pokrenite aplikaciju.
- Linux: instalirajte PulseAudio ili PipeWire sa pipewire-pulse i libpulse.

## Licenca

MIT — pogledajte [LICENSE](LICENSE).

> [Potpuna dokumentacija na engleskom](README.md).

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
  <strong>Bosanski</strong> ·
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
