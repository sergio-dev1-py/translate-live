# Security policy

## API keys

LiveTranslate stores the Google Gemini API key in the operating system keychain:

- **Windows:** Credential Manager
- **macOS:** Keychain
- **Linux:** Secret Service

The key is never written to the repository, `settings.json` or the React frontend. The Rust backend reads it only when it opens a Gemini session.

Never commit:

- API keys or tokens
- `.env` files
- exported keychain data
- captured settings files
- private certificates or signing keys

The repository's `.gitignore` excludes the common local files, but you are responsible for checking your changes before pushing.

If a key is accidentally committed, revoke it immediately in [Google AI Studio](https://aistudio.google.com/apikey), remove it from the Git history and force-push the cleaned history.

## Reporting a vulnerability

Please do not open a public issue for a security problem. Use GitHub's **Security → Report a vulnerability** page for this repository so the report stays private until it is fixed.

Include:

- the affected version or commit;
- the operating system and architecture;
- steps to reproduce;
- the possible impact;
- any suggested fix.

## Scope

LiveTranslate is a local desktop app. Audio is sent directly from the user's computer to the Gemini API; there is no LiveTranslate server, account or telemetry.
