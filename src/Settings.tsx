import { useCallback, useEffect, useMemo, useRef, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import {
  AlertIcon,
  CheckIcon,
  ExternalIcon,
  EyeIcon,
  EyeOffIcon,
  GhostIcon,
  GlobeIcon,
  MonitorIcon,
  MoonIcon,
  RefreshIcon,
  SpeakerIcon,
  SunIcon,
} from "./icons";
import { LANGUAGES, languageName, shortCode } from "./languages";
import { UI_LANGUAGES, uiLanguageNative } from "./i18n/languages";
import { applyTheme, useI18n } from "./i18n";
import type {
  Bootstrap,
  DeviceInfo,
  LanguagePayload,
  ProbeResult,
  Settings as SettingsType,
} from "./types";

type Tab = "general" | "audio" | "appearance" | "advanced";
type Toast = { kind: "ok" | "error" | "info"; text: string } | null;
type Translate = (key: string) => string;

const API_KEY_URL = "https://aistudio.google.com/apikey";

const DEFAULT_SETTINGS: SettingsType = {
  target_language: "es",
  capture_device_id: "default",
  output_device_name: "",
  output_volume: 1,
  input_gain_db: 0,
  echo_target_language: false,
  auto_reconnect: true,
  ui_language: "es",
  theme: "light",
};

function localizeError(error: unknown, t: Translate): string {
  const raw = String(error);
  const lower = raw.toLowerCase();
  if (lower.includes("configura tu api key")) return t("error.apiKeyMissing");
  if (lower.includes("api key") || lower.includes("401") || lower.includes("403")) {
    return t("error.apiKeyInvalid");
  }
  if (
    lower.includes("429") ||
    lower.includes("límite") ||
    lower.includes("limit") ||
    lower.includes("quota")
  ) {
    return t("error.quota");
  }
  if (
    lower.includes("conectar") ||
    lower.includes("connect") ||
    lower.includes("websocket") ||
    lower.includes("network")
  ) {
    return t("error.connection");
  }
  if (lower.includes("dispositivo") || lower.includes("device")) {
    return t("error.device");
  }
  return raw;
}

export default function Settings() {
  const { t, lang } = useI18n();
  const [tab, setTab] = useState<Tab>("general");
  const [form, setForm] = useState<SettingsType>(DEFAULT_SETTINGS);
  const [hasKey, setHasKey] = useState(false);
  const [apiKey, setApiKey] = useState("");
  const [showKey, setShowKey] = useState(false);
  const [captureDevices, setCaptureDevices] = useState<DeviceInfo[]>([]);
  const [outputDevices, setOutputDevices] = useState<DeviceInfo[]>([]);
  const [detected, setDetected] = useState<string | null>(null);
  const [version, setVersion] = useState("0.1.0");
  const [loading, setLoading] = useState(true);
  const [saving, setSaving] = useState(false);
  const [refreshing, setRefreshing] = useState(false);
  const [toast, setToast] = useState<Toast>(null);
  const [testingKey, setTestingKey] = useState(false);
  const [testingAudio, setTestingAudio] = useState(false);
  const [testingCapture, setTestingCapture] = useState(false);
  const [probe, setProbe] = useState<ProbeResult | null>(null);
  const [restartLanguage, setRestartLanguage] = useState<string | null>(null);
  const toastTimer = useRef<number | null>(null);

  const showToast = useCallback((kind: NonNullable<Toast>["kind"], text: string) => {
    setToast({ kind, text });
    if (toastTimer.current !== null) window.clearTimeout(toastTimer.current);
    toastTimer.current = window.setTimeout(() => setToast(null), 5200);
  }, []);

  const loadDevices = useCallback(
    async (notify = false) => {
      setRefreshing(true);
      try {
        const [captures, outputs] = await Promise.all([
          invoke<DeviceInfo[]>("list_capture_devices"),
          invoke<DeviceInfo[]>("list_output_devices"),
        ]);
        setCaptureDevices(captures);
        setOutputDevices(outputs);
        if (notify) showToast("ok", t("toast.devicesRefreshed"));
      } catch (error) {
        showToast("error", localizeError(error, t));
      } finally {
        setRefreshing(false);
      }
    },
    [showToast, t]
  );

  const load = useCallback(async () => {
    setLoading(true);
    try {
      const boot = await invoke<Bootstrap>("get_bootstrap");
      setForm({ ...DEFAULT_SETTINGS, ...boot.settings });
      setHasKey(boot.has_api_key);
      setDetected(boot.detected_language);
      setVersion(boot.version);
      applyTheme(boot.settings.theme);
      await loadDevices(false);
    } catch (error) {
      showToast("error", localizeError(error, t));
    } finally {
      setLoading(false);
    }
  }, [loadDevices, showToast, t]);

  useEffect(() => {
    void load();
    void invoke("set_window_title", { title: t("settings.windowTitle") }).catch(() => {});
    const onFocus = () => {
      void loadDevices(false);
    };
    window.addEventListener("focus", onFocus);
    const unlisten = listen<LanguagePayload>("translation-language", (event) => {
      if (event.payload.code) setDetected(event.payload.code);
    });
    return () => {
      window.removeEventListener("focus", onFocus);
      void unlisten.then((off) => off());
      if (toastTimer.current !== null) window.clearTimeout(toastTimer.current);
    };
  }, [load, loadDevices, t]);

  const update = <K extends keyof SettingsType>(key: K, value: SettingsType[K]) => {
    setForm((current) => ({ ...current, [key]: value }));
  };

  const save = async (appearanceOnly = false) => {
    setSaving(true);
    try {
      if (!appearanceOnly && apiKey.trim()) {
        await invoke("save_api_key", { key: apiKey.trim() });
        setApiKey("");
        setHasKey(true);
      }
      if (appearanceOnly) {
        await invoke("save_appearance", {
          theme: form.theme,
          lang: form.ui_language,
        });
      } else {
        await invoke("save_settings", { settings: form });
      }
      applyTheme(form.theme);
      localStorage.setItem("lt_theme", form.theme);
      localStorage.setItem("lt_ui_language", form.ui_language);
      if (form.ui_language !== lang) {
        setRestartLanguage(form.ui_language);
      } else {
        showToast("ok", t("toast.saved"));
      }
    } catch (error) {
      showToast("error", localizeError(error, t));
    } finally {
      setSaving(false);
    }
  };

  const validateKey = async () => {
    setTestingKey(true);
    try {
      await invoke<string>("validate_api_key", {
        key: apiKey.trim() || null,
      });
      showToast("ok", t("toast.connectionOk"));
    } catch (error) {
      showToast("error", localizeError(error, t));
    } finally {
      setTestingKey(false);
    }
  };

  const testOutput = async () => {
    setTestingAudio(true);
    try {
      await invoke("test_output", { device: form.output_device_name });
      showToast("ok", t("toast.outputTest"));
    } catch (error) {
      showToast("error", localizeError(error, t));
    } finally {
      setTestingAudio(false);
    }
  };

  const testCapture = async () => {
    setTestingCapture(true);
    setProbe(null);
    try {
      const result = await invoke<ProbeResult>("probe_capture", {
        device: form.capture_device_id,
      });
      setProbe(result);
      if (result.rms > 0.0008) {
        showToast("ok", t("toast.captureTest"));
      } else {
        showToast("info", t("toast.captureSilent"));
      }
    } catch (error) {
      showToast("error", localizeError(error, t));
    } finally {
      setTestingCapture(false);
    }
  };

  const openApiKeys = () => {
    void invoke("open_url", { url: API_KEY_URL });
  };

  const restartNow = () => {
    void invoke("restart_app").catch((error) => {
      showToast("error", localizeError(error, t));
    });
  };

  const chooseTheme = (theme: string) => {
    update("theme", theme);
    applyTheme(theme);
    localStorage.setItem("lt_theme", theme);
    void invoke("save_theme", { theme }).catch((error) => {
      showToast("error", localizeError(error, t));
    });
  };

  const sameDevice = useMemo(() => {
    const captureName =
      form.capture_device_id === "default"
        ? captureDevices.find((device) => device.is_default)?.name
        : captureDevices.find((device) => device.id === form.capture_device_id)?.name;
    const outputName =
      form.output_device_name || outputDevices.find((device) => device.is_default)?.name;
    return Boolean(captureName && outputName && captureName === outputName);
  }, [captureDevices, outputDevices, form.capture_device_id, form.output_device_name]);

  const captureMissing =
    form.capture_device_id !== "default" &&
    captureDevices.length > 0 &&
    !captureDevices.some((device) => device.id === form.capture_device_id);
  const outputMissing =
    form.output_device_name !== "" &&
    outputDevices.length > 0 &&
    !outputDevices.some((device) => device.name === form.output_device_name);
  const targetName = languageName(form.target_language);

  return (
    <div className="settings-shell">
      <aside className="settings-sidebar">
        <div className="brand">
          <div className="brand-mark">
            <GhostIcon size={19} strokeWidth={1.7} />
          </div>
          <div>
            <div className="brand-name">LiveTranslate</div>
            <div className="brand-sub">{t("app.tagline")}</div>
          </div>
        </div>

        <nav className="settings-nav" aria-label={t("nav.general")}>
          <button className={tab === "general" ? "active" : ""} onClick={() => setTab("general")}>
            {t("nav.general")}
          </button>
          <button className={tab === "audio" ? "active" : ""} onClick={() => setTab("audio")}>
            {t("nav.audio")}
          </button>
          <button
            className={tab === "appearance" ? "active" : ""}
            onClick={() => setTab("appearance")}
          >
            {t("nav.appearance")}
          </button>
          <button
            className={tab === "advanced" ? "active" : ""}
            onClick={() => setTab("advanced")}
          >
            {t("nav.advanced")}
          </button>
        </nav>

        <div className="sidebar-footer">
          <div className={`key-chip ${hasKey ? "ok" : "missing"}`}>
            <span className="key-dot" />
            {hasKey ? t("general.apiConfigured") : t("general.apiPending")}
          </div>
          <div className="version">v{version}</div>
        </div>
      </aside>

      <main className="settings-main">
        <header className="settings-header">
          <div>
            <h1>
              {tab === "general"
                ? t("nav.general")
                : tab === "audio"
                  ? t("nav.audio")
                  : tab === "appearance"
                    ? t("nav.appearance")
                    : t("nav.advanced")}
            </h1>
            <p>
              {tab === "general"
                ? t("general.apiDesc")
                : tab === "audio"
                  ? t("audio.desc")
                  : tab === "appearance"
                    ? t("appearance.desc")
                    : t("advanced.desc")}
            </p>
          </div>
        </header>

        <div className="settings-content">
          {loading ? (
            <div className="loading-card">
              <span className="spinner dark" />
              {t("settings.loading")}
            </div>
          ) : (
            <>
              {!hasKey && (
                <div className="banner">
                  <div className="banner-icon">
                    <AlertIcon size={18} />
                  </div>
                  <div>
                    <strong>{t("general.missingKeyTitle")}</strong>
                    <p>{t("general.missingKeyDesc")}</p>
                  </div>
                  <button className="text-button" onClick={openApiKeys}>
                    {t("general.apiOpen")} <ExternalIcon />
                  </button>
                </div>
              )}

              {tab === "general" && (
                <>
                  <section className="card">
                    <div className="card-title">
                      <div>
                        <h2>{t("general.apiTitle")}</h2>
                        <p>{t("general.apiDesc")}</p>
                      </div>
                      <span className={`pill-badge ${hasKey ? "ok" : "missing"}`}>
                        {hasKey ? t("general.apiConfigured") : t("general.apiPending")}
                      </span>
                    </div>

                    <label className="field">
                      <span className="field-label">{t("general.apiLabel")}</span>
                      <div className="input-row">
                        <div className="input-with-action">
                          <input
                            type={showKey ? "text" : "password"}
                            value={apiKey}
                            onChange={(event) => setApiKey(event.target.value)}
                            placeholder={
                              hasKey
                                ? t("general.apiPlaceholderSaved")
                                : t("general.apiPlaceholder")
                            }
                            spellCheck={false}
                            autoComplete="off"
                          />
                          <button
                            className="input-action"
                            onClick={() => setShowKey((value) => !value)}
                            aria-label={showKey ? "•••" : "•••"}
                            type="button"
                          >
                            {showKey ? <EyeOffIcon /> : <EyeIcon />}
                          </button>
                        </div>
                        <button
                          className="secondary-button"
                          onClick={validateKey}
                          disabled={testingKey || (!apiKey.trim() && !hasKey)}
                        >
                          {testingKey ? <span className="spinner dark" /> : <RefreshIcon />}
                          {t("general.apiTest")}
                        </button>
                      </div>
                      <span className="field-help">{t("general.apiHelp")}</span>
                    </label>
                  </section>

                  <section className="card">
                    <div className="card-title">
                      <div>
                        <h2>{t("general.langTitle")}</h2>
                        <p>{t("general.langDesc")}</p>
                      </div>
                    </div>

                    <div className="field-grid">
                      <label className="field">
                        <span className="field-label">{t("general.source")}</span>
                        <div className="readonly-box">
                          <span className="readonly-value">{t("general.sourceAuto")}</span>
                          <span className="readonly-hint">
                            {detected
                              ? t("general.sourceDetected")
                                  .replace("{language}", languageName(detected))
                                  .replace("{code}", shortCode(detected))
                              : t("general.sourceUnknown")}
                          </span>
                        </div>
                      </label>

                      <label className="field">
                        <span className="field-label">{t("general.target")}</span>
                        <select
                          value={form.target_language}
                          onChange={(event) => update("target_language", event.target.value)}
                        >
                          {LANGUAGES.map((language) => (
                            <option key={language.code} value={language.code}>
                              {language.native} — {language.code}
                            </option>
                          ))}
                        </select>
                        <span className="field-help">
                          {t("general.targetHelp").replace("{language}", targetName)}
                        </span>
                      </label>
                    </div>
                  </section>

                  <section className="card compact">
                    <div className="inline-info">
                      <CheckIcon />
                      <div>
                        <strong>{t("general.privacyTitle")}</strong>
                        <p>{t("general.privacyDesc")}</p>
                      </div>
                    </div>
                  </section>
                </>
              )}

              {tab === "audio" && (
                <>
                  <section className="card">
                    <div className="card-title">
                      <div>
                        <h2>{t("audio.title")}</h2>
                        <p>{t("audio.desc")}</p>
                      </div>
                      <button
                        className="refresh-button"
                        onClick={() => void loadDevices(true)}
                        disabled={refreshing}
                      >
                        <RefreshIcon className={refreshing ? "spin" : ""} />
                        {refreshing ? t("audio.refreshing") : t("audio.refresh")}
                      </button>
                    </div>

                    <label className="field">
                      <span className="field-label">
                        <MonitorIcon size={15} /> {t("audio.capture")}
                      </span>
                      <select
                        value={form.capture_device_id}
                        onChange={(event) => update("capture_device_id", event.target.value)}
                      >
                        <option value="default">{t("audio.defaultCapture")}</option>
                        {captureDevices.map((device) => (
                          <option key={device.id} value={device.id}>
                            {device.name}
                            {device.is_default ? " ★" : ""}
                          </option>
                        ))}
                        {captureMissing && (
                          <option value={form.capture_device_id}>
                            {form.capture_device_id} (—)
                          </option>
                        )}
                      </select>
                      <span className="field-help">{t("audio.captureHelp")}</span>
                    </label>

                    <label className="field">
                      <span className="field-label">
                        <SpeakerIcon size={15} /> {t("audio.output")}
                      </span>
                      <select
                        value={form.output_device_name}
                        onChange={(event) => update("output_device_name", event.target.value)}
                      >
                        <option value="">{t("audio.defaultOutput")}</option>
                        {outputDevices.map((device) => (
                          <option key={device.name} value={device.name}>
                            {device.name}
                            {device.is_default ? " ★" : ""}
                          </option>
                        ))}
                        {outputMissing && (
                          <option value={form.output_device_name}>
                            {form.output_device_name} (—)
                          </option>
                        )}
                      </select>
                      <span className="field-help">{t("audio.outputHelp")}</span>
                    </label>

                    {(sameDevice || captureMissing || outputMissing || captureDevices.length === 0) && (
                      <div className="warning-inline">
                        <AlertIcon size={16} />
                        {captureDevices.length === 0
                          ? t("audio.noDevices")
                          : captureMissing || outputMissing
                            ? t("audio.deviceMissing")
                            : t("audio.sameWarning")}
                      </div>
                    )}

                    <div className="button-row">
                      <button className="secondary-button" onClick={testOutput} disabled={testingAudio}>
                        {testingAudio ? <span className="spinner dark" /> : <SpeakerIcon />}
                        {t("audio.testOutput")}
                      </button>
                      <button
                        className="secondary-button"
                        onClick={testCapture}
                        disabled={testingCapture}
                      >
                        {testingCapture ? <span className="spinner dark" /> : <MonitorIcon />}
                        {t("audio.testCapture")}
                      </button>
                      {probe && (
                        <span className="probe-readout">
                          {t("audio.probe")
                            .replace("{rms}", (probe.rms * 100).toFixed(1))
                            .replace("{peak}", (probe.peak * 100).toFixed(0))}
                        </span>
                      )}
                    </div>
                    <p className="field-help bt-hint">{t("audio.btHint")}</p>
                  </section>

                  <section className="card">
                    <div className="card-title">
                      <div>
                        <h2>{t("audio.volume")}</h2>
                        <p>{t("audio.gainHelp")}</p>
                      </div>
                    </div>

                    <label className="field">
                      <span className="field-label">
                        {t("audio.volume")}{" "}
                        <span className="value-chip">{Math.round(form.output_volume * 100)}%</span>
                      </span>
                      <input
                        type="range"
                        min={0}
                        max={1}
                        step={0.01}
                        value={form.output_volume}
                        onChange={(event) => update("output_volume", Number(event.target.value))}
                      />
                    </label>

                    <label className="field">
                      <span className="field-label">
                        {t("audio.gain")}{" "}
                        <span className="value-chip">
                          {form.input_gain_db > 0 ? "+" : ""}
                          {form.input_gain_db.toFixed(0)} dB
                        </span>
                      </span>
                      <input
                        type="range"
                        min={-12}
                        max={12}
                        step={1}
                        value={form.input_gain_db}
                        onChange={(event) => update("input_gain_db", Number(event.target.value))}
                      />
                      <span className="field-help">{t("audio.gainHelp")}</span>
                    </label>
                  </section>
                </>
              )}

              {tab === "appearance" && (
                <>
                  <section className="card">
                    <div className="card-title">
                      <div>
                        <h2>{t("appearance.themeTitle")}</h2>
                        <p>{t("appearance.themeDesc")}</p>
                      </div>
                    </div>

                    <div className="theme-grid">
                      <button
                        className={`theme-card ${form.theme === "light" ? "active" : ""}`}
                        onClick={() => chooseTheme("light")}
                      >
                        <div className="theme-preview light">
                          <div className="theme-preview-pill">
                            <span className="theme-preview-dot" />
                            <span>AUTO → ES</span>
                            <span className="theme-preview-play" />
                          </div>
                        </div>
                        <span className="theme-card-label">
                          <SunIcon size={15} /> {t("appearance.light")}
                        </span>
                        {form.theme === "light" && <CheckIcon className="theme-check" />}
                      </button>

                      <button
                        className={`theme-card ${form.theme === "dark" ? "active" : ""}`}
                        onClick={() => chooseTheme("dark")}
                      >
                        <div className="theme-preview dark">
                          <div className="theme-preview-pill">
                            <span className="theme-preview-dot" />
                            <span>AUTO → ES</span>
                            <span className="theme-preview-play" />
                          </div>
                        </div>
                        <span className="theme-card-label">
                          <MoonIcon size={15} /> {t("appearance.dark")}
                        </span>
                        {form.theme === "dark" && <CheckIcon className="theme-check" />}
                      </button>
                    </div>
                  </section>

                  <section className="card">
                    <div className="card-title">
                      <div>
                        <h2>{t("appearance.languageTitle")}</h2>
                        <p>{t("appearance.languageDesc")}</p>
                      </div>
                    </div>
                    <label className="field">
                      <span className="field-label">
                        <GlobeIcon size={15} /> {t("appearance.languageTitle")}
                      </span>
                      <select
                        value={form.ui_language}
                        onChange={(event) => update("ui_language", event.target.value)}
                      >
                        {UI_LANGUAGES.map((language) => (
                          <option key={language.code} value={language.code}>
                            {language.native}
                          </option>
                        ))}
                      </select>
                      <span className="field-help">{t("appearance.languageHelp")}</span>
                    </label>
                    <div className="appearance-actions">
                      <button
                        className="primary-action"
                        onClick={() => void save(true)}
                        disabled={saving}
                      >
                        {saving ? <span className="spinner" /> : <CheckIcon />}
                        {t("appearance.save")}
                      </button>
                    </div>
                  </section>

                </>
              )}

              {tab === "advanced" && (
                <>
                  <section className="card">
                    <div className="card-title">
                      <div>
                        <h2>{t("advanced.behavior")}</h2>
                        <p>{t("advanced.desc")}</p>
                      </div>
                    </div>

                    <label className="switch-row">
                      <div>
                        <strong>{t("advanced.autoReconnect")}</strong>
                        <span>{t("advanced.autoReconnectHelp")}</span>
                      </div>
                      <input
                        type="checkbox"
                        checked={form.auto_reconnect}
                        onChange={(event) => update("auto_reconnect", event.target.checked)}
                      />
                    </label>

                    <label className="switch-row">
                      <div>
                        <strong>{t("advanced.echo")}</strong>
                        <span>{t("advanced.echoHelp")}</span>
                      </div>
                      <input
                        type="checkbox"
                        checked={form.echo_target_language}
                        onChange={(event) =>
                          update("echo_target_language", event.target.checked)
                        }
                      />
                    </label>
                  </section>

                  <section className="card">
                    <div className="card-title">
                      <div>
                        <h2>{t("advanced.diagnostics")}</h2>
                        <p>{t("advanced.desc")}</p>
                      </div>
                    </div>
                    <dl className="diagnostics">
                      <div>
                        <dt>{t("advanced.model")}</dt>
                        <dd dir="ltr">gemini-3.5-live-translate-preview</dd>
                      </div>
                      <div>
                        <dt>{t("advanced.endpoint")}</dt>
                        <dd dir="ltr">Gemini Developer API (v1beta)</dd>
                      </div>
                      <div>
                        <dt>{t("advanced.detected")}</dt>
                        <dd>
                          {detected
                            ? `${languageName(detected)} (${shortCode(detected)})`
                            : "—"}
                        </dd>
                      </div>
                      <div>
                        <dt>{t("advanced.apiKey")}</dt>
                        <dd>{hasKey ? t("advanced.apiKeyStored") : t("advanced.apiKeyNotSet")}</dd>
                      </div>
                      <div>
                        <dt>{t("advanced.cost")}</dt>
                        <dd>{t("advanced.costValue")}</dd>
                      </div>
                      <div>
                        <dt>{t("advanced.version")}</dt>
                        <dd>{version}</dd>
                      </div>
                      <div>
                        <dt>{t("advanced.license")}</dt>
                        <dd>MIT</dd>
                      </div>
                    </dl>
                  </section>

                  <section className="card compact">
                    <div className="inline-info">
                      <AlertIcon />
                      <div>
                        <strong>{t("advanced.limits")}</strong>
                        <p>{t("advanced.limitsText")}</p>
                      </div>
                    </div>
                  </section>
                </>
              )}
            </>
          )}
        </div>

        <footer className="settings-footer">
          <span className="footer-note">{t("settings.footerNote")}</span>
          <button className="primary-action" onClick={() => void save(false)} disabled={saving || loading}>
            {saving ? <span className="spinner" /> : <CheckIcon />}
            {t("settings.save")}
          </button>
        </footer>
      </main>

      {toast && (
        <div className={`toast ${toast.kind}`} role="status">
          {toast.kind === "ok" ? (
            <CheckIcon />
          ) : toast.kind === "error" ? (
            <AlertIcon />
          ) : (
            <RefreshIcon />
          )}
          <span>{toast.text}</span>
        </div>
      )}

      {restartLanguage && (
        <div className="modal-backdrop" onClick={() => setRestartLanguage(null)}>
          <div className="modal" onClick={(event) => event.stopPropagation()}>
            <div className="modal-icon">
              <GhostIcon size={30} />
            </div>
            <h2>{t("appearance.restartTitle")}</h2>
            <p>
              {t("appearance.restartDesc").replace(
                "{language}",
                uiLanguageNative(restartLanguage)
              )}
            </p>
            <div className="modal-actions">
              <button className="ghost-button" onClick={() => setRestartLanguage(null)}>
                {t("appearance.later")}
              </button>
              <button className="primary-action" onClick={restartNow}>
                {t("appearance.restartNow")}
              </button>
            </div>
          </div>
        </div>
      )}
    </div>
  );
}
