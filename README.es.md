<p align="center">
  <img src="assets/icon.svg" width="96" alt="LiveTranslate ghost icon" />
</p>

<h1 align="center">LiveTranslate</h1>

<p align="center"><strong>Traducción del audio del sistema en tiempo real con Gemini Live. Una píldora flotante de 100×32, sin cables virtuales.</strong></p>

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
  <strong>Español</strong> ·
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

## Descripción

LiveTranslate captura el audio que ya está reproduciendo tu equipo y lo traduce en tiempo real con Gemini Live Translate. La voz traducida se reproduce por la salida que elijas, directamente desde tu equipo y sin servidores intermedios.

<p align="center"><img src="docs/settings.png" width="620" alt="LiveTranslate settings" /></p>

## Características

- Traducción de voz a voz en tiempo real con Gemini Live Translate.
- Píldora flotante de 100×32 px: pulsa el estado para iniciar o parar, arrastra el fantasma y usa el engranaje mini.
- Captura nativa del audio del sistema en Windows (WASAPI), macOS (ScreenCaptureKit) y Linux (PulseAudio/PipeWire), sin cables virtuales.
- 22 idiomas de interfaz, temas claro/oscuro, bandeja del sistema, atajo global Ctrl+Alt+T y reconexión automática.
- La API key se guarda en el llavero del sistema; el audio viaja directamente a Gemini, sin cuentas ni telemetría.

## Instalación

Descarga la última versión desde [GitHub Releases](https://github.com/sergio-dev1-py/translate-live/releases/latest). Las compilaciones no están firmadas; SmartScreen o Gatekeeper pueden pedir confirmación.

| Plataforma | Archivo |
|---|---|
| Windows 10/11 | <code>LiveTranslate_&lt;version&gt;_x64-setup.exe</code> |
| macOS 13+ | <code>LiveTranslate_&lt;version&gt;_universal.dmg</code> |
| Linux | .AppImage / .deb |

## Uso rápido

1. Instala y abre LiveTranslate; la píldora aparece sobre la barra de tareas.
2. Crea una API key en [Google AI Studio](https://aistudio.google.com/apikey) y pégala en Configuración → General; prueba la conexión y guarda.
3. En Audio elige captura y salida. Reproduce audio y pulsa <code>AUTO→ES</code>; el punto verde indica que está en vivo. Ctrl+Alt+T también inicia o detiene.

## Privacidad

La API key se guarda en el llavero del sistema y nunca se envía al frontend. El audio va directamente a Gemini; no hay servidor de LiveTranslate, cuentas, telemetría ni grabaciones. No subas claves ni archivos .env al repositorio.

## Plataformas

| Plataforma | Captura | Estado |
|---|---|---|
| Windows 10/11 | WASAPI loopback | Probado en Windows 10 21H2 |
| macOS 13+ | ScreenCaptureKit | Implementado; CI; prueba manual pendiente |
| Linux | PulseAudio / PipeWire | Implementado; CI; prueba manual pendiente |

## Compilar desde el código

Necesitas Node.js 22+, Rust estable y las dependencias de tu plataforma. Los detalles para macOS y Linux están en el README en inglés.

    git clone https://github.com/sergio-dev1-py/translate-live.git
    cd translate-live
    npm ci
    npm run tauri dev

## Problemas comunes

- No aparece la salida: reconecta el dispositivo, pulsa Actualizar y selecciónalo en Audio.
- macOS: concede el permiso Screen & System Audio Recording y reinicia la app.
- Linux: instala PulseAudio o PipeWire con pipewire-pulse y libpulse.

## Licencia

MIT — consulta [LICENSE](LICENSE).

> [Documentación completa en inglés](README.md).

---

<p align="center">
  <a href="README.md">English</a> ·
  <a href="README.zh-CN.md">简体中文</a> ·
  <a href="README.zh-TW.md">繁體中文</a> ·
  <a href="README.ko.md">한국어</a> ·
  <a href="README.de.md">Deutsch</a> ·
  <strong>Español</strong> ·
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
