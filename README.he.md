<p align="center">
  <img src="assets/icon.svg" width="96" alt="LiveTranslate ghost icon" />
</p>

<h1 align="center">LiveTranslate</h1>

<p align="center"><strong>תרגום אודיו של המערכת בזמן אמת עם Gemini Live. גלולה צפה בגודל 100×32, ללא כבלים שמע וירטואליים.</strong></p>

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
  <strong>עברית</strong> ·
  <a href="README.tr.md">Türkçe</a> ·
  <a href="README.uk.md">Українська</a> ·
  <a href="README.hi.md">हिन्दी</a> ·
  <a href="README.el.md">Ελληνικά</a> ·
  <a href="README.vi.md">Tiếng Việt</a>
</p>

<p align="center"><img src="docs/pill.png" width="620" alt="LiveTranslate floating pill" /></p>

<p align="center"><sub>100×32 · LiveTranslate</sub></p>

---

## סקירה

LiveTranslate לוכד את האודיו שהמחשב כבר מנגן ומתרגם אותו בזמן אמת עם Gemini Live Translate. הקול המתורגם מושמע ביציאה שתבחרו, ישירות מהמחשב וללא שרת מתווך.

<p align="center"><img src="docs/settings.png" width="620" alt="LiveTranslate settings" /></p>

## תכונות

- תרגום דיבור לדיבור בזמן אמת עם Gemini Live Translate.
- גלולה צפה קומפקטית (100×32 px): לחצו על אזור המצב להתחלה/עצירה, גררו את הרוח והשתמשו בגלגל השיניים הקטן.
- לכידת אודיו מערכת מקורית ב-Windows (WASAPI), macOS (ScreenCaptureKit) ו-Linux (PulseAudio/PipeWire), ללא כבלים וירטואליים.
- 22 שפות ממשק, ערכת נושא בהירה/כהה, מגש מערכת, קיצור גלובלי Ctrl+Alt+T וחיבור מחדש אוטומטי.
- מפתח ה-API נשמר במנהל האישורים של המערכת; האודיו נשלח ישירות ל-Gemini, ללא חשבון או טלמטריה.

## התקנה

הורידו את הגרסה האחרונה מ-[GitHub Releases](https://github.com/sergio-dev1-py/translate-live/releases/latest). הגרסאות אינן חתומות; SmartScreen או Gatekeeper עשויים לבקש אישור.

| פלטפורמה | קובץ |
|---|---|
| Windows 10/11 | <code>LiveTranslate_&lt;version&gt;_x64-setup.exe</code> |
| macOS 13+ | <code>LiveTranslate_&lt;version&gt;_universal.dmg</code> |
| Linux | .AppImage / .deb |

## התחלה מהירה

1. התקינו ופתחו את LiveTranslate; הגלולה תופיע מעל שורת המשימות.
2. צרו מפתח API ב-[Google AI Studio](https://aistudio.google.com/apikey), הדביקו אותו בהגדרות → כללי, בדקו את החיבור ושמרו.
3. בשמע בחרו לכידה ויציאה. הפעילו אודיו ולחצו על <code>AUTO→ES</code>; נקודה ירוקה מציינת שידור חי. Ctrl+Alt+T גם מתחיל או עוצר.

## פרטיות

מפתח ה-API נשמר רק במנהל האישורים של המערכת ולעולם לא נשלח לחזית. האודיו נשלח ישירות ל-Gemini; אין שרת LiveTranslate, חשבון, טלמטריה או הקלטה. אל תעלו מפתחות או קובצי .env לריפו.

## פלטפורמות

| פלטפורמה | לכידה | מצב |
|---|---|---|
| Windows 10/11 | WASAPI loopback | נבדק ב-Windows 10 21H2 |
| macOS 13+ | ScreenCaptureKit | מיושם; בניית CI; בדיקה ידנית ממתינה |
| Linux | PulseAudio / PipeWire | מיושם; בניית CI; בדיקה ידנית ממתינה |

## בנייה מהמקור

נדרשים Node.js 22+, Rust יציב ותלויות הפלטפורמה. פרטים ל-macOS/Linux נמצאים ב-README באנגלית.

    git clone https://github.com/sergio-dev1-py/translate-live.git
    cd translate-live
    npm ci
    npm run tauri dev

## פתרון בעיות

- אין יציאה: חברו מחדש את ההתקן, לחצו רענון ובחרו אותו בשמע.
- macOS: אשרו Screen & System Audio Recording והפעילו מחדש את האפליקציה.
- Linux: התקינו PulseAudio או PipeWire עם pipewire-pulse ו-libpulse.

## רישיון

MIT — ראו [LICENSE](LICENSE).

> [תיעוד מלא באנגלית](README.md).

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
  <strong>עברית</strong> ·
  <a href="README.tr.md">Türkçe</a> ·
  <a href="README.uk.md">Українська</a> ·
  <a href="README.hi.md">हिन्दी</a> ·
  <a href="README.el.md">Ελληνικά</a> ·
  <a href="README.vi.md">Tiếng Việt</a>
</p>
