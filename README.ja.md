<p align="center">
  <img src="assets/icon.svg" width="96" alt="LiveTranslate ghost icon" />
</p>

<h1 align="center">LiveTranslate</h1>

<p align="center"><strong>Gemini Live でシステム音声をリアルタイム翻訳。100×32 のフローティングピル。仮想オーディオケーブルは不要です。</strong></p>

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
  <strong>日本語</strong> ·
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

## 概要

LiveTranslate は、コンピューターで再生中の音声を取り込み、Gemini Live Translate でリアルタイムに翻訳します。翻訳された音声は選択した出力先から再生され、仮想オーディオケーブルや中間サーバーは必要ありません。

<p align="center"><img src="docs/settings.png" width="620" alt="LiveTranslate settings" /></p>

## 機能

- Gemini Live Translate によるリアルタイムの音声対音声翻訳。
- 100×32 のフローティングピル：状態エリアをクリックして開始/停止、ゴーストをドラッグ、ミニ歯車で設定を開きます。
- Windows（WASAPI）、macOS（ScreenCaptureKit）、Linux（PulseAudio/PipeWire）のネイティブなシステム音声キャプチャ。仮想ケーブル不要。
- 22 の UI 言語、ライト/ダークテーマ、タスクトレイ、グローバルショートカット Ctrl+Alt+T、自動再接続。
- API キーは OS のキーチェーンに保存され、音声は Gemini に直接送信されます。アカウントやテレメトリはありません。

## インストール

最新版は [GitHub Releases](https://github.com/sergio-dev1-py/translate-live/releases/latest) からダウンロードできます。ビルドは署名されていないため、SmartScreen や Gatekeeper が確認を求める場合があります。

| プラットフォーム | ファイル |
|---|---|
| Windows 10/11 | <code>LiveTranslate_&lt;version&gt;_x64-setup.exe</code> |
| macOS 13+ | <code>LiveTranslate_&lt;version&gt;_universal.dmg</code> |
| Linux | .AppImage / .deb |

## クイックスタート

1. LiveTranslate をインストールして起動すると、タスクバーの上にピルが表示されます。
2. [Google AI Studio](https://aistudio.google.com/apikey) で API キーを作成し、「設定 → 一般」に貼り付けて接続テストと保存を行います。
3. 「オーディオ」でキャプチャと出力を選択します。音声を再生して <code>AUTO→ES</code> をクリックすると、緑の点がライブ状態を示します。Ctrl+Alt+T でも開始/停止できます。

## プライバシー

API キーは OS のキーチェーンにのみ保存され、フロントエンドには渡されません。音声は Gemini に直接送信され、LiveTranslate のサーバー、アカウント、テレメトリ、録音はありません。キーや .env ファイルをコミットしないでください。

## プラットフォーム

| プラットフォーム | キャプチャ | 状態 |
|---|---|---|
| Windows 10/11 | WASAPI loopback | Windows 10 21H2 でテスト済み |
| macOS 13+ | ScreenCaptureKit | 実装済み。CI ビルド。手動テスト待ち |
| Linux | PulseAudio / PipeWire | 実装済み。CI ビルド。手動テスト待ち |

## ソースからビルド

Node.js 22+、安定版 Rust、各プラットフォームの依存関係が必要です。macOS/Linux の詳細は英語 README を参照してください。

    git clone https://github.com/sergio-dev1-py/translate-live.git
    cd translate-live
    npm ci
    npm run tauri dev

## トラブルシューティング

- 出力デバイスがない場合：デバイスを再接続し、「更新」を押して「オーディオ」で選択します。
- macOS：Screen & System Audio Recording の権限を許可してアプリを再起動します。
- Linux：PulseAudio または pipewire-pulse を含む PipeWire と libpulse をインストールします。

## ライセンス

MIT — [LICENSE](LICENSE) を参照してください。

> [英語の完全なドキュメント](README.md)。

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
  <strong>日本語</strong> ·
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
