/**
 * The data the components draw. It is written by `metaspec docs build` (site.json,
 * pages/<id>.json) and later served by the studio; the shapes follow the schemas in schema/ of
 * metaspec: metaspec-site/1 (Presentation), metaspec-page/1 (the data of each component),
 * metaspec-scene/1 (Scene) and metaspec-kpis/1 (KpiRow).
 */

/** How one block is drawn, from the pack's manifest through metaspec. */
export interface BlockStyle {
  label: string;
  colour: string;
  icon: string | null;
  group: string;
  layer: number | null;
  layer_name: string | null;
  route: string | null;
}

export interface Group {
  label: string;
  colour: string;
  icon: string | null;
}

/** The presentation table of a pack: one per site, read by every component. */
export interface Presentation {
  blocks: Record<string, BlockStyle>;
  groups: Group[];
  /** The 14 by 14 stroke path of each icon metaspec draws; any other icon is its first letter. */
  icons: Record<string, string>;
  vocabulary: Record<string, string>;
}

export interface MapRow {
  layer: number | null;
  name: string;
  y: number;
  h: number;
}

export interface MapBlock {
  name: string;
  label: string;
  layer: number | null;
  summary: string;
  route: string | null;
  kpis: number;
  entities: number;
  commands: number;
  events: number;
  views: number;
  uses: string[];
  used_by: string[];
  band: boolean;
  x: number;
  y: number;
  w: number;
  h: number;
}

export interface MapEdge {
  from: string;
  to: string;
  count: number;
  d: string;
}

export interface LayerMapData {
  width: number;
  height: number;
  rows: MapRow[];
  blocks: MapBlock[];
  edges: MapEdge[];
}

export type Counts = Partial<Record<'entities' | 'commands' | 'events' | 'views' | 'refusals' | 'kpis', number>>;

export interface BlockHeaderData {
  name: string;
  label: string;
  layer: number | null;
  layer_name: string | null;
  planned: boolean;
  counts: Counts;
}

export interface Transition {
  name: string;
  from: string[];
  to: string;
}

export interface LifecycleData {
  entity: string;
  initial: string;
  states: string[];
  terminal?: string[] | null;
  transitions: Transition[];
}

export interface KpiMode {
  mode: string;
  direction: string;
}

export interface KpiRow {
  id: string;
  block: string;
  measure: string;
  unit: string;
  summary: string;
  how: string;
  direction: string;
  modes: KpiMode[];
  gate: boolean;
  reads: string[];
  status: string;
  extra: Record<string, string>;
  layer: number | null;
  planned: boolean;
  route: string | null;
}

export interface KpiCatalogueData {
  format: string;
  /** The pack's name of the measure field, shown as its column. */
  measure: string;
  /** The pack's name of the modes field. */
  modes: string;
  kpis: KpiRow[];
}

export interface Rect {
  x: number;
  y: number;
  w: number;
  h: number;
}

export interface SceneBox extends Rect {
  id: string;
  block?: string;
  label: string;
  lines: string[];
  colour: string;
  icon?: string;
  group: string;
  parent?: string;
  container: boolean;
  on_border: boolean;
  ghost: boolean;
  findings: number[];
  title: string;
}

export interface SceneEdge {
  id: string;
  from: string;
  to: string;
  carries?: string;
  label: string;
  points: [number, number][];
  label_at: Rect;
  added_by?: string;
  findings: number[];
  title: string;
}

export interface SceneMark {
  finding: number;
  rule: string;
  gate: boolean;
  subject: string;
  x: number;
  y: number;
  w: number;
}

export interface LegendEntry {
  label: string;
  colour: string;
  icon?: string;
  blocks: string[];
}

/** A laid-out view of a document, metaspec-scene/1. */
export interface Scene {
  format: string;
  document: string;
  view: {id: string; title: string; layout: string; implicit: boolean};
  width: number;
  height: number;
  rows: {title: string; y: number; h: number}[];
  boxes: SceneBox[];
  edges: SceneEdge[];
  marks: SceneMark[];
  legend: LegendEntry[];
}

export interface FindingRow {
  n: number;
  rule: string;
  gate: boolean;
  subject: string;
  message: string;
  drawn_in: string[];
}

export interface FindingsData {
  findings: FindingRow[];
  views: {id: string; title: string}[];
}

export interface RecordTableData {
  title: string;
  at: string;
  key: string | null;
  columns: string[];
  rows: string[][];
}

/** A run of the pack's tool, metaspec-result/1: baseline and best with their KPI values. */
export interface ResultData {
  baseline?: {fitness?: number; kpis?: Record<string, {value?: number | null; kind?: string}>};
  best?: {fitness?: number; kpis?: Record<string, {value?: number | null; kind?: string}>} | null;
  variants?: unknown[];
}
