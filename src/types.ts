export type Settings = {
  target_language: string;
  capture_device_id: string;
  output_device_name: string;
  output_volume: number;
  input_gain_db: number;
  echo_target_language: boolean;
  auto_reconnect: boolean;
  ui_language: string;
  theme: string;
};

export type DeviceInfo = {
  id: string;
  name: string;
  is_default: boolean;
};

export type Bootstrap = {
  settings: Settings;
  has_api_key: boolean;
  active: boolean;
  state: string;
  message: string;
  detected_language: string | null;
  version: string;
};

export type StatusPayload = {
  state: string;
  message: string;
};

export type LanguagePayload = {
  code: string;
  name?: string;
};

export type ErrorPayload = {
  message: string;
  retryable: boolean;
};

export type ProbeResult = {
  rms: number;
  peak: number;
  seconds: number;
};
