import {useId, useMemo, useRef, useState, type ReactNode} from 'react';
import {SpecLink} from './config.js';
import {Icon, styleOf} from './Icon.js';
import {useScrollRegion} from './scroll.js';
import type {LayerMapData, Presentation} from './types.js';

function plural(n: number, one: string, many: string): string {
  return `${n} ${n === 1 ? one : many}`;
}

/**
 * The layer map: every block as a card in the row of its layer, linked to its page, with arrows to
 * the blocks it depends on. Pointing at a block (or focusing it) lights what it uses and what uses
 * it. Colours and icons come from the presentation table; the legend names what each stands for.
 */
export function LayerMap({data, presentation}: {data: LayerMapData; presentation?: Presentation}): ReactNode {
  const [focus, setFocus] = useState<string | null>(null);
  const near = useMemo(() => {
    if (!focus) return null;
    const b = data.blocks.find((x) => x.name === focus);
    return new Set([focus, ...(b?.uses ?? []), ...(b?.used_by ?? [])]);
  }, [focus, data]);
  const groups = (presentation?.groups ?? []).filter((g) =>
    data.blocks.some((b) => styleOf(presentation, b.name).group === g.label),
  );
  const pad = 14;
  const frame = useRef<HTMLDivElement>(null);
  const region = useScrollRegion(frame, 'Layer map');
  const uid = useId().replace(/[^a-zA-Z0-9_-]/g, '');
  return (
    <figure className="sa-map" id="layer-map">
      <div className="sa-map__frame" ref={frame} {...region}>
        <svg
          viewBox={`0 0 ${data.width} ${data.height}`}
          role="group"
          aria-label="Building blocks by layer, each linked to its page, with arrows to the blocks it depends on">
          <defs>
            <marker id={`${uid}-a`} viewBox="0 0 10 10" refX="9" refY="5" markerWidth="7" markerHeight="7" orient="auto-start-reverse">
              <path className="sa-map__arrow" d="M0 0L10 5L0 10z" />
            </marker>
            <marker id={`${uid}-on`} viewBox="0 0 10 10" refX="9" refY="5" markerWidth="7" markerHeight="7" orient="auto-start-reverse">
              <path className="sa-map__arrow--on" d="M0 0L10 5L0 10z" />
            </marker>
          </defs>
          {data.rows.map((r, i) => (
            <g key={`${r.layer ?? 'none'}-${r.name}`}>
              {i % 2 === 0 && <rect className="sa-map__band" x={0} y={r.y - pad} width={data.width} height={r.h + 2 * pad} rx={10} />}
              <text className="sa-map__layer-n" x={10} y={r.y + 30}>
                {r.layer ?? '–'}
              </text>
              <text className="sa-map__layer-name" x={10} y={r.y + 50}>
                {r.name}
              </text>
            </g>
          ))}
          {data.edges.map((e) => {
            const on = focus !== null && (e.from === focus || e.to === focus);
            const off = focus !== null && !on;
            return (
              <path
                key={`${e.from}-${e.to}`}
                className={`sa-map__edge${on ? ' sa-map__edge--on' : ''}${off ? ' sa-map__edge--off' : ''}`}
                d={e.d}
                markerEnd={on ? `url(#${uid}-on)` : `url(#${uid}-a)`}>
                <title>{`${e.from} uses ${e.to} (${plural(e.count, 'reference', 'references')})`}</title>
              </path>
            );
          })}
          {data.blocks.map((b) => {
            const style = styleOf(presentation, b.name);
            const state = near ? (near.has(b.name) ? (b.name === focus ? ' sa-card--on' : '') : ' sa-card--off') : '';
            const body = (
              <>
                <span className="sa-card__head">
                  <Icon name={style.icon} presentation={presentation} className="sa-card__icon" />
                  <span className="sa-card__name">{b.label}</span>
                </span>
                <span className="sa-card__summary">{b.band ? `${b.summary} Every block above uses it.` : b.summary}</span>
                {!b.band && (
                  <span className="sa-card__meta">
                    <span>
                      <b>{b.entities}</b> ent
                    </span>
                    <span>
                      <b>{b.commands}</b> cmd
                    </span>
                    <span>
                      <b>{b.events}</b> evt
                    </span>
                    <span className="sa-card__kpi">
                      <b>{b.kpis}</b> {b.kpis === 1 ? 'KPI' : 'KPIs'}
                    </span>
                  </span>
                )}
              </>
            );
            const className = `sa-card sa-c-${style.colour}${b.band ? ' sa-card--band' : ''}${state}`;
            const handlers = {
              onMouseEnter: () => setFocus(b.name),
              onMouseLeave: () => setFocus(null),
              onFocus: () => setFocus(b.name),
              onBlur: () => setFocus(null),
            };
            return (
              <foreignObject key={b.name} x={b.x} y={b.y} width={b.w} height={b.h}>
                {b.route ? (
                  <SpecLink to={b.route} className={className} title={b.summary} {...handlers}>
                    {body}
                  </SpecLink>
                ) : (
                  <span className={className} title={b.summary}>
                    {body}
                  </span>
                )}
              </foreignObject>
            );
          })}
        </svg>
      </div>
      {groups.length > 0 && (
        <ul className="sa-legend" aria-label="Colours">
          {groups.map((g) => (
            <li key={g.label} className={`sa-c-${g.colour}`}>
              <Icon name={g.icon} presentation={presentation} className="sa-legend__icon" />
              {g.label}
            </li>
          ))}
        </ul>
      )}
      <p className="sa-map__hint">
        Point at a block to see what it uses and what uses it. ent, cmd, evt: entities, commands and events of the block.
      </p>
    </figure>
  );
}
