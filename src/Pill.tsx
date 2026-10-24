import { useCallback, useEffect, useState, type MouseEvent } from "react";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { GearIcon, GhostIcon, PlayIcon, StopIcon } from "./icons";
import { shortCode } from "./languages";
import { applyTheme, useI18n } from "./i18n";
import type { Bootstrap, LanguagePayload, StatusPayload } from "./types";

type Status = "idle" | "connecting" | "live" | "reconnecting" | "error";

const ACTIVE_STATES: Status[] = ["connecting", "live", "reconnecting"];

const STATE_KEYS: Record<Status, string> = {
  idle: "pill.ready",
  connecting: "pill.connecting",
  live: "pill.live",
  reconnecting: "pill.reconnecting",
  error: "pill.error",
};

export default function Pill() {
  const { t, dir } = useI18n();
  const [status, setStatus] = useState<Status>("idle");
  const [message, setMessage] = useState("");
  const [target, setTarget] = useState("es");
  const [detected, setDetected] = useState<string | null>(null);
  const [hasKey, setHasKey] = useState(true);
  const [busy, setBusy] = useState(false);

  const isActive = ACTIVE_STATES.includes(status);
  const sourceCode = detected ? shortCode(detected) : "AUTO";

  const load = useCallback(async () => {
    try {
      const boot = await invoke<Bootstrap>("get_bootstrap");
      setTarget(boot.settings.target_language);
      setDetected(boot.detected_language);
      setHasKey(boot.has_api_key);
      setStatus((boot.state as Status) || "idle");
      setMessage(boot.message || "");
      applyTheme(boot.settings.theme);
    } catch (error) {
      setStatus("error");
      setMessage(String(error));
    }
  }, []);

  useEffect(() => {
    void load();

    const unlisteners: Array<Promise<() => void>> = [
      listen<StatusPayload>("translation-status", (event) => {
        setStatus((event.payload.state as Status) || "idle");
        setMessage(event.payload.message || "");
      }),
      listen<LanguagePayload>("translation-language", (event) => {
        if (event.payload.code) setDetected(event.payload.code);
      }),
      listen("settings-changed", () => {
        void load();
      }),
    ];

    return () => {
      for (const pending of unlisteners) {
        void pending.then((unlisten) => unlisten());
      }
    };
  }, [load]);

  const openSettings = useCallback(() => {
    void invoke("open_settings");
  }, []);

  const toggle = useCallback(async () => {
    if (busy) return;
    if (!hasKey) {
      setStatus("error");
      setMessage(t("pill.noKey"));
      openSettings();
      return;
    }

    setBusy(true);
    try {
      if (isActive) {
        await invoke("stop_translation");
      } else {
        setStatus("connecting");
        await invoke("start_translation");
      }
    } catch (error) {
      setStatus("error");
      setMessage(String(error));
    } finally {
      setBusy(false);
    }
  }, [busy, hasKey, isActive, openSettings, t]);

  const startDrag = useCallback((event: MouseEvent) => {
    if (event.button !== 0) return;
    void getCurrentWindow().startDragging();
  }, []);

  const stateLabel = !hasKey
    ? t("pill.noKey")
    : t(STATE_KEYS[status] ?? "pill.ready");
  const tooltip = message ? `${stateLabel} · ${message}` : stateLabel;

  return (
    <div className="pill-shell">
      <div className="pill" data-state={status} data-has-key={hasKey} data-no-key={!hasKey}>
        <button
          className="ghost-handle"
          onMouseDown={startDrag}
          aria-label={t("pill.drag")}
          title={t("pill.drag")}
        >
          <GhostIcon size={10} strokeWidth={1.9} />
        </button>

        <button
          className="status-area"
          onClick={toggle}
          aria-label={isActive ? t("pill.stop") : t("pill.start")}
          title={tooltip}
        >
          <span className="status-dot" data-state={!hasKey ? "error" : status} />
          <span className="lang-pair">
            {sourceCode}
            <span className="lang-arrow">{dir === "rtl" ? "←" : "→"}</span>
            {shortCode(target)}
          </span>
        </button>

        <button
          className="primary-button"
          onClick={toggle}
          disabled={busy}
          aria-label={isActive ? t("pill.stop") : t("pill.start")}
          title={isActive ? t("pill.stop") : t("pill.start")}
        >
          {busy ? <span className="spinner" /> : isActive ? <StopIcon size={9} /> : <PlayIcon size={11} />}
        </button>

        <button
          className="icon-button"
          onClick={openSettings}
          aria-label={t("pill.settings")}
          title={t("pill.settings")}
        >
          <GearIcon size={11} />
        </button>
      </div>
    </div>
  );
}
