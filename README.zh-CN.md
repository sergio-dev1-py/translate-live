<p align="center">
  <img src="assets/icon.svg" width="96" alt="LiveTranslate ghost icon" />
</p>

<h1 align="center">LiveTranslate</h1>

<p align="center"><strong>使用 Gemini Live 实时翻译系统音频。100×32 悬浮胶囊，无需虚拟音频线。</strong></p>

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
  <strong>简体中文</strong> ·
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
  <a href="README.vi.md">Tiếng Việt</a>
</p>

<p align="center"><img src="docs/pill.png" width="620" alt="LiveTranslate floating pill" /></p>

<p align="center"><sub>100×32 · LiveTranslate</sub></p>

---

## 简介

LiveTranslate 捕获电脑正在播放的音频，并通过 Gemini Live Translate 实时翻译。翻译后的声音从你选择的输出设备播放，不需要虚拟音频线，也没有中间服务器。

<p align="center"><img src="docs/settings.png" width="620" alt="LiveTranslate settings" /></p>

## 功能

- 使用 Gemini Live Translate 进行实时语音到语音翻译。
- 100×32 悬浮胶囊：点击状态区开始/停止，拖动幽灵图标，使用迷你齿轮打开设置。
- Windows（WASAPI）、macOS（ScreenCaptureKit）和 Linux（PulseAudio/PipeWire）原生系统音频捕获，无需虚拟音频线。
- 22 种界面语言、浅色/深色主题、系统托盘、全局快捷键 Ctrl+Alt+T 和自动重连。
- API key 保存在系统钥匙串中；音频直接发送到 Gemini，没有账户或遥测。

## 安装

从 [GitHub Releases](https://github.com/sergio-dev1-py/translate-live/releases/latest) 下载最新版本。构建未签名，SmartScreen 或 Gatekeeper 可能要求确认。

| 平台 | 文件 |
|---|---|
| Windows 10/11 | <code>LiveTranslate_&lt;version&gt;_x64-setup.exe</code> |
| macOS 13+ | <code>LiveTranslate_&lt;version&gt;_universal.dmg</code> |
| Linux | .AppImage / .deb |

## 快速开始

1. 安装并打开 LiveTranslate，胶囊会出现在任务栏上方。
2. 在 [Google AI Studio](https://aistudio.google.com/apikey) 创建 API key，粘贴到“设置 → 通用”，测试连接并保存。
3. 在“音频”中选择捕获和输出设备。播放音频并点击 <code>AUTO→ES</code>；绿点表示正在实时翻译。Ctrl+Alt+T 也可开始或停止。

## 隐私

API key 只保存在系统钥匙串中，前端永远无法读取。音频直接发送到 Gemini；没有 LiveTranslate 服务器、账户、遥测或录音。请勿提交密钥或 .env 文件。

## 平台支持

| 平台 | 捕获 | 状态 |
|---|---|---|
| Windows 10/11 | WASAPI loopback | 已在 Windows 10 21H2 测试 |
| macOS 13+ | ScreenCaptureKit | 已实现；CI 构建；待手动测试 |
| Linux | PulseAudio / PipeWire | 已实现；CI 构建；待手动测试 |

## 从源码构建

需要 Node.js 22+、稳定版 Rust 和系统依赖。macOS/Linux 的详细步骤见英文 README。

    git clone https://github.com/sergio-dev1-py/translate-live.git
    cd translate-live
    npm ci
    npm run tauri dev

## 常见问题

- 没有输出设备：重新连接设备，点击“刷新”，然后在“音频”中选择它。
- macOS：授予 Screen & System Audio Recording 权限并重启应用。
- Linux：安装 PulseAudio 或带 pipewire-pulse 的 PipeWire，并安装 libpulse。

## 许可证

MIT — 查看 [LICENSE](LICENSE)。

> [完整的英文文档](README.md)。

---

<p align="center">
  <a href="README.md">English</a> ·
  <strong>简体中文</strong> ·
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
  <a href="README.vi.md">Tiếng Việt</a>
</p>
