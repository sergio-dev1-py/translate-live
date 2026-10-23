import React from "react";
import ReactDOM from "react-dom/client";
import { getCurrentWindow } from "@tauri-apps/api/window";
import Pill from "./Pill";
import Settings from "./Settings";
import { I18nProvider } from "./i18n";
import "./styles.css";

const label = getCurrentWindow().label;
document.documentElement.dataset.window = label;
document.body.dataset.window = label;

ReactDOM.createRoot(document.getElementById("root")!).render(
  <React.StrictMode>
    <I18nProvider>{label === "settings" ? <Settings /> : <Pill />}</I18nProvider>
  </React.StrictMode>
);
