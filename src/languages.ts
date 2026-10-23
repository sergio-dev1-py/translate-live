export type Language = {
  code: string;
  name: string;
  native: string;
};

/**
 * Idiomas soportados por gemini-3.5-live-translate-preview.
 * Fuente: https://ai.google.dev/gemini-api/docs/live-api/live-translate
 */
export const LANGUAGES: Language[] = [
  { code: "af", name: "Afrikáans", native: "Afrikaans" },
  { code: "ak", name: "Akan", native: "Akan" },
  { code: "sq", name: "Albanés", native: "Shqip" },
  { code: "am", name: "Amárico", native: "አማርኛ" },
  { code: "ar", name: "Árabe", native: "العربية" },
  { code: "hy", name: "Armenio", native: "Հայերեն" },
  { code: "az", name: "Azerbaiyano", native: "Azərbaycan dili" },
  { code: "eu", name: "Euskera", native: "Euskara" },
  { code: "be", name: "Bielorruso", native: "Беларуская" },
  { code: "bn", name: "Bengalí", native: "বাংলা" },
  { code: "bg", name: "Búlgaro", native: "Български" },
  { code: "my", name: "Birmano", native: "မြန်မာ" },
  { code: "ca", name: "Catalán", native: "Català" },
  { code: "zh-Hans", name: "Chino simplificado", native: "简体中文" },
  { code: "zh-Hant", name: "Chino tradicional", native: "繁體中文" },
  { code: "hr", name: "Croata", native: "Hrvatski" },
  { code: "cs", name: "Checo", native: "Čeština" },
  { code: "da", name: "Danés", native: "Dansk" },
  { code: "nl", name: "Neerlandés", native: "Nederlands" },
  { code: "en", name: "Inglés", native: "English" },
  { code: "et", name: "Estonio", native: "Eesti" },
  { code: "fil", name: "Filipino", native: "Filipino" },
  { code: "fi", name: "Finés", native: "Suomi" },
  { code: "fr", name: "Francés", native: "Français" },
  { code: "gl", name: "Gallego", native: "Galego" },
  { code: "ka", name: "Georgiano", native: "ქართული" },
  { code: "de", name: "Alemán", native: "Deutsch" },
  { code: "el", name: "Griego", native: "Ελληνικά" },
  { code: "gu", name: "Gujarati", native: "ગુજરાતી" },
  { code: "ha", name: "Hausa", native: "Hausa" },
  { code: "he", name: "Hebreo", native: "עברית" },
  { code: "hi", name: "Hindi", native: "हिन्दी" },
  { code: "hu", name: "Húngaro", native: "Magyar" },
  { code: "is", name: "Islandés", native: "Íslenska" },
  { code: "id", name: "Indonesio", native: "Bahasa Indonesia" },
  { code: "it", name: "Italiano", native: "Italiano" },
  { code: "ja", name: "Japonés", native: "日本語" },
  { code: "jv", name: "Javanés", native: "Basa Jawa" },
  { code: "kn", name: "Kannada", native: "ಕನ್ನಡ" },
  { code: "kk", name: "Kazajo", native: "Қазақ тілі" },
  { code: "km", name: "Jemer", native: "ខ្មែរ" },
  { code: "rw", name: "Kinyarwanda", native: "Kinyarwanda" },
  { code: "ko", name: "Coreano", native: "한국어" },
  { code: "lo", name: "Lao", native: "ລາວ" },
  { code: "lv", name: "Letón", native: "Latviešu" },
  { code: "lt", name: "Lituano", native: "Lietuvių" },
  { code: "mk", name: "Macedonio", native: "Македонски" },
  { code: "ms", name: "Malayo", native: "Bahasa Melayu" },
  { code: "ml", name: "Malayalam", native: "മലയാളം" },
  { code: "mr", name: "Maratí", native: "मराठी" },
  { code: "mn", name: "Mongol", native: "Монгол" },
  { code: "ne", name: "Nepalí", native: "नेपाली" },
  { code: "no", name: "Noruego", native: "Norsk" },
  { code: "fa", name: "Persa", native: "فارسی" },
  { code: "pl", name: "Polaco", native: "Polski" },
  { code: "pt-BR", name: "Portugués (Brasil)", native: "Português (Brasil)" },
  { code: "pt-PT", name: "Portugués (Portugal)", native: "Português (Portugal)" },
  { code: "pa", name: "Punyabí", native: "ਪੰਜਾਬੀ" },
  { code: "ro", name: "Rumano", native: "Română" },
  { code: "ru", name: "Ruso", native: "Русский" },
  { code: "sr", name: "Serbio", native: "Српски" },
  { code: "sd", name: "Sindhi", native: "سنڌي" },
  { code: "si", name: "Cingalés", native: "සිංහල" },
  { code: "sk", name: "Eslovaco", native: "Slovenčina" },
  { code: "sl", name: "Esloveno", native: "Slovenščina" },
  { code: "es", name: "Español", native: "Español" },
  { code: "su", name: "Sundanés", native: "Basa Sunda" },
  { code: "sw", name: "Suajili", native: "Kiswahili" },
  { code: "sv", name: "Sueco", native: "Svenska" },
  { code: "ta", name: "Tamil", native: "தமிழ்" },
  { code: "te", name: "Telugu", native: "తెలుగు" },
  { code: "th", name: "Tailandés", native: "ไทย" },
  { code: "tr", name: "Turco", native: "Türkçe" },
  { code: "uk", name: "Ucraniano", native: "Українська" },
  { code: "ur", name: "Urdu", native: "اردو" },
  { code: "uz", name: "Uzbeko", native: "Oʻzbek" },
  { code: "vi", name: "Vietnamita", native: "Tiếng Việt" },
  { code: "zu", name: "Zulú", native: "isiZulu" },
];

export const languageName = (code: string | null | undefined): string => {
  if (!code) return "";
  const clean = code.split("-")[0].toLowerCase();
  return (
    LANGUAGES.find((l) => l.code.toLowerCase() === code.toLowerCase())?.native ??
    LANGUAGES.find((l) => l.code.split("-")[0].toLowerCase() === clean)?.native ??
    code
  );
};

export const shortCode = (code: string | null | undefined): string => {
  if (!code) return "AUTO";
  return code.split("-")[0].toUpperCase();
};
