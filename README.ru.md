<p align="center">
  <img src="assets/icon.svg" width="96" alt="LiveTranslate ghost icon" />
</p>

<h1 align="center">LiveTranslate</h1>

<p align="center"><strong>Перевод системного звука в реальном времени с Gemini Live. Плавающая пилюля 100×32, без виртуальных аудиокабелей.</strong></p>

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
  <strong>Русский</strong><br/>
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

## Обзор

LiveTranslate захватывает звук, который уже воспроизводится на компьютере, и переводит его в реальном времени с помощью Gemini Live Translate. Переведённый голос воспроизводится на выбранном устройстве вывода — напрямую, без промежуточного сервера.

<p align="center"><img src="docs/settings.png" width="620" alt="LiveTranslate settings" /></p>

## Возможности

- Перевод речи в речь в реальном времени с Gemini Live Translate.
- Компактная плавающая пилюля (100×32 px): нажмите на статус для запуска/остановки, перетащите призрака и откройте настройки мини-шестерёнкой.
- Нативный захват системного звука в Windows (WASAPI), macOS (ScreenCaptureKit) и Linux (PulseAudio/PipeWire), без виртуальных кабелей.
- 22 языка интерфейса, светлая/тёмная тема, системный трей, глобальная горячая клавиша Ctrl+Alt+T и автопереподключение.
- API-ключ хранится в системном хранилище; звук отправляется напрямую в Gemini, без аккаунта и телеметрии.

## Установка

Скачайте последнюю версию со страницы [GitHub Releases](https://github.com/sergio-dev1-py/translate-live/releases/latest). Сборки не подписаны; SmartScreen или Gatekeeper могут запросить подтверждение.

| Платформа | Файл |
|---|---|
| Windows 10/11 | <code>LiveTranslate_&lt;version&gt;_x64-setup.exe</code> |
| macOS 13+ | <code>LiveTranslate_&lt;version&gt;_universal.dmg</code> |
| Linux | .AppImage / .deb |

## Быстрый старт

1. Установите и запустите LiveTranslate — пилюля появится над панелью задач.
2. Создайте API-ключ в [Google AI Studio](https://aistudio.google.com/apikey), вставьте его в «Настройки → Общие», проверьте подключение и сохраните.
3. В разделе «Аудио» выберите устройства захвата и вывода. Включите аудио и нажмите <code>AUTO→ES</code>; зелёная точка означает прямой эфир. Ctrl+Alt+T также запускает и останавливает перевод.

## Конфиденциальность

API-ключ хранится только в системном хранилище и никогда не передаётся во фронтенд. Звук идёт напрямую в Gemini; сервера LiveTranslate, аккаунтов, телеметрии и записей нет. Не коммитьте ключи и файлы .env.

## Платформы

| Платформа | Захват | Статус |
|---|---|---|
| Windows 10/11 | WASAPI loopback | Протестировано на Windows 10 21H2 |
| macOS 13+ | ScreenCaptureKit | Реализовано; CI-сборка; ручное тестирование ожидается |
| Linux | PulseAudio / PipeWire | Реализовано; CI-сборка; ручное тестирование ожидается |

## Сборка из исходников

Требуются Node.js 22+, стабильный Rust и зависимости платформы. Подробности для macOS/Linux — в английском README.

    git clone https://github.com/sergio-dev1-py/translate-live.git
    cd translate-live
    npm ci
    npm run tauri dev

## Решение проблем

- Нет устройства вывода: переподключите его, нажмите «Обновить» и выберите в разделе «Аудио».
- macOS: разрешите Screen & System Audio Recording и перезапустите приложение.
- Linux: установите PulseAudio или PipeWire с pipewire-pulse и libpulse.

## Лицензия

MIT — см. [LICENSE](LICENSE).

> [Полная документация на английском](README.md).

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
  <strong>Русский</strong><br/>
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
