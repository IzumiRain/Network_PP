// Mirrors the serde structs in src-tauri/src/commands/*.

export interface CidrInfo {
  input: string;
  ip: string;
  prefix: number;
  network: string;
  broadcast: string;
  netmask: string;
  wildcard: string;
  first_host: string;
  last_host: string;
  total_addresses: number;
  usable_hosts: number;
  ip_class: string;
  address_type: string;
  netmask_binary: string;
  ip_binary: string;
  network_int: number;
}

export interface PortInfo {
  port: number;
  protocol: string;
  service: string;
  description: string;
}

export interface PortReport {
  port: number;
  range: string;
  assignments: PortInfo[];
  notes: string[];
}

export interface EthernetHeader {
  destination: string;
  source: string;
  ethertype: number;
  ethertype_name: string;
}

export interface Ipv4Header {
  version: number;
  ihl: number;
  dscp: number;
  ecn: number;
  total_length: number;
  identification: number;
  dont_fragment: boolean;
  more_fragments: boolean;
  fragment_offset: number;
  ttl: number;
  protocol: number;
  protocol_name: string;
  header_checksum: number;
  source: string;
  destination: string;
  options: string[];
}

export interface Ipv6Header {
  traffic_class: number;
  flow_label: number;
  payload_length: number;
  next_header: number;
  next_header_name: string;
  hop_limit: number;
  source: string;
  destination: string;
}

export interface TcpFlags {
  fin: boolean;
  syn: boolean;
  rst: boolean;
  psh: boolean;
  ack: boolean;
  urg: boolean;
  ece: boolean;
  cwr: boolean;
}


export interface TcpHeader {
  source_port: number;
  destination_port: number;
  sequence: number;
  acknowledgment: number;
  data_offset: number;
  flags: TcpFlags;
  flag_names: string[];
  window_size: number;
  checksum: number;
  urgent_pointer: number;
  options: string[];
}

export interface UdpHeader {
  source_port: number;
  destination_port: number;
  length: number;
  checksum: number;
}

export interface IcmpHeader {
  message_type: number;
  code: number;
  type_name: string;
  rest_of_header: string;
}

export interface HttpRequest {
  method: string;
  target: string;
  version: string;
  headers: [string, string][];
  body: string;
}

export interface HttpResponse {
  version: string;
  status_code: number;
  reason: string;
  headers: [string, string][];
  body: string;
}

export type NetworkLayer =
  | { kind: "ipv4"; data: Ipv4Header }
  | { kind: "ipv6"; data: Ipv6Header };

export type TransportLayer =
  | { kind: "tcp"; data: TcpHeader }
  | { kind: "udp"; data: UdpHeader }
  | { kind: "icmp"; data: IcmpHeader };

export type AppLayer =
  | { kind: "http-request"; data: HttpRequest }
  | { kind: "http-response"; data: HttpResponse }
  | { kind: "raw"; data: null };

export interface Packet {
  link: EthernetHeader | null;
  network: NetworkLayer | null;
  transport: TransportLayer | null;
  application: AppLayer | null;
  payload_hex: string;
  payload_ascii: string;
  total_length: number;
}

export type PortState = "open" | "closed" | "filtered";

export interface PortScanResult {
  port: number;
  state: PortState;
  service: string;
  banner: string | null;
}

export interface HostResult {
  ip: string;
  hostname: string | null;
  latency_ms: number | null;
  open_ports: number[];
}

export interface PingResult {
  ip: string;
  up: boolean;
  latency_ms: number | null;
}

export interface ArpEntry {
  ip: string;
  mac: string;
  vendor: string;
  state: string;
}

export interface LocalNetworkInfo {
  local_ip: string;
  suggested_cidr: string;
}

export interface Connection {
  proto: "tcp" | "udp";
  local_addr: string;
  local_port: number;
  remote_addr: string;
  remote_port: number | null;
  state: string;
  pid: number | null;
}

export interface NetstatSnapshot {
  connections: Connection[];
  total: number;
  by_state: [string, number][];
}

// ---------- content & packs (v0.2) ----------

export interface Entity {
  id: string;
  pack_id: string;
  kind: string;
  title: string;
  category: string;
  tags: string[];
  risk: string | null;
  version: string | null;
  body: string;
  /** v2 document model: `##` sections of `body`, each with its own tags. */
  sections: EntitySection[];
}

/** One `##` section of a document entity (see docs/PACK-SPEC.md v2). */
export interface EntitySection {
  /** Heading text; empty string for the preamble before the first `##`. */
  title: string;
  tags: string[];
  body: string;
}

/** Result of the backend's generic category-path parser. */
export interface CategoryPath {
  category: string;
  root: string;
  segments: { slug: string; role: string; path: string }[];
}

export interface SearchHit {
  id: string;
  kind: string;
  title: string;
  category: string;
  pack: string;
  snippet: string;
}

export interface HistoryEntry {
  id: string;
  title: string;
  kind: string;
  visited_at: number;
}

export interface PackRow {
  id: string;
  title: string;
  version: string;
  installed_at: number;
  entity_count: number;
  source: string;
}

export interface RegistryEntry {
  id: string;
  title: string;
  version: string;
  description: string;
  size: number;
  sha256: string;
  url: string;
  min_app_version: string;
}

export interface InstallReport {
  id: string;
  title: string;
  version: string;
  entity_count: number;
  warnings: string[];
}

export interface AppInfo {
  version: string;
  os: string;
  arch: string;
}

export interface StoreStats {
  entities: number;
  packs: number;
  favorites: number;
  history: number;
}
