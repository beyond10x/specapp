import {useId, useRef, type ReactNode} from 'react';
import {useScrollRegion} from './scroll.js';
import type {LifecycleData, Presentation} from './types.js';

interface Edge {
  from: string;
  to: string;
  label: string;
}

const NODE_H = 28;
const ROW_H = 52;
const CHAR = 7.7;
const LABEL_CHAR = 6.7;

function nodeWidth(name: string): number {
  return Math.max(70, Math.round(name.length * CHAR + 26));
}

/**
 * An entity's lifecycle as a small state diagram: states in columns by their distance from the
 * initial state, transitions as labelled arrows. Arrows that go back or skip a column arc around.
 */
export function Lifecycle({data}: {data: LifecycleData; presentation?: Presentation}): ReactNode {
  const {entity, initial, states, transitions} = data;
  const terminal = data.terminal ?? [];
  const frame = useRef<HTMLDivElement>(null);
  const region = useScrollRegion(frame, `Lifecycle of ${entity}`);
  const uid = useId().replace(/[^a-zA-Z0-9_-]/g, '');
  const merged = new Map<string, Edge>();
  for (const t of transitions) {
    for (const f of t.from) {
      const key = `${f}\u0000${t.to}`;
      const e = merged.get(key);
      if (e) {
        if (!e.label.split(', ').includes(t.name)) e.label += `, ${t.name}`;
      } else merged.set(key, {from: f, to: t.to, label: t.name});
    }
  }
  const edges = [...merged.values()];

  // Columns by breadth-first distance from the initial state.
  const depth = new Map<string, number>([[initial, 0]]);
  const order: string[] = [initial];
  for (let i = 0; i < order.length; i++) {
    const s = order[i];
    for (const e of edges) {
      if (e.from === s && !depth.has(e.to)) {
        depth.set(e.to, (depth.get(s) ?? 0) + 1);
        order.push(e.to);
      }
    }
  }
  const last = Math.max(0, ...depth.values());
  for (const s of states) {
    if (!depth.has(s)) {
      depth.set(s, last + 1);
      order.push(s);
    }
  }
  const columns: string[][] = [];
  for (const s of order) {
    const d = depth.get(s) ?? 0;
    (columns[d] ??= []).push(s);
  }
  const rows = Math.max(1, ...columns.map((c) => c.length));

  const gapAfter = (d: number) => {
    const labels = edges.filter((e) => depth.get(e.from) === d && depth.get(e.to) === d + 1).map((e) => e.label.length);
    return Math.max(64, Math.round(Math.max(0, ...labels) * LABEL_CHAR + 30));
  };
  const hasSelf = edges.some((e) => e.from === e.to);
  const isBack = (e: Edge) => e.from !== e.to && (depth.get(e.to) ?? 0) < (depth.get(e.from) ?? 0);
  // A backward arrow from the top state of its column arcs over the diagram, any other under it.
  const backAbove = (e: Edge) => (columns[depth.get(e.from) ?? 0] ?? []).indexOf(e.from) === 0;
  const hasAbove =
    hasSelf || edges.some((e) => (depth.get(e.to) ?? 0) - (depth.get(e.from) ?? 0) > 1 || (isBack(e) && backAbove(e)));
  const hasBelow = edges.some((e) => isBack(e) && !backAbove(e));
  const hasSide = edges.some((e) => e.from !== e.to && depth.get(e.to) === depth.get(e.from));
  const top = hasAbove ? 34 : 10;
  const left = 30;

  const pos = new Map<string, {x: number; y: number; w: number}>();
  let x = left;
  columns.forEach((col, d) => {
    const w = Math.max(...col.map(nodeWidth));
    col.forEach((s, i) => {
      const y = top + ((rows - col.length) * ROW_H) / 2 + i * ROW_H;
      const sw = nodeWidth(s);
      pos.set(s, {x: x + (w - sw) / 2, y, w: sw});
    });
    x += w + (d < columns.length - 1 ? gapAfter(d) : 0);
  });
  const contentBottom = top + (rows - 1) * ROW_H + NODE_H;
  const width = x + (hasSide ? 52 : 16);
  const height = contentBottom + (hasBelow ? 40 : 10);

  const paths: ReactNode[] = [];
  const labels: ReactNode[] = [];
  const anchors = new Map<number, 'start' | 'middle' | 'end'>();
  edges.forEach((e, i) => {
    const a = pos.get(e.from);
    const b = pos.get(e.to);
    if (!a || !b) return;
    const da = depth.get(e.from) ?? 0;
    const db = depth.get(e.to) ?? 0;
    let d: string;
    let lx: number;
    let ly: number;
    if (e.from === e.to) {
      const x1 = a.x + a.w * 0.3;
      const x2 = a.x + a.w * 0.7;
      d = `M${x1} ${a.y} C${x1 - 6} ${a.y - 30} ${x2 + 6} ${a.y - 30} ${x2} ${a.y}`;
      lx = a.x + a.w / 2;
      ly = a.y - 26;
    } else if (db - da === 1) {
      const sx = a.x + a.w;
      const sy = a.y + NODE_H / 2;
      const tx = b.x;
      const ty = b.y + NODE_H / 2;
      const m = (tx - sx) / 2;
      d = `M${sx} ${sy} C${sx + m} ${sy} ${tx - m} ${ty} ${tx} ${ty}`;
      // The label sits right-aligned over the arrow's arrival, where arrows that fan out of one
      // state are a row apart; the column gap is wide enough for the longest label.
      lx = tx - 10;
      ly = ty - 6;
      anchors.set(i, 'end');
    } else if (db > da || (db < da && backAbove(e))) {
      const sx = a.x + a.w / 2;
      const tx = b.x + b.w / 2;
      const c = Math.min(a.y, b.y) - 30;
      d = `M${sx} ${a.y} C${sx} ${c} ${tx} ${c} ${tx} ${b.y}`;
      lx = (sx + tx) / 2;
      ly = 0.25 * Math.min(a.y, b.y) + 0.75 * c - 4;
    } else if (db < da) {
      const sx = a.x + a.w / 2;
      const tx = b.x + b.w / 2;
      const c = Math.max(a.y, b.y) + NODE_H + 30;
      d = `M${sx} ${a.y + NODE_H} C${sx} ${c} ${tx} ${c} ${tx} ${b.y + NODE_H}`;
      lx = (sx + tx) / 2;
      ly = 0.25 * (Math.max(a.y, b.y) + NODE_H) + 0.75 * c + 12;
    } else {
      const sx = a.x + a.w;
      const c = Math.max(a.x + a.w, b.x + b.w) + 34;
      d = `M${sx} ${a.y + NODE_H / 2} C${c} ${a.y + NODE_H / 2} ${c} ${b.y + NODE_H / 2} ${b.x + b.w} ${b.y + NODE_H / 2}`;
      lx = c - 4;
      ly = (a.y + b.y) / 2 + NODE_H / 2 + 4;
    }
    paths.push(<path key={`p${i}`} className="sa-life__edge" d={d} markerEnd={`url(#${uid}-a)`} />);
    labels.push(
      <text key={`l${i}`} className="sa-life__label" x={lx} y={ly} textAnchor={anchors.get(i) ?? 'middle'}>
        {e.label}
      </text>,
    );
  });

  const init = pos.get(initial);
  const description = `Lifecycle of ${entity}: starts ${initial}; ${edges
    .map((e) => `${e.label} moves ${e.from} to ${e.to}`)
    .join('; ')}${terminal.length ? `; ends in ${terminal.join(', ')}` : ''}.`;
  return (
    <figure className="sa-life">
      <div className="sa-life__frame" ref={frame} {...region}>
        <p className="sa-life__title">Lifecycle of {entity}</p>
        <svg width={width} height={height} viewBox={`0 0 ${width} ${height}`} role="img" aria-label={description}>
          <defs>
            <marker id={`${uid}-a`} viewBox="0 0 10 10" refX="9" refY="5" markerWidth="7" markerHeight="7" orient="auto-start-reverse">
              <path className="sa-life__arrow" d="M0 0L10 5L0 10z" />
            </marker>
          </defs>
          {init && (
            <g>
              <circle className="sa-life__start" cx={8} cy={init.y + NODE_H / 2} r={4} />
              <path
                className="sa-life__edge"
                d={`M12 ${init.y + NODE_H / 2} L${init.x} ${init.y + NODE_H / 2}`}
                markerEnd={`url(#${uid}-a)`}
              />
            </g>
          )}
          {paths}
          {states.map((s) => {
            const p = pos.get(s);
            if (!p) return null;
            const term = terminal.includes(s);
            return (
              <g
                key={s}
                className={`sa-life__node${term ? ' sa-life__node--terminal' : ''}${s === initial ? ' sa-life__node--initial' : ''}`}>
                {term && <rect className="sa-life__outer" x={p.x - 3} y={p.y - 3} width={p.w + 6} height={NODE_H + 6} rx={17} />}
                <rect x={p.x} y={p.y} width={p.w} height={NODE_H} rx={14} />
                <text x={p.x + p.w / 2} y={p.y + NODE_H / 2 + 4.5} textAnchor="middle">
                  {s}
                </text>
              </g>
            );
          })}
          {labels}
        </svg>
      </div>
    </figure>
  );
}
