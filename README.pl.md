<p align="center">
  <img src="assets/icon.svg" width="96" alt="LiveTranslate ghost icon" />
</p>

<h1 align="center">LiveTranslate</h1>

<p align="center"><strong>Tłumacz dźwięk systemowy w czasie rzeczywistym z Gemini Live. Mała pływająca pigułka 100×32, bez wirtualnych kabli audio.</strong></p>

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
  <strong>Polski</strong> ·
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

## Przegląd

LiveTranslate przechwytuje dźwięk odtwarzany przez komputer i tłumaczy go w czasie rzeczywistym za pomocą Gemini Live Translate. Przetłumaczony głos jest odtwarzany na wybranym wyjściu, bezpośrednio z komputera i bez serwera pośredniczącego.

<p align="center"><img src="docs/settings.png" width="620" alt="LiveTranslate settings" /></p>

## Funkcje

- Tłumaczenie mowy na mowę w czasie rzeczywistym z Gemini Live Translate.
- Kompaktowa pływająca pigułka (100×32 px): kliknij status, aby uruchomić/zatrzymać, przeciągnij duszka i użyj mini zębatki.
- Natywne przechwytywanie dźwięku systemowego w Windows (WASAPI), macOS (ScreenCaptureKit) i Linux (PulseAudio/PipeWire), bez wirtualnych kabli.
- 22 języki interfejsu, motywy jasny/ciemny, zasobnik systemowy, skrót globalny Ctrl+Alt+T i automatyczne ponowne łączenie.
- Klucz API jest przechowywany w systemowym pęku kluczy; dźwięk trafia bezpośrednio do Gemini, bez konta i telemetrii.

## Instalacja

Pobierz najnowszą wersję z [GitHub Releases](https://github.com/sergio-dev1-py/translate-live/releases/latest). Kompilacje nie są podpisane; SmartScreen lub Gatekeeper może poprosić o potwierdzenie.

| Platforma | Plik |
|---|---|
| Windows 10/11 | <code>LiveTranslate_&lt;version&gt;_x64-setup.exe</code> |
| macOS 13+ | <code>LiveTranslate_&lt;version&gt;_universal.dmg</code> |
| Linux | .AppImage / .deb |

## Szybki start

1. Zainstaluj i uruchom LiveTranslate; pigułka pojawi się nad paskiem zadań.
2. Utwórz klucz API w [Google AI Studio](https://aistudio.google.com/apikey), wklej go w Ustawienia → Ogólne, przetestuj połączenie i zapisz.
3. W Audio wybierz przechwytywanie i wyjście. Odtwórz dźwięk i kliknij <code>AUTO→ES</code>; zielona kropka oznacza transmisję na żywo. Ctrl+Alt+T również uruchamia lub zatrzymuje.

## Prywatność

Klucz API jest przechowywany wyłącznie w systemowym pęku kluczy i nigdy nie trafia do frontendu. Dźwięk jest wysyłany bezpośrednio do Gemini; nie ma serwera LiveTranslate, konta, telemetrii ani nagrań. Nie commituj kluczy ani plików .env.

## Platformy

| Platforma | Przechwytywanie | Status |
|---|---|---|
| Windows 10/11 | WASAPI loopback | Przetestowano na Windows 10 21H2 |
| macOS 13+ | ScreenCaptureKit | Zaimplementowano; build CI; test ręczny oczekuje |
| Linux | PulseAudio / PipeWire | Zaimplementowano; build CI; test ręczny oczekuje |

## Kompilacja ze źródeł

Wymaga Node.js 22+, stabilnego Rusta i zależności platformy. Szczegóły dla macOS/Linux są w angielskim README.

    git clone https://github.com/sergio-dev1-py/translate-live.git
    cd translate-live
    npm ci
    npm run tauri dev

## Rozwiązywanie problemów

- Brak wyjścia: podłącz ponownie urządzenie, kliknij Odśwież i wybierz je w Audio.
- macOS: przyznaj uprawnienie Screen & System Audio Recording i uruchom aplikację ponownie.
- Linux: zainstaluj PulseAudio lub PipeWire z pipewire-pulse oraz libpulse.

## Licencja

MIT — zobacz [LICENSE](LICENSE).

> [Pełna dokumentacja po angielsku](README.md).

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
  <strong>Polski</strong> ·
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
