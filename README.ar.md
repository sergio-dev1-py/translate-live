<p align="center">
  <img src="assets/icon.svg" width="96" alt="LiveTranslate ghost icon" />
</p>

<h1 align="center">LiveTranslate</h1>

<p align="center"><strong>ترجمة صوت النظام في الوقت الفعلي باستخدام Gemini Live. كبسولة عائمة بحجم 100×32، دون كابلات صوتية افتراضية.</strong></p>

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
  <strong>العربية</strong> ·
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

## نظرة عامة

يلتقط LiveTranslate الصوت الذي يشغّله حاسوبك بالفعل ويترجمه في الوقت الفعلي باستخدام Gemini Live Translate. يُشغَّل الصوت المترجم عبر جهاز الإخراج الذي تختاره، مباشرة من حاسوبك ودون خادم وسيط.

<p align="center"><img src="docs/settings.png" width="620" alt="LiveTranslate settings" /></p>

## الميزات

- ترجمة صوتية فورية من الكلام إلى الكلام باستخدام Gemini Live Translate.
- كبسولة عائمة مدمجة (100×32 بكسل): انقر على منطقة الحالة للتشغيل أو الإيقاف، واسحب الشبح، واستخدم الترس الصغير للإعدادات.
- التقاط صوت النظام أصليًا على Windows (WASAPI) وmacOS (ScreenCaptureKit) وLinux (PulseAudio/PipeWire)، دون كابلات افتراضية.
- 22 لغة لواجهة المستخدم، سمة فاتحة/داكنة، شريط النظام، اختصار عام Ctrl+Alt+T وإعادة اتصال تلقائية.
- يُحفظ مفتاح API في مخزن مفاتيح النظام؛ يُرسل الصوت مباشرة إلى Gemini دون حساب أو قياس عن بُعد.

## التثبيت

نزّل أحدث إصدار من [GitHub Releases](https://github.com/sergio-dev1-py/translate-live/releases/latest). الإصدارات غير موقّعة؛ قد يطلب SmartScreen أو Gatekeeper تأكيدًا.

| المنصة | الملف |
|---|---|
| Windows 10/11 | <code>LiveTranslate_&lt;version&gt;_x64-setup.exe</code> |
| macOS 13+ | <code>LiveTranslate_&lt;version&gt;_universal.dmg</code> |
| Linux | .AppImage / .deb |

## البدء السريع

1. ثبّت LiveTranslate وافتحه؛ ستظهر الكبسولة فوق شريط المهام.
2. أنشئ مفتاح API من [Google AI Studio](https://aistudio.google.com/apikey)، والصقه في الإعدادات → عام، واختبر الاتصال ثم احفظ.
3. في الصوت اختر جهاز الالتقاط والإخراج. شغّل صوتًا وانقر على <code>AUTO→ES</code>؛ النقطة الخضراء تعني البث المباشر. يعمل Ctrl+Alt+T أيضًا على التشغيل والإيقاف.

## الخصوصية

يُحفظ مفتاح API في مخزن مفاتيح النظام فقط ولا يُرسل إلى الواجهة أبدًا. يُرسل الصوت مباشرة إلى Gemini؛ لا يوجد خادم LiveTranslate أو حساب أو قياس عن بُعد أو تسجيل. لا ترفع المفاتيح أو ملفات .env إلى المستودع.

## المنصات

| المنصة | الالتقاط | الحالة |
|---|---|---|
| Windows 10/11 | WASAPI loopback | تم الاختبار على Windows 10 21H2 |
| macOS 13+ | ScreenCaptureKit | مُنفَّذ؛ بناء CI؛ اختبار يدوي معلّق |
| Linux | PulseAudio / PipeWire | مُنفَّذ؛ بناء CI؛ اختبار يدوي معلّق |

## البناء من المصدر

يتطلب Node.js 22+ وRust مستقرًا واعتماديات النظام. التفاصيل الخاصة بـ macOS/Linux موجودة في README الإنجليزي.

    git clone https://github.com/sergio-dev1-py/translate-live.git
    cd translate-live
    npm ci
    npm run tauri dev

## استكشاف الأخطاء

- لا يوجد جهاز إخراج: أعد توصيل الجهاز، واضغط تحديث، ثم اختره في الصوت.
- macOS: امنح إذن Screen & System Audio Recording وأعد تشغيل التطبيق.
- Linux: ثبّت PulseAudio أو PipeWire مع pipewire-pulse وlibpulse.

## الترخيص

MIT — راجع [LICENSE](LICENSE).

> [الوثائق الكاملة بالإنجليزية](README.md).

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
  <strong>العربية</strong> ·
  <a href="README.no.md">Norsk</a> ·
  <a href="README.pt-BR.md">Português (Brasil)</a> ·
  <a href="README.he.md">עברית</a> ·
  <a href="README.tr.md">Türkçe</a> ·
  <a href="README.uk.md">Українська</a> ·
  <a href="README.hi.md">हिन्दी</a> ·
  <a href="README.el.md">Ελληνικά</a> ·
  <a href="README.vi.md">Tiếng Việt</a>
</p>
