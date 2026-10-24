<p align="center">
  <img src="assets/icon.svg" width="96" alt="LiveTranslate ghost icon" />
</p>

<h1 align="center">LiveTranslate</h1>

<p align="center"><strong>Dịch âm thanh hệ thống theo thời gian thực với Gemini Live. Một viên thuốc nổi 100×32, không cần cáp âm thanh ảo.</strong></p>

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
  <a href="README.el.md">Ελληνικά</a> ·
  <strong>Tiếng Việt</strong>
</p>

<p align="center"><img src="docs/pill.png" width="620" alt="LiveTranslate floating pill" /></p>

<p align="center"><sub>100×32 · LiveTranslate</sub></p>

---

## Tổng quan

LiveTranslate ghi lại âm thanh mà máy tính đang phát và dịch theo thời gian thực bằng Gemini Live Translate. Giọng dịch được phát trên thiết bị đầu ra bạn chọn, trực tiếp từ máy tính và không qua máy chủ trung gian.

<p align="center"><img src="docs/settings.png" width="620" alt="LiveTranslate settings" /></p>

## Tính năng

- Dịch giọng nói sang giọng nói theo thời gian thực với Gemini Live Translate.
- Viên thuốc nổi nhỏ gọn (100×32 px): nhấp vào trạng thái để bắt đầu/dừng, kéo biểu tượng ma và dùng bánh răng mini.
- Ghi âm thanh hệ thống gốc trên Windows (WASAPI), macOS (ScreenCaptureKit) và Linux (PulseAudio/PipeWire), không cần cáp ảo.
- 22 ngôn ngữ giao diện, giao diện sáng/tối, khay hệ thống, phím tắt toàn cục Ctrl+Alt+T và tự động kết nối lại.
- Khóa API được lưu trong keychain của hệ điều hành; âm thanh đi trực tiếp đến Gemini, không có tài khoản hay telemetry.

## Cài đặt

Tải bản mới nhất từ [GitHub Releases](https://github.com/sergio-dev1-py/translate-live/releases/latest). Các bản dựng chưa được ký; SmartScreen hoặc Gatekeeper có thể yêu cầu xác nhận.

| Nền tảng | Tệp |
|---|---|
| Windows 10/11 | <code>LiveTranslate_&lt;version&gt;_x64-setup.exe</code> |
| macOS 13+ | <code>LiveTranslate_&lt;version&gt;_universal.dmg</code> |
| Linux | .AppImage / .deb |

## Bắt đầu nhanh

1. Cài đặt và mở LiveTranslate; viên thuốc xuất hiện phía trên thanh tác vụ.
2. Tạo khóa API tại [Google AI Studio](https://aistudio.google.com/apikey), dán vào Cài đặt → Chung, kiểm tra kết nối và lưu.
3. Trong Âm thanh, chọn thiết bị ghi và đầu ra. Phát âm thanh và nhấp vào <code>AUTO→ES</code>; chấm xanh cho biết đang phát trực tiếp. Ctrl+Alt+T cũng bắt đầu hoặc dừng.

## Quyền riêng tư

Khóa API chỉ được lưu trong keychain của hệ điều hành và không bao giờ gửi đến frontend. Âm thanh đi trực tiếp đến Gemini; không có máy chủ LiveTranslate, tài khoản, telemetry hay bản ghi. Không commit khóa hoặc tệp .env.

## Nền tảng

| Nền tảng | Thu âm | Trạng thái |
|---|---|---|
| Windows 10/11 | WASAPI loopback | Đã kiểm tra trên Windows 10 21H2 |
| macOS 13+ | ScreenCaptureKit | Đã triển khai; build CI; chờ kiểm tra thủ công |
| Linux | PulseAudio / PipeWire | Đã triển khai; build CI; chờ kiểm tra thủ công |

## Biên dịch từ mã nguồn

Cần Node.js 22+, Rust ổn định và các phụ thuộc của nền tảng. Chi tiết cho macOS/Linux có trong README tiếng Anh.

    git clone https://github.com/sergio-dev1-py/translate-live.git
    cd translate-live
    npm ci
    npm run tauri dev

## Khắc phục sự cố

- Không có đầu ra: kết nối lại thiết bị, nhấn Làm mới và chọn trong Âm thanh.
- macOS: cấp quyền Screen & System Audio Recording và khởi động lại ứng dụng.
- Linux: cài PulseAudio hoặc PipeWire với pipewire-pulse và libpulse.

## Giấy phép

MIT — xem [LICENSE](LICENSE).

> [Tài liệu đầy đủ bằng tiếng Anh](README.md).

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
  <a href="README.el.md">Ελληνικά</a> ·
  <strong>Tiếng Việt</strong>
</p>
