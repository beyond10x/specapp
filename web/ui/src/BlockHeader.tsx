import type {ReactNode} from 'react';
import {Icon, styleOf} from './Icon.js';
import type {BlockHeaderData, Counts, Presentation} from './types.js';

const LABELS: [keyof Counts, string, string][] = [
  ['entities', 'entity', 'entities'],
  ['commands', 'command', 'commands'],
  ['events', 'event', 'events'],
  ['views', 'view', 'views'],
  ['refusals', 'refusal', 'refusals'],
  ['kpis', 'KPI', 'KPIs'],
];

/** A block's place and size at a glance: its colour and icon, its layer, and how much its contract holds. */
export function BlockHeader({data, presentation}: {data: BlockHeaderData; presentation?: Presentation}): ReactNode {
  const style = styleOf(presentation, data.name);
  return (
    <div className={`sa-head sa-c-${style.colour}`}>
      <div className="sa-head__id">
        <Icon name={style.icon} presentation={presentation} className="sa-head__icon" title={style.icon ? `${style.group} (${style.icon})` : undefined} />
        <div className="sa-head__layer">
          {data.planned ? (
            <>
              <b>planned</b>
              <span>not specified yet</span>
            </>
          ) : data.layer !== null ? (
            <>
              <b>layer {data.layer}</b>
              <span>{data.layer_name}</span>
            </>
          ) : (
            <>
              <b>no layer</b>
              <span>{style.group}</span>
            </>
          )}
        </div>
      </div>
      <div className="sa-head__stats">
        {LABELS.filter(([key]) => data.counts[key] !== undefined).map(([key, one, many]) => {
          const n = data.counts[key] ?? 0;
          return (
            <div key={key} className={`sa-stat${n === 0 ? ' sa-stat--zero' : ''}`}>
              <b>{n}</b>
              <span>{n === 1 ? one : many}</span>
            </div>
          );
        })}
      </div>
    </div>
  );
}
