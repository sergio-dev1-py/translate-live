<p align="center">
  <img src="assets/icon.svg" width="96" alt="LiveTranslate ghost icon" />
</p>

<h1 align="center">LiveTranslate</h1>

<p align="center"><strong>Переклад системного звуку в реальному часі з Gemini Live. Плаваюча пігулка 100×32, без віртуальних аудіокабелів.</strong></p>

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
  <a href="README.tr.md">Türkçe</a> ·
  <strong>Українська</strong> ·
  <a href="README.hi.md">हिन्दी</a> ·
  <a href="README.el.md">Ελληνικά</a> ·
  <a href="README.vi.md">Tiếng Việt</a>
</p>

<p align="center"><img src="docs/pill.png" width="620" alt="LiveTranslate floating pill" /></p>

<p align="center"><sub>100×32 · LiveTranslate</sub></p>

---

## Огляд

LiveTranslate захоплює звук, який уже відтворюється на комп'ютері, і перекладає його в реальному часі за допомогою Gemini Live Translate. Перекладений голос відтворюється на вибраному пристрої виводу — напряму, без проміжного сервера.

<p align="center"><img src="docs/settings.png" width="620" alt="LiveTranslate settings" /></p>

## Можливості

- Переклад мовлення в мовлення в реальному часі з Gemini Live Translate.
- Компактна плаваюча пігулка (100×32 px): натисніть на статус для запуску/зупинки, перетягніть привида й відкрийте налаштування міні-шестернею.
- Нативне захоплення системного звуку в Windows (WASAPI), macOS (ScreenCaptureKit) і Linux (PulseAudio/PipeWire), без віртуальних кабелів.
- 22 мови інтерфейсу, світла/темна тема, системний трей, глобальна гаряча клавіша Ctrl+Alt+T і автоперепідключення.
- API-ключ зберігається в системному сховищі; звук надсилається напряму в Gemini, без акаунта й телеметрії.

## Встановлення

Завантажте останню версію зі сторінки [GitHub Releases](https://github.com/sergio-dev1-py/translate-live/releases/latest). Збірки не підписані; SmartScreen або Gatekeeper можуть попросити підтвердження.

| Платформа | Файл |
|---|---|
| Windows 10/11 | <code>LiveTranslate_&lt;version&gt;_x64-setup.exe</code> |
| macOS 13+ | <code>LiveTranslate_&lt;version&gt;_universal.dmg</code> |
| Linux | .AppImage / .deb |

## Швидкий старт

1. Встановіть і запустіть LiveTranslate — пігулка з'явиться над панеллю завдань.
2. Створіть API-ключ у [Google AI Studio](https://aistudio.google.com/apikey), вставте його в «Налаштування → Загальні», перевірте з'єднання й збережіть.
3. У розділі «Аудіо» виберіть пристрої захоплення та виводу. Увімкніть аудіо й натисніть <code>AUTO→ES</code>; зелена точка означає прямий ефір. Ctrl+Alt+T також запускає й зупиняє.

## Конфіденційність

API-ключ зберігається лише в системному сховищі й ніколи не передається у фронтенд. Звук іде напряму в Gemini; сервера LiveTranslate, акаунтів, телеметрії та записів немає. Не комітьте ключі й файли .env.

## Платформи

| Платформа | Захоплення | Статус |
|---|---|---|
| Windows 10/11 | WASAPI loopback | Перевірено на Windows 10 21H2 |
| macOS 13+ | ScreenCaptureKit | Реалізовано; CI-збірка; ручне тестування очікується |
| Linux | PulseAudio / PipeWire | Реалізовано; CI-збірка; ручне тестування очікується |

## Збірка з вихідного коду

Потрібні Node.js 22+, стабільний Rust і залежності платформи. Деталі для macOS/Linux — в англійському README.

    git clone https://github.com/sergio-dev1-py/translate-live.git
    cd translate-live
    npm ci
    npm run tauri dev

## Вирішення проблем

- Немає пристрою виводу: перепідключіть його, натисніть «Оновити» й виберіть у розділі «Аудіо».
- macOS: дозвольте Screen & System Audio Recording і перезапустіть застосунок.
- Linux: встановіть PulseAudio або PipeWire з pipewire-pulse і libpulse.

## Ліцензія

MIT — див. [LICENSE](LICENSE).

> [Повна документація англійською](README.md).

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
  <a href="README.tr.md">Türkçe</a> ·
  <strong>Українська</strong> ·
  <a href="README.hi.md">हिन्दी</a> ·
  <a href="README.el.md">Ελληνικά</a> ·
  <a href="README.vi.md">Tiếng Việt</a>
</p>
