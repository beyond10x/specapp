import type {ReactNode} from 'react';
import type {Presentation, ResultData} from './types.js';

function number(v: number | null | undefined): string {
  if (v === null || v === undefined || Number.isNaN(v)) return '–';
  if (Number.isInteger(v)) return String(v);
  return Math.abs(v) >= 100 ? v.toFixed(1) : v.toFixed(3);
}

/**
 * The result of a run of the pack's tool (metaspec-result/1): the fitness and every KPI as built
 * and for the best variant. The docs build runs no simulation; a page shows a result committed
 * beside its document, or says that there is none.
 */
export function ResultSections({data}: {data: ResultData | null; presentation?: Presentation}): ReactNode {
  const base = data?.baseline?.kpis;
  if (!data || !base) {
    return <p className="sa-result__none">No run result is committed beside this document.</p>;
  }
  const best = data.best?.kpis;
  return (
    <section className="sa-result" aria-label="Run result">
      <p>
        Fitness as built {number(data.baseline?.fitness)}
        {data.best ? `, best variant ${number(data.best.fitness)}` : ', no variant passes the gates'}
        {data.variants ? ` (${data.variants.length} variants tried)` : ''}.
      </p>
      <div className="sa-table">
        <table>
          <thead>
            <tr>
              <th scope="col">KPI</th>
              <th scope="col">kind</th>
              <th scope="col">as built</th>
              <th scope="col">best variant</th>
            </tr>
          </thead>
          <tbody>
            {Object.entries(base).map(([id, k]) => (
              <tr key={id}>
                <td>
                  <code>{id}</code>
                </td>
                <td>{k.kind ?? '–'}</td>
                <td>{number(k.value)}</td>
                <td>{best ? number(best[id]?.value) : '–'}</td>
              </tr>
            ))}
          </tbody>
        </table>
      </div>
    </section>
  );
}
