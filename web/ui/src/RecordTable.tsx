import {useRef, type ReactNode} from 'react';
import {useScrollRegion} from './scroll.js';
import type {Presentation, RecordTableData} from './types.js';

/**
 * One table of a record document as the manifest names it: the key column first when the table
 * has one, then the columns in the manifest's order. Each row has an anchor by its key.
 */
export function RecordTable({data}: {data: RecordTableData; presentation?: Presentation}): ReactNode {
  const scroller = useRef<HTMLDivElement>(null);
  const region = useScrollRegion(scroller, data.title);
  const anchor = (row: string[]) =>
    data.key && row[0] ? `${data.at}-${row[0]}`.toLowerCase().replace(/[^a-z0-9_-]+/g, '-') : undefined;
  return (
    <div className="sa-table" ref={scroller} {...region}>
      <table className="sa-record">
        <caption>{`${data.title}: ${data.rows.length} ${data.rows.length === 1 ? 'row' : 'rows'}`}</caption>
        <thead>
          <tr>
            {data.columns.map((c) => (
              <th key={c} scope="col">
                {c}
              </th>
            ))}
          </tr>
        </thead>
        <tbody>
          {data.rows.map((row, i) => (
            <tr key={i} id={anchor(row)}>
              {row.map((value, k) =>
                k === 0 && data.key ? (
                  <th key={k} scope="row">
                    <code>{value}</code>
                  </th>
                ) : (
                  <td key={k}>{value || '–'}</td>
                ),
              )}
            </tr>
          ))}
        </tbody>
      </table>
    </div>
  );
}
