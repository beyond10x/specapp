import {useEffect, useMemo, useRef, useState, type ReactNode} from 'react';
import {SpecLink} from './config.js';
import {styleOf} from './Icon.js';
import {useScrollRegion} from './scroll.js';
import type {KpiCatalogueData, KpiRow, Presentation} from './types.js';

type Key = 'id' | 'block' | 'measure' | 'unit' | 'direction' | 'gate' | 'status';

const GATE = 'gate';
const NO_GATE = 'no gate';

/** The filters of the table, kept in the address so that every state of it has a URL. */
interface Filters {
  q: string;
  block: string;
  measure: string[];
  status: string[];
  gate: string[];
}

const EMPTY: Filters = {q: '', block: '', measure: [], status: [], gate: []};

function readFilters(search: string): Filters {
  const p = new URLSearchParams(search);
  const list = (k: string) => (p.get(k) ?? '').split(',').filter(Boolean);
  return {q: p.get('q') ?? '', block: p.get('block') ?? '', measure: list('measure'), status: list('status'), gate: list('gate')};
}

function writeFilters(f: Filters): string {
  const p = new URLSearchParams();
  if (f.q) p.set('q', f.q);
  if (f.block) p.set('block', f.block);
  if (f.measure.length) p.set('measure', f.measure.join(','));
  if (f.status.length) p.set('status', f.status.join(','));
  if (f.gate.length) p.set('gate', f.gate.join(','));
  const s = p.toString();
  return s ? `?${s}` : '';
}

/** The values of a field in the order they first appear. */
function values(rows: KpiRow[], pick: (r: KpiRow) => string): string[] {
  const out: string[] = [];
  for (const r of rows) {
    const v = pick(r);
    if (!out.includes(v)) out.push(v);
  }
  return out;
}

function layerOf(r: KpiRow): number {
  return r.layer ?? Number.MAX_SAFE_INTEGER;
}

function toggle(set: string[], value: string): string[] {
  return set.includes(value) ? set.filter((v) => v !== value) : [...set, value];
}

function Chips({
  label,
  options,
  selected,
  onChange,
  counts,
}: {
  label: string;
  options: string[];
  selected: string[];
  onChange: (next: string[]) => void;
  counts: Record<string, number>;
}): ReactNode {
  return (
    <div className="sa-kpis__group" role="group" aria-label={label}>
      <span>{label}</span>
      {options.map((v) => (
        <button key={v} type="button" className="sa-chip" aria-pressed={selected.includes(v)} onClick={() => onChange(toggle(selected, v))}>
          {v || '–'}
          <small>{counts[v] ?? 0}</small>
        </button>
      ))}
    </div>
  );
}

/**
 * Every KPI of the catalogue in one table: filter by block, by the pack's measure, by status and
 * by gate, search, sort. The pack's words are data: the measure column carries the pack's name of
 * the field, and the values of every filter are the values the catalogue holds.
 */
export function KpiCatalogue({data, presentation}: {data: KpiCatalogueData; presentation?: Presentation}): ReactNode {
  const rows = data.kpis;
  const [filters, setFilters] = useState<Filters>(EMPTY);
  const [sort, setSort] = useState<{key: Key; up: boolean}>({key: 'block', up: true});
  const loaded = useRef(false);
  const scroller = useRef<HTMLDivElement>(null);
  const region = useScrollRegion(scroller, 'KPI table');

  useEffect(() => {
    if (typeof window === 'undefined') return;
    setFilters(readFilters(window.location.search));
    loaded.current = true;
  }, []);
  useEffect(() => {
    if (typeof window === 'undefined' || !loaded.current) return;
    const next = writeFilters(filters);
    if (next !== window.location.search) {
      window.history.replaceState(window.history.state, '', `${window.location.pathname}${next}${window.location.hash}`);
    }
  }, [filters]);

  const measures = useMemo(() => values(rows, (r) => r.measure), [rows]);
  const statuses = useMemo(() => values(rows, (r) => r.status), [rows]);
  const blocks = useMemo(() => {
    const seen = new Map<string, KpiRow>();
    for (const r of rows) if (!seen.has(r.block)) seen.set(r.block, r);
    return [...seen.values()].sort((a, b) => layerOf(a) - layerOf(b) || a.block.localeCompare(b.block));
  }, [rows]);

  const compare = (a: KpiRow, b: KpiRow, key: Key): number => {
    switch (key) {
      case 'block':
        return layerOf(a) - layerOf(b) || a.block.localeCompare(b.block) || a.id.localeCompare(b.id);
      case 'measure':
        return measures.indexOf(a.measure) - measures.indexOf(b.measure) || a.id.localeCompare(b.id);
      case 'status':
        return statuses.indexOf(a.status) - statuses.indexOf(b.status) || a.id.localeCompare(b.id);
      case 'gate':
        return Number(b.gate) - Number(a.gate) || a.id.localeCompare(b.id);
      default:
        return String(a[key]).localeCompare(String(b[key])) || a.id.localeCompare(b.id);
    }
  };

  const shown = useMemo(() => {
    const q = filters.q.trim().toLowerCase();
    const out = rows.filter(
      (r) =>
        (!filters.block || r.block === filters.block) &&
        (filters.measure.length === 0 || filters.measure.includes(r.measure)) &&
        (filters.status.length === 0 || filters.status.includes(r.status)) &&
        (filters.gate.length === 0 || filters.gate.includes(r.gate ? GATE : NO_GATE)) &&
        (!q || `${r.id} ${r.summary} ${r.how} ${r.block}`.toLowerCase().includes(q)),
    );
    out.sort((a, b) => (sort.up ? 1 : -1) * compare(a, b, sort.key));
    return out;
    // compare reads measures and statuses, which follow rows
  }, [rows, filters, sort, measures, statuses]);

  const count = (pick: (r: KpiRow) => string) => {
    const c: Record<string, number> = {};
    for (const r of rows) c[pick(r)] = (c[pick(r)] ?? 0) + 1;
    return c;
  };

  if (rows.length === 0) {
    return <p className="sa-kpis__empty">The KPI catalogue holds no KPI.</p>;
  }
  const columns: [Key, string][] = [
    ['id', 'KPI'],
    ['block', 'block'],
    ['measure', data.measure],
    ['unit', 'unit'],
    ['direction', 'better'],
    ['gate', 'gate'],
    ['status', 'status'],
  ];
  return (
    <section className="sa-kpis" aria-label="KPI catalogue">
      <div className="sa-kpis__controls">
        <label className="sa-kpis__field">
          Search
          <input
            type="search"
            value={filters.q}
            placeholder="name, what it measures, how"
            onChange={(e) => setFilters({...filters, q: e.target.value})}
          />
        </label>
        <label className="sa-kpis__field">
          Block
          <select value={filters.block} onChange={(e) => setFilters({...filters, block: e.target.value})}>
            <option value="">every block</option>
            {blocks.map((r) => (
              <option key={r.block} value={r.block}>
                {r.planned ? `${r.block} (planned)` : r.layer !== null ? `${r.layer} · ${r.block}` : r.block}
              </option>
            ))}
          </select>
        </label>
        <div className="sa-kpis__chips">
          <Chips
            label={data.measure}
            options={measures}
            selected={filters.measure}
            onChange={(measure) => setFilters({...filters, measure})}
            counts={count((r) => r.measure)}
          />
          <Chips
            label="Status"
            options={statuses}
            selected={filters.status}
            onChange={(status) => setFilters({...filters, status})}
            counts={count((r) => r.status)}
          />
          <Chips
            label="Gate"
            options={[GATE, NO_GATE]}
            selected={filters.gate}
            onChange={(gate) => setFilters({...filters, gate})}
            counts={count((r) => (r.gate ? GATE : NO_GATE))}
          />
        </div>
      </div>
      <p className="sa-kpis__count" aria-live="polite">
        {shown.length} of {rows.length} KPIs
      </p>
      <div className="sa-kpis__scroll" ref={scroller} {...region}>
        <table>
          <thead>
            <tr>
              {columns.map(([key, label]) => (
                <th key={key} aria-sort={sort.key === key ? (sort.up ? 'ascending' : 'descending') : 'none'}>
                  <button type="button" onClick={() => setSort({key, up: sort.key === key ? !sort.up : true})}>
                    {label}
                    {sort.key === key ? (sort.up ? ' ▲' : ' ▼') : ''}
                  </button>
                </th>
              ))}
            </tr>
          </thead>
          <tbody>
            {shown.map((r) => {
              const style = styleOf(presentation, r.block);
              const block = (
                <>
                  <i />
                  {style.label}
                </>
              );
              return (
                <tr key={r.id} id={`kpi-${r.id}`}>
                  <td className="sa-kpis__what">
                    <code>{r.id}</code>
                    <p>{r.summary}</p>
                    {r.how && <small>How: {r.how}</small>}
                    {r.modes.length > 0 && (
                      <small>
                        By {data.modes}: {r.modes.map((m) => `${m.mode} ${m.direction}`).join(', ')}
                      </small>
                    )}
                    {Object.entries(r.extra).map(([k, v]) => (
                      <small key={k}>
                        {k}: {v}
                      </small>
                    ))}
                  </td>
                  <td>
                    {r.route ? (
                      <SpecLink to={r.route} className={`sa-kpis__block sa-c-${style.colour}`}>
                        {block}
                      </SpecLink>
                    ) : (
                      <span className={`sa-kpis__block sa-c-${style.colour}`}>{block}</span>
                    )}
                    {r.planned && (
                      <>
                        {' '}
                        <span className="sa-tag sa-tag--planned">planned</span>
                      </>
                    )}
                  </td>
                  <td>{r.measure || '–'}</td>
                  <td>{r.unit || '–'}</td>
                  <td>{r.direction || '–'}</td>
                  <td>{r.gate ? <span className="sa-tag sa-tag--gate">gate</span> : '–'}</td>
                  <td>{r.status || '–'}</td>
                </tr>
              );
            })}
          </tbody>
        </table>
        {shown.length === 0 && <p className="sa-kpis__empty">No KPI matches these filters.</p>}
      </div>
    </section>
  );
}
