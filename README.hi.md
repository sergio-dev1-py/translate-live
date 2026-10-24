<p align="center">
  <img src="assets/icon.svg" width="96" alt="LiveTranslate ghost icon" />
</p>

<h1 align="center">LiveTranslate</h1>

<p align="center"><strong>Gemini Live के साथ सिस्टम ऑडियो का रीयल-टाइम अनुवाद। 100×32 की फ़्लोटिंग पिल, कोई वर्चुअल ऑडियो केबल नहीं।</strong></p>

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
  <strong>हिन्दी</strong> ·
  <a href="README.el.md">Ελληνικά</a> ·
  <a href="README.vi.md">Tiếng Việt</a>
</p>

<p align="center"><img src="docs/pill.png" width="620" alt="LiveTranslate floating pill" /></p>

<p align="center"><sub>100×32 · LiveTranslate</sub></p>

---

## अवलोकन

LiveTranslate आपके कंप्यूटर पर पहले से चल रहे ऑडियो को कैप्चर करता है और Gemini Live Translate से उसका रीयल-टाइम अनुवाद करता है। अनुवादित आवाज़ आपके चुने हुए आउटपुट पर सीधे आपके कंप्यूटर से चलती है; किसी बीच के सर्वर की ज़रूरत नहीं।

<p align="center"><img src="docs/settings.png" width="620" alt="LiveTranslate settings" /></p>

## विशेषताएँ

- Gemini Live Translate के साथ रीयल-टाइम speech-to-speech अनुवाद।
- कॉम्पैक्ट फ़्लोटिंग पिल (100×32 px): स्टेटस पर क्लिक करके शुरू/रोकें, भूत को खींचें और मिनी गियर से सेटिंग खोलें।
- Windows (WASAPI), macOS (ScreenCaptureKit) और Linux (PulseAudio/PipeWire) पर नेटिव सिस्टम ऑडियो कैप्चर, बिना वर्चुअल केबल।
- 22 UI भाषाएँ, लाइट/डार्क थीम, सिस्टम ट्रे, ग्लोबल शॉर्टकट Ctrl+Alt+T और ऑटो-रीकनेक्ट।
- API key OS keychain में रहती है; ऑडियो सीधे Gemini जाता है, कोई अकाउंट या टेलीमेट्री नहीं।

## इंस्टॉल

नवीनतम संस्करण [GitHub Releases](https://github.com/sergio-dev1-py/translate-live/releases/latest) से डाउनलोड करें। बिल्ड unsigned हैं; SmartScreen या Gatekeeper पुष्टि माँग सकते हैं।

| प्लेटफ़ॉर्म | फ़ाइल |
|---|---|
| Windows 10/11 | <code>LiveTranslate_&lt;version&gt;_x64-setup.exe</code> |
| macOS 13+ | <code>LiveTranslate_&lt;version&gt;_universal.dmg</code> |
| Linux | .AppImage / .deb |

## त्वरित शुरुआत

1. LiveTranslate इंस्टॉल करके खोलें; पिल टास्कबार के ऊपर दिखाई देगी।
2. [Google AI Studio](https://aistudio.google.com/apikey) पर API key बनाएँ, उसे सेटिंग्स → सामान्य में पेस्ट करें, कनेक्शन टेस्ट करें और सेव करें।
3. ऑडियो में कैप्चर और आउटपुट चुनें। ऑडियो चलाएँ और <code>AUTO→ES</code> पर क्लिक करें; हरा बिंदु लाइव होने का संकेत है। Ctrl+Alt+T से भी शुरू/रोक सकते हैं।

## गोपनीयता

API key केवल OS keychain में सेव होती है और frontend को कभी नहीं भेजी जाती। ऑडियो सीधे Gemini जाता है; LiveTranslate सर्वर, अकाउंट, टेलीमेट्री या रिकॉर्डिंग नहीं है। कुंजियाँ या .env फ़ाइलें कभी कमिट न करें।

## प्लेटफ़ॉर्म

| प्लेटफ़ॉर्म | कैप्चर | स्थिति |
|---|---|---|
| Windows 10/11 | WASAPI loopback | Windows 10 21H2 पर परीक्षण किया गया |
| macOS 13+ | ScreenCaptureKit | लागू; CI बिल्ड; मैन्युअल परीक्षण बाकी |
| Linux | PulseAudio / PipeWire | लागू; CI बिल्ड; मैन्युअल परीक्षण बाकी |

## स्रोत से बिल्ड

Node.js 22+, स्थिर Rust और प्लेटफ़ॉर्म निर्भरताएँ आवश्यक हैं। macOS/Linux के विवरण अंग्रेज़ी README में हैं।

    git clone https://github.com/sergio-dev1-py/translate-live.git
    cd translate-live
    npm ci
    npm run tauri dev

## समस्या निवारण

- आउटपुट नहीं दिख रहा: डिवाइस फिर से कनेक्ट करें, Refresh दबाएँ और ऑडियो में चुनें।
- macOS: Screen & System Audio Recording अनुमति दें और ऐप को पुनः आरंभ करें।
- Linux: PulseAudio या pipewire-pulse वाला PipeWire और libpulse इंस्टॉल करें।

## लाइसेंस

MIT — [LICENSE](LICENSE) देखें।

> [पूरा अंग्रेज़ी दस्तावेज़](README.md).

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
  <strong>हिन्दी</strong> ·
  <a href="README.el.md">Ελληνικά</a> ·
  <a href="README.vi.md">Tiếng Việt</a>
</p>
