<p align="center">
  <img src="assets/icon.svg" width="96" alt="LiveTranslate ghost icon" />
</p>

<h1 align="center">LiveTranslate</h1>

<p align="center"><strong>Μετάφραση ήχου συστήματος σε πραγματικό χρόνο με το Gemini Live. Μια μικρή αιωρούμενη κάψουλα 100×32, χωρίς εικονικά καλώδια ήχου.</strong></p>

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
  <a href="README.uk.md">Українська</a> ·
  <a href="README.hi.md">हिन्दी</a> ·
  <strong>Ελληνικά</strong> ·
  <a href="README.vi.md">Tiếng Việt</a>
</p>

<p align="center"><img src="docs/pill.png" width="620" alt="LiveTranslate floating pill" /></p>

<p align="center"><sub>100×32 · LiveTranslate</sub></p>

---

## Επισκόπηση

Το LiveTranslate καταγράφει τον ήχο που αναπαράγει ήδη ο υπολογιστής και τον μεταφράζει σε πραγματικό χρόνο με το Gemini Live Translate. Η μεταφρασμένη φωνή αναπαράγεται στην έξοδο που επιλέγετε, απευθείας από τον υπολογιστή και χωρίς ενδιάμεσο διακομιστή.

<p align="center"><img src="docs/settings.png" width="620" alt="LiveTranslate settings" /></p>

## Χαρακτηριστικά

- Μετάφραση ομιλίας σε ομιλία σε πραγματικό χρόνο με το Gemini Live Translate.
- Συμπαγής αιωρούμενη κάψουλα (100×32 px): κλικ στην κατάσταση για έναρξη/διακοπή, σύρετε το φάντασμα και χρησιμοποιήστε το μίνι γρανάζι.
- Εγγενής καταγραφή ήχου συστήματος σε Windows (WASAPI), macOS (ScreenCaptureKit) και Linux (PulseAudio/PipeWire), χωρίς εικονικά καλώδια.
- 22 γλώσσες διεπαφής, φωτεινό/σκοτεινό θέμα, δίσκος συστήματος, καθολική συντόμευση Ctrl+Alt+T και αυτόματη επανασύνδεση.
- Το κλειδί API αποθηκεύεται στο keychain του συστήματος· ο ήχος πηγαίνει απευθείας στο Gemini, χωρίς λογαριασμό ή τηλεμετρία.

## Εγκατάσταση

Κατεβάστε την τελευταία έκδοση από το [GitHub Releases](https://github.com/sergio-dev1-py/translate-live/releases/latest). Οι εκδόσεις δεν είναι υπογεγραμμένες· το SmartScreen ή το Gatekeeper μπορεί να ζητήσει επιβεβαίωση.

| Πλατφόρμα | Αρχείο |
|---|---|
| Windows 10/11 | <code>LiveTranslate_&lt;version&gt;_x64-setup.exe</code> |
| macOS 13+ | <code>LiveTranslate_&lt;version&gt;_universal.dmg</code> |
| Linux | .AppImage / .deb |

## Γρήγορη εκκίνηση

1. Εγκαταστήστε και ανοίξτε το LiveTranslate· η κάψουλα εμφανίζεται πάνω από τη γραμμή εργασιών.
2. Δημιουργήστε ένα κλειδί API στο [Google AI Studio](https://aistudio.google.com/apikey), επικολλήστε το στις Ρυθμίσεις → Γενικά, δοκιμάστε τη σύνδεση και αποθηκεύστε.
3. Στον Ήχο επιλέξτε καταγραφή και έξοδο. Αναπαράγετε ήχο και κάντε κλικ στο <code>AUTO→ES</code>· η πράσινη κουκκίδα δείχνει ζωντανή μετάφραση. Το Ctrl+Alt+T ξεκινά ή σταματά επίσης.

## Απόρρητο

Το κλειδί API αποθηκεύεται μόνο στο keychain του συστήματος και δεν αποστέλλεται ποτέ στο frontend. Ο ήχος πηγαίνει απευθείας στο Gemini· δεν υπάρχει διακομιστής LiveTranslate, λογαριασμός, τηλεμετρία ή ηχογράφηση. Μην κάνετε commit κλειδιά ή αρχεία .env.

## Πλατφόρμες

| Πλατφόρμα | Λήψη | Κατάσταση |
|---|---|---|
| Windows 10/11 | WASAPI loopback | Δοκιμασμένο σε Windows 10 21H2 |
| macOS 13+ | ScreenCaptureKit | Υλοποιημένο· build CI· εκκρεμεί χειροκίνητη δοκιμή |
| Linux | PulseAudio / PipeWire | Υλοποιημένο· build CI· εκκρεμεί χειροκίνητη δοκιμή |

## Κατασκευή από τον κώδικα

Απαιτούνται Node.js 22+, σταθερό Rust και οι εξαρτήσεις της πλατφόρμας. Λεπτομέρειες για macOS/Linux υπάρχουν στο αγγλικό README.

    git clone https://github.com/sergio-dev1-py/translate-live.git
    cd translate-live
    npm ci
    npm run tauri dev

## Αντιμετώπιση προβλημάτων

- Δεν υπάρχει έξοδος: επανασυνδέστε τη συσκευή, πατήστε Ανανέωση και επιλέξτε την στον Ήχο.
- macOS: παραχωρήστε την άδεια Screen & System Audio Recording και επανεκκινήστε την εφαρμογή.
- Linux: εγκαταστήστε PulseAudio ή PipeWire με pipewire-pulse και libpulse.

## Άδεια

MIT — δείτε [LICENSE](LICENSE).

> [Πλήρης τεκμηρίωση στα αγγλικά](README.md).

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
  <a href="README.uk.md">Українська</a> ·
  <a href="README.hi.md">हिन्दी</a> ·
  <strong>Ελληνικά</strong> ·
  <a href="README.vi.md">Tiếng Việt</a>
</p>
