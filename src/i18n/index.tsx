import {
  createContext,
  useCallback,
  useContext,
  useEffect,
  useMemo,
  useState,
  type ReactNode,
} from "react";
import { invoke } from "@tauri-apps/api/core";
import { getCurrentWindow } from "@tauri-apps/api/window";
import {
  isRtlLanguage,
  normalizeUiLanguage,
  uiLanguageNative,
} from "./languages";
import type { Bootstrap } from "../types";

type Vars = Record<string, string | number>;
type Dict = Record<string, string>;

const dictionaries = import.meta.glob<Dict>("./locales/*.json", {
  eager: true,
  import: "default",
}) as Record<string, Dict>;

const localeDicts: Record<string, Dict> = {};
for (const [path, dict] of Object.entries(dictionaries)) {
  const code = path.replace("./locales/", "").replace(".json", "");
  localeDicts[code] = dict;
}

function translate(lang: string, key: string, vars?: Vars): string {
  const template =
    localeDicts[lang]?.[key] ??
    localeDicts.en?.[key] ??
    localeDicts.es?.[key] ??
    key;
  if (!vars) return template;
  return template.replace(/\{(\w+)\}/g, (_, name: string) =>
    vars[name] !== undefined ? String(vars[name]) : `{${name}}`
  );
}

export function applyTheme(theme: string | null | undefined): void {
  const normalized = theme === "dark" ? "dark" : "light";
  document.documentElement.dataset.theme = normalized;
  localStorage.setItem("lt_theme", normalized);
  try {
    void getCurrentWindow().setTheme(normalized);
  } catch {
    // El tema nativo puede no estar disponible en todos los escritorios.
  }
}

export function applyUiLanguage(lang: string): void {
  const normalized = normalizeUiLanguage(lang);
  document.documentElement.lang = normalized;
  document.documentElement.dir = isRtlLanguage(normalized) ? "rtl" : "ltr";
  localStorage.setItem("lt_ui_language", normalized);
}

type I18nValue = {
  lang: string;
  dir: "ltr" | "rtl";
  t: (key: string, vars?: Vars) => string;
  nativeLanguage: (code?: string | null) => string;
};

const I18nContext = createContext<I18nValue>({
  lang: "es",
  dir: "ltr",
  t: (key) => key,
  nativeLanguage: (code) => uiLanguageNative(code),
});

export function I18nProvider({ children }: { children: ReactNode }) {
  const [lang, setLang] = useState(() => {
    const cached =
      typeof localStorage !== "undefined"
        ? localStorage.getItem("lt_ui_language")
        : null;
    return normalizeUiLanguage(cached || "es");
  });

  useEffect(() => {
    applyUiLanguage(lang);
  }, [lang]);

  useEffect(() => {
    let cancelled = false;
    void invoke<Bootstrap>("get_bootstrap")
      .then((boot) => {
        if (cancelled) return;
        const next = normalizeUiLanguage(boot.settings.ui_language);
        setLang(next);
        applyUiLanguage(next);
        applyTheme(boot.settings.theme || "light");
        localStorage.setItem("lt_ui_language", next);
        localStorage.setItem("lt_theme", boot.settings.theme || "light");
        localStorage.setItem("lt_detected_language", boot.detected_language || "");
        const t = (key: string) => translate(next, key);
        void invoke("set_tray_labels", {
          labels: {
            show: t("tray.show"),
            toggle: t("tray.toggle"),
            settings: t("tray.settings"),
            quit: t("tray.quit"),
            tooltip: t("tray.tooltip"),
          },
        }).catch(() => {});
      })
      .catch(() => {});
    return () => {
      cancelled = true;
    };
  }, []);

  const t = useCallback(
    (key: string, vars?: Vars) => translate(lang, key, vars),
    [lang]
  );

  const value = useMemo<I18nValue>(
    () => ({
      lang,
      dir: isRtlLanguage(lang) ? "rtl" : "ltr",
      t,
      nativeLanguage: (code?: string | null) => uiLanguageNative(code),
    }),
    [lang, t]
  );

  return <I18nContext.Provider value={value}>{children}</I18nContext.Provider>;
}

export function useI18n(): I18nValue {
  return useContext(I18nContext);
}
