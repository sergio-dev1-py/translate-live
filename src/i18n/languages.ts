export type UiLanguage = {
  code: string;
  native: string;
  rtl?: boolean;
};

/** Idiomas de la interfaz de LiveTranslate. */
export const UI_LANGUAGES: UiLanguage[] = [
  { code: "en", native: "English" },
  { code: "zh-Hans", native: "简体中文" },
  { code: "zh-Hant", native: "繁體中文" },
  { code: "ko", native: "한국어" },
  { code: "de", native: "Deutsch" },
  { code: "es", native: "Español" },
  { code: "fr", native: "Français" },
  { code: "it", native: "Italiano" },
  { code: "da", native: "Dansk" },
  { code: "ja", native: "日本語" },
  { code: "pl", native: "Polski" },
  { code: "ru", native: "Русский" },
  { code: "bs", native: "Bosanski" },
  { code: "ar", native: "العربية", rtl: true },
  { code: "no", native: "Norsk" },
  { code: "pt-BR", native: "Português (Brasil)" },
  { code: "he", native: "עברית", rtl: true },
  { code: "tr", native: "Türkçe" },
  { code: "uk", native: "Українська" },
  { code: "hi", native: "हिन्दी" },
  { code: "el", native: "Ελληνικά" },
  { code: "vi", native: "Tiếng Việt" },
];

const supported = new Set(UI_LANGUAGES.map((language) => language.code));

export function normalizeUiLanguage(code: string | null | undefined): string {
  if (!code) return "en";
  const raw = code.replace("_", "-");
  if (supported.has(raw)) return raw;
  const lower = raw.toLowerCase();
  if (lower.startsWith("zh")) {
    return lower.includes("tw") || lower.includes("hk") || lower.includes("hant")
      ? "zh-Hant"
      : "zh-Hans";
  }
  if (lower.startsWith("pt")) return "pt-BR";
  if (lower.startsWith("nb") || lower.startsWith("nn")) return "no";
  if (lower.startsWith("iw")) return "he";
  const base = lower.split("-")[0];
  return supported.has(base) ? base : "en";
}

export function uiLanguageNative(code: string | null | undefined): string {
  const normalized = normalizeUiLanguage(code);
  return UI_LANGUAGES.find((language) => language.code === normalized)?.native ?? "English";
}

export function isRtlLanguage(code: string | null | undefined): boolean {
  const normalized = normalizeUiLanguage(code);
  return Boolean(UI_LANGUAGES.find((language) => language.code === normalized)?.rtl);
}
