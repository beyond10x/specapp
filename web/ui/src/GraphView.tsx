import {useId, useRef, type ReactNode} from 'react';
import {Icon} from './Icon.js';
import {useScrollRegion} from './scroll.js';
import type {Presentation, Scene} from './types.js';

/** The geometry of a box's text, the same as specapp's layout measures it. */
const PAD = 10;
const LABEL_H = 18;
const LINE_H = 15;
const ICON = 16;

function px(n: number): number {
  return Math.round(n * 10) / 10;
}

/** The path of a `rows` edge: vertical runs straight, a step between two x as a curve. */
function curve(points: [number, number][]): string {
  let d = '';
  points.forEach((p, i) => {
    if (i === 0) {
      d += `M${px(p[0])} ${px(p[1])}`;
      return;
    }
    const a = points[i - 1];
    if (Math.abs(a[0] - p[0]) < 0.05) {
      d += `L${px(p[0])} ${px(p[1])}`;
    } else {
      const my = (a[1] + p[1]) / 2;
      d += `C${px(a[0])} ${px(my)} ${px(p[0])} ${px(my)} ${px(p[0])} ${px(p[1])}`;
    }
  });
  return d;
}

function polyline(points: [number, number][]): string {
  return points.map((p, i) => `${i === 0 ? 'M' : 'L'}${px(p[0])} ${px(p[1])}`).join('');
}

/**
 * A laid-out view of a document (specapp-scene/1) drawn as SVG in the page, at its own size: the
 * drawing scrolls sideways in its box rather than shrinking its text. Each box and edge names its
 * identifiers on hover; a finding is a numbered mark (filled: gated; outlined: not gated) that
 * links to its row in the list of findings; the legend says what each colour and icon stands for.
 */
export function GraphView({data, presentation}: {data: Scene; presentation?: Presentation}): ReactNode {
  const scene = data;
  const frame = useRef<HTMLDivElement>(null);
  const region = useScrollRegion(frame, `${scene.view.title}, drawing`);
  const p = `sa-g${useId().replace(/[^a-zA-Z0-9_-]/g, '')}`;
  const curved = scene.view.layout === 'rows';
  const markWidth = (subject: string, n: number) =>
    scene.marks.find((m) => m.finding === n && m.subject === subject)?.w ?? 22;
  return (
    <figure className="sa-graph" id={`view-${scene.view.id}`}>
      <div className="sa-graph__frame" ref={frame} {...region}>
        <svg
          className="sa-scene"
          viewBox={`0 0 ${scene.width} ${scene.height}`}
          width={scene.width}
          height={scene.height}
          role={scene.marks.length > 0 ? 'group' : 'img'}
          aria-labelledby={`${p}-t ${p}-d`}>
          <title id={`${p}-t`}>{`${scene.view.title} of ${scene.document}`}</title>
          <desc id={`${p}-d`}>
            {`${scene.boxes.length} boxes and ${scene.edges.length} edges, each edge labelled by what it carries; ${scene.marks.length} findings marked with their number. The tables below the drawing list the same parts, links and findings as text.`}
          </desc>
          <defs>
            <marker id={`${p}-arrow`} viewBox="0 0 10 10" refX="9" refY="5" markerWidth="7" markerHeight="7" orient="auto-start-reverse">
              <path d="M0 0L10 5L0 10z" className="sa-scene__arrowhead" />
            </marker>
          </defs>
          {scene.rows.length > 0 && (
            <g>
              {scene.rows.map((band) => (
                <g key={`${band.title}-${band.y}`}>
                  <rect className="sa-scene__band" x={4} y={band.y} width={px(scene.width - 8)} height={band.h} rx={6} />
                  <text className="sa-scene__band-title" x={12} y={px(band.y + 16)}>
                    {band.title}
                  </text>
                </g>
              ))}
            </g>
          )}
          <g>
            {scene.boxes.map((b, i) => {
              const child = b.parent !== undefined && b.lines.length === 0;
              const classes = ['sa-scene__box', `sa-c-${b.colour}`];
              if (b.ghost) classes.push('sa-scene__box--ghost');
              if (b.container) classes.push('sa-scene__box--container');
              if (child) classes.push('sa-scene__box--child');
              if (b.findings.length > 0) classes.push('sa-scene__box--marked');
              let lx = b.x + PAD;
              return (
                <g key={`${b.id}-${i}`} className={classes.join(' ')} id={`${p}-b${i}`} data-id={b.id}>
                  <title>{b.title}</title>
                  {b.on_border && (
                    <rect className="sa-scene__border" x={px(b.x - 3)} y={px(b.y - 3)} width={px(b.w + 6)} height={px(b.h + 6)} rx={9} />
                  )}
                  <rect className="sa-scene__body" x={b.x} y={b.y} width={b.w} height={b.h} rx={6} />
                  {child ? (
                    <text className="sa-scene__label" x={px(b.x + 8)} y={px(b.y + 16)}>
                      {b.label}
                    </text>
                  ) : (
                    <>
                      {b.icon &&
                        (() => {
                          const x = b.x + PAD;
                          const y = b.y + PAD;
                          lx += ICON + 6;
                          return (
                            <svg x={px(x)} y={px(y)} width={ICON} height={ICON} viewBox="0 0 14 14" className="sa-scene__icon">
                              <Icon name={b.icon} presentation={presentation} className="sa-scene__icon-inner" title={b.icon} />
                            </svg>
                          );
                        })()}
                      <text className="sa-scene__label" x={px(lx)} y={px(b.y + PAD + 13)}>
                        {b.label}
                      </text>
                      {b.lines.map((line, k) => (
                        <text key={k} className="sa-scene__line" x={px(b.x + PAD)} y={px(b.y + PAD + LABEL_H + 11 + k * LINE_H)}>
                          {line}
                        </text>
                      ))}
                    </>
                  )}
                </g>
              );
            })}
          </g>
          <g>
            {scene.edges.map((e, i) => {
              const classes = ['sa-scene__edge'];
              if (e.added_by) classes.push('sa-scene__edge--added');
              if (e.findings.length > 0) classes.push('sa-scene__edge--marked');
              const marks = e.findings.reduce((sum, n) => sum + markWidth(`link:${e.id}`, n) + 3, 0);
              const r = e.label_at;
              return (
                <g key={`${e.id}-${i}`} className={classes.join(' ')} id={`${p}-e${i}`} data-id={e.id}>
                  <title>{e.title}</title>
                  <path d={curved ? curve(e.points) : polyline(e.points)} markerEnd={`url(#${p}-arrow)`} />
                  <rect className="sa-scene__pill" x={r.x} y={r.y} width={r.w} height={r.h} rx={9} />
                  <text className="sa-scene__edge-label" x={px(r.x + marks + (r.w - marks) / 2)} y={px(r.y + 13)} textAnchor="middle">
                    {e.label}
                  </text>
                </g>
              );
            })}
          </g>
          <g>
            {scene.marks.map((m) => (
              <a key={`${m.finding}-${m.subject}`} href={`#finding-${m.finding}`}>
                <g className={m.gate ? 'sa-scene__mark' : 'sa-scene__mark sa-scene__mark--note'}>
                  <title>{`finding ${m.finding}: ${m.rule} on ${m.subject}${m.gate ? '' : ' (not gated)'}`}</title>
                  <rect x={px(m.x - m.w / 2)} y={px(m.y - 8)} width={m.w} height={16} rx={8} />
                  <text x={m.x} y={px(m.y + 3.5)} textAnchor="middle">
                    {`!${m.finding}`}
                  </text>
                </g>
              </a>
            ))}
          </g>
        </svg>
      </div>
      {scene.legend.length > 0 && (
        <ul className="sa-legend" aria-label={`Legend of ${scene.view.title}`}>
          {scene.legend.map((entry) => (
            <li key={entry.label} className={`sa-c-${entry.colour}`}>
              <Icon name={entry.icon} presentation={presentation} className="sa-legend__icon" />
              <strong>{entry.label}</strong>
              {entry.blocks.length > 0 ? `: ${entry.blocks.join(', ')}` : ''}
            </li>
          ))}
        </ul>
      )}
    </figure>
  );
}
