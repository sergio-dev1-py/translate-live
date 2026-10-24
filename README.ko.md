<p align="center">
  <img src="assets/icon.svg" width="96" alt="LiveTranslate ghost icon" />
</p>

<h1 align="center">LiveTranslate</h1>

<p align="center"><strong>Gemini Live로 시스템 오디오를 실시간 번역합니다. 100×32 플로팅 필, 가상 오디오 케이블이 필요 없습니다.</strong></p>

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
  <strong>한국어</strong> ·
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
  <a href="README.el.md">Ελληνικά</a> ·
  <a href="README.vi.md">Tiếng Việt</a>
</p>

<p align="center"><img src="docs/pill.png" width="620" alt="LiveTranslate floating pill" /></p>

<p align="center"><sub>100×32 · LiveTranslate</sub></p>

---

## 소개

LiveTranslate는 컴퓨터에서 재생 중인 오디오를 캡처해 Gemini Live Translate로 실시간 번역합니다. 번역된 음성은 선택한 출력 장치로 재생되며, 가상 오디오 케이블이나 중간 서버가 필요 없습니다.

<p align="center"><img src="docs/settings.png" width="620" alt="LiveTranslate settings" /></p>

## 기능

- Gemini Live Translate를 사용한 실시간 음성 대 음성 번역.
- 100×32 플로팅 필: 상태 영역을 클릭해 시작/중지하고, 유령 아이콘을 드래그하며, 미니 기어로 설정을 엽니다.
- Windows(WASAPI), macOS(ScreenCaptureKit), Linux(PulseAudio/PipeWire)에서 네이티브 시스템 오디오 캡처. 가상 케이블 불필요.
- 22개 UI 언어, 라이트/다크 테마, 시스템 트레이, 전역 단축키 Ctrl+Alt+T, 자동 재연결.
- API key는 OS 키체인에 저장되며 오디오는 Gemini로 직접 전송됩니다. 계정이나 텔레메트리는 없습니다.

## 설치

[GitHub Releases](https://github.com/sergio-dev1-py/translate-live/releases/latest)에서 최신 버전을 다운로드하세요. 빌드는 서명되지 않았으며 SmartScreen 또는 Gatekeeper가 확인을 요청할 수 있습니다.

| 플랫폼 | 파일 |
|---|---|
| Windows 10/11 | <code>LiveTranslate_&lt;version&gt;_x64-setup.exe</code> |
| macOS 13+ | <code>LiveTranslate_&lt;version&gt;_universal.dmg</code> |
| Linux | .AppImage / .deb |

## 빠른 시작

1. LiveTranslate를 설치하고 실행하면 작업 표시줄 위에 필이 나타납니다.
2. [Google AI Studio](https://aistudio.google.com/apikey)에서 API key를 만들고 설정 → 일반에 붙여 넣은 뒤 연결 테스트와 저장을 누르세요.
3. 오디오에서 캡처 및 출력 장치를 선택하세요. 오디오를 재생하고 <code>AUTO→ES</code>를 클릭하면 녹색 점이 실시간 상태를 나타냅니다. Ctrl+Alt+T로도 시작/중지할 수 있습니다.

## 개인정보 보호

API key는 OS 키체인에만 저장되며 프런트엔드에 전달되지 않습니다. 오디오는 Gemini로 직접 전송되며 LiveTranslate 서버, 계정, 텔레메트리, 녹음이 없습니다. 키나 .env 파일을 커밋하지 마세요.

## 플랫폼

| 플랫폼 | 캡처 | 상태 |
|---|---|---|
| Windows 10/11 | WASAPI loopback | Windows 10 21H2에서 테스트 완료 |
| macOS 13+ | ScreenCaptureKit | 구현됨; CI 빌드; 수동 테스트 대기 |
| Linux | PulseAudio / PipeWire | 구현됨; CI 빌드; 수동 테스트 대기 |

## 소스에서 빌드

Node.js 22+, 안정적인 Rust 및 플랫폼 의존성이 필요합니다. macOS/Linux 자세한 내용은 영어 README를 참고하세요.

    git clone https://github.com/sergio-dev1-py/translate-live.git
    cd translate-live
    npm ci
    npm run tauri dev

## 문제 해결

- 출력 장치가 없음: 장치를 다시 연결하고 새로 고침을 누른 뒤 오디오에서 선택하세요.
- macOS: Screen & System Audio Recording 권한을 허용하고 앱을 다시 시작하세요.
- Linux: PulseAudio 또는 pipewire-pulse가 포함된 PipeWire와 libpulse를 설치하세요.

## 라이선스

MIT — [LICENSE](LICENSE)를 참고하세요.

> [전체 영어 문서](README.md).

---

<p align="center">
  <a href="README.md">English</a> ·
  <a href="README.zh-CN.md">简体中文</a> ·
  <a href="README.zh-TW.md">繁體中文</a> ·
  <strong>한국어</strong> ·
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
  <a href="README.el.md">Ελληνικά</a> ·
  <a href="README.vi.md">Tiếng Việt</a>
</p>
