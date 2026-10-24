<p align="center">
  <img src="assets/icon.svg" width="96" alt="LiveTranslate ghost icon" />
</p>

<h1 align="center">LiveTranslate</h1>

<p align="center"><strong>Traduza o áudio do sistema em tempo real com o Gemini Live. Uma pílula flutuante de 100×32, sem cabos de áudio virtuais.</strong></p>

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
  <strong>Português (Brasil)</strong> ·
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

## Visão geral

O LiveTranslate captura o áudio que o computador já está reproduzindo e o traduz em tempo real com o Gemini Live Translate. A voz traduzida é reproduzida na saída escolhida, diretamente do computador e sem servidor intermediário.

<p align="center"><img src="docs/settings.png" width="620" alt="LiveTranslate settings" /></p>

## Recursos

- Tradução de fala para fala em tempo real com o Gemini Live Translate.
- Pílula flutuante compacta (100×32 px): clique no status para iniciar/parar, arraste o fantasma e use a engrenagem mini.
- Captura nativa do áudio do sistema no Windows (WASAPI), macOS (ScreenCaptureKit) e Linux (PulseAudio/PipeWire), sem cabos virtuais.
- 22 idiomas de interface, temas claro/escuro, bandeja do sistema, atalho global Ctrl+Alt+T e reconexão automática.
- A chave de API fica no cofre do sistema; o áudio vai direto para o Gemini, sem conta ou telemetria.

## Instalação

Baixe a versão mais recente em [GitHub Releases](https://github.com/sergio-dev1-py/translate-live/releases/latest). As compilações não são assinadas; o SmartScreen ou o Gatekeeper pode pedir confirmação.

| Plataforma | Arquivo |
|---|---|
| Windows 10/11 | <code>LiveTranslate_&lt;version&gt;_x64-setup.exe</code> |
| macOS 13+ | <code>LiveTranslate_&lt;version&gt;_universal.dmg</code> |
| Linux | .AppImage / .deb |

## Início rápido

1. Instale e abra o LiveTranslate; a pílula aparece acima da barra de tarefas.
2. Crie uma chave de API no [Google AI Studio](https://aistudio.google.com/apikey), cole em Configurações → Geral, teste a conexão e salve.
3. Em Áudio, escolha a captura e a saída. Reproduza um áudio e clique em <code>AUTO→ES</code>; o ponto verde indica transmissão ao vivo. Ctrl+Alt+T também inicia ou para.

## Privacidade

A chave de API é armazenada apenas no cofre do sistema e nunca é enviada ao frontend. O áudio vai diretamente para o Gemini; não há servidor do LiveTranslate, conta, telemetria ou gravação. Não faça commit de chaves ou arquivos .env.

## Plataformas

| Plataforma | Captura | Status |
|---|---|---|
| Windows 10/11 | WASAPI loopback | Testado no Windows 10 21H2 |
| macOS 13+ | ScreenCaptureKit | Implementado; build de CI; teste manual pendente |
| Linux | PulseAudio / PipeWire | Implementado; build de CI; teste manual pendente |

## Compilar do código-fonte

Requer Node.js 22+, Rust estável e as dependências da plataforma. Os detalhes para macOS/Linux estão no README em inglês.

    git clone https://github.com/sergio-dev1-py/translate-live.git
    cd translate-live
    npm ci
    npm run tauri dev

## Solução de problemas

- Sem saída: reconecte o dispositivo, clique em Atualizar e selecione-o em Áudio.
- macOS: conceda a permissão Screen & System Audio Recording e reinicie o app.
- Linux: instale PulseAudio ou PipeWire com pipewire-pulse e libpulse.

## Licença

MIT — consulte [LICENSE](LICENSE).

> [Documentação completa em inglês](README.md).

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
  <strong>Português (Brasil)</strong> ·
  <a href="README.he.md">עברית</a> ·
  <a href="README.tr.md">Türkçe</a> ·
  <a href="README.uk.md">Українська</a> ·
  <a href="README.hi.md">हिन्दी</a> ·
  <a href="README.el.md">Ελληνικά</a> ·
  <a href="README.vi.md">Tiếng Việt</a>
</p>
