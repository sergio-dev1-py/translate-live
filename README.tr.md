<p align="center">
  <img src="assets/icon.svg" width="96" alt="LiveTranslate ghost icon" />
</p>

<h1 align="center">LiveTranslate</h1>

<p align="center"><strong>Gemini Live ile sistem sesini gerçek zamanlı çevirin. 100×32 yüzen bir kapsül, sanal ses kablosu yok.</strong></p>

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
  <a href="README.no.md">Norsk</a> ·
  <a href="README.pt-BR.md">Português (Brasil)</a> ·
  <a href="README.he.md">עברית</a> ·
  <strong>Türkçe</strong> ·
  <a href="README.uk.md">Українська</a> ·
  <a href="README.hi.md">हिन्दी</a> ·
  <a href="README.el.md">Ελληνικά</a> ·
  <a href="README.vi.md">Tiếng Việt</a>
</p>

<p align="center"><img src="docs/pill.png" width="620" alt="LiveTranslate floating pill" /></p>

<p align="center"><sub>100×32 · LiveTranslate</sub></p>

---

## Genel bakış

LiveTranslate, bilgisayarınızın çalmakta olduğu sesi yakalar ve Gemini Live Translate ile gerçek zamanlı olarak çevirir. Çevrilen ses, seçtiğiniz çıkışta doğrudan bilgisayarınızdan oynatılır; aracı sunucu gerekmez.

<p align="center"><img src="docs/settings.png" width="620" alt="LiveTranslate settings" /></p>

## Özellikler

- Gemini Live Translate ile gerçek zamanlı sesten sese çeviri.
- Kompakt yüzen kapsül (100×32 px): durum alanına tıklayarak başlatın/durdurun, hayaleti sürükleyin, mini dişliyi kullanın.
- Windows (WASAPI), macOS (ScreenCaptureKit) ve Linux (PulseAudio/PipeWire) üzerinde yerel sistem sesi yakalama; sanal kablo gerekmez.
- 22 arayüz dili, açık/koyu tema, sistem tepsisi, Ctrl+Alt+T genel kısayolu ve otomatik yeniden bağlanma.
- API anahtarı sistem anahtarlığında saklanır; ses doğrudan Gemini'ye gider, hesap veya telemetri yoktur.

## Kurulum

En son sürümü [GitHub Releases](https://github.com/sergio-dev1-py/translate-live/releases/latest) sayfasından indirin. Derlemeler imzasızdır; SmartScreen veya Gatekeeper onay isteyebilir.

| Platform | Dosya |
|---|---|
| Windows 10/11 | <code>LiveTranslate_&lt;version&gt;_x64-setup.exe</code> |
| macOS 13+ | <code>LiveTranslate_&lt;version&gt;_universal.dmg</code> |
| Linux | .AppImage / .deb |

## Hızlı başlangıç

1. LiveTranslate'i kurun ve açın; kapsül görev çubuğunun üzerinde görünür.
2. [Google AI Studio](https://aistudio.google.com/apikey) üzerinde API anahtarı oluşturun, Ayarlar → Genel bölümüne yapıştırın, bağlantıyı test edin ve kaydedin.
3. Ses bölümünde yakalama ve çıkışı seçin. Ses oynatın ve <code>AUTO→ES</code> öğesine tıklayın; yeşil nokta canlı yayını gösterir. Ctrl+Alt+T de başlatır veya durdurur.

## Gizlilik

API anahtarı yalnızca sistem anahtarlığında saklanır ve ön yüze asla gönderilmez. Ses doğrudan Gemini'ye gider; LiveTranslate sunucusu, hesap, telemetri veya kayıt yoktur. Anahtarları veya .env dosyalarını repoya göndermeyin.

## Platformlar

| Platform | Yakalama | Durum |
|---|---|---|
| Windows 10/11 | WASAPI loopback | Windows 10 21H2 üzerinde test edildi |
| macOS 13+ | ScreenCaptureKit | Uygulandı; CI derlemesi; manuel test bekliyor |
| Linux | PulseAudio / PipeWire | Uygulandı; CI derlemesi; manuel test bekliyor |

## Kaynaktan derleme

Node.js 22+, kararlı Rust ve platform bağımlılıkları gerekir. macOS/Linux ayrıntıları İngilizce README'de.

    git clone https://github.com/sergio-dev1-py/translate-live.git
    cd translate-live
    npm ci
    npm run tauri dev

## Sorun giderme

- Çıkış yok: cihazı yeniden bağlayın, Yenile'ye basın ve Ses bölümünde seçin.
- macOS: Screen & System Audio Recording iznini verin ve uygulamayı yeniden başlatın.
- Linux: PulseAudio veya pipewire-pulse içeren PipeWire ve libpulse kurun.

## Lisans

MIT — bkz. [LICENSE](LICENSE).

> [Tam İngilizce belgeler](README.md).

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
  <a href="README.no.md">Norsk</a> ·
  <a href="README.pt-BR.md">Português (Brasil)</a> ·
  <a href="README.he.md">עברית</a> ·
  <strong>Türkçe</strong> ·
  <a href="README.uk.md">Українська</a> ·
  <a href="README.hi.md">हिन्दी</a> ·
  <a href="README.el.md">Ελληνικά</a> ·
  <a href="README.vi.md">Tiếng Việt</a>
</p>
