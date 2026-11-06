// 0.3 tool-expansion types (mirrors src-tauri commands).

export interface Ipv6Info {
  input: string;
  compressed: string;
  expanded: string;
  prefix: number;
  network: string;
  first_address: string;
  last_address: string;
  total_addresses: string;
  address_type: string;
  binary: string;
}

export interface TransferResult {
  size_bytes: number;
  bandwidth_mbps: number;
  seconds: number;
  human_time: string;
}

export interface MtuResult {
  mtu: number;
  tcp_mss: number;
  udp_payload_max: number;
  ipv4_min_mtu: number;
  pings_bytes: string;
}

export interface OverheadResult {
  payload_bytes: number;
  frame_bytes: number;
  efficiency_percent: number;
  packets: number;
  total_wire_bytes: number;
}

export interface NumberResult {
  decimal: string;
  binary: string;
  hex: string;
  octal: string;
  bits: number;
}

export interface Ipv4Forms {
  dotted: string;
  integer: string;
  binary: string;
  hex: string;
}

export interface MacForms {
  normalized: string;
  windows: string;
  cisco: string;
  bare: string;
  oui: string;
  nic: string;
}

export interface TimestampResult {
  unix: number;
  iso_utc: string;
  iso_local_hint: string;
}

export interface Generated {
  value: string;
  count: number;
  extra: string | null;
}

export interface RegexMatch {
  text: string;
  start: number;
  end: number;
  groups: string[];
}

export interface RegexResult {
  valid: boolean;
  error: string | null;
  matches: RegexMatch[];
  match_count: number;
}

export interface DiffLine {
  op: "same" | "add" | "del";
  text: string;
  line_a: number | null;
  line_b: number | null;
}

export interface JsonVars {
  valid: boolean;
  formatted: string;
  error: string | null;
}
