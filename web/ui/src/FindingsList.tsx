import {useRef, type ReactNode} from 'react';
import {useScrollRegion} from './scroll.js';
import type {FindingsData, Presentation} from './types.js';

/**
 * The findings of the pack's tool on one document, one row each: its number (the mark in the
 * drawings), the rule, whether it is gated, what it is about, the tool's sentence, and the views
 * it is drawn in. A finding about the whole document, or about a part or link the document does
 * not have, is a row only.
 */
export function FindingsList({data}: {data: FindingsData; presentation?: Presentation}): ReactNode {
  const scroller = useRef<HTMLDivElement>(null);
  const region = useScrollRegion(scroller, 'Findings');
  if (data.findings.length === 0) {
    return <p className="sa-findings__none">None.</p>;
  }
  const title = (id: string) => data.views.find((v) => v.id === id)?.title ?? id;
  return (
    <div className="sa-table" ref={scroller} {...region}>
      <table className="sa-findings">
        <thead>
          <tr>
            <th scope="col">#</th>
            <th scope="col">Rule</th>
            <th scope="col">Gate</th>
            <th scope="col">Subject</th>
            <th scope="col">Message</th>
            <th scope="col">Drawn in</th>
          </tr>
        </thead>
        <tbody>
          {data.findings.map((f) => (
            <tr key={f.n} id={`finding-${f.n}`}>
              <td>
                <span className={f.gate ? 'sa-badge' : 'sa-badge sa-badge--note'}>!{f.n}</span>
              </td>
              <td>
                <code>{f.rule}</code>
              </td>
              <td>{f.gate ? 'gate' : 'not gated'}</td>
              <td>
                <code>{f.subject}</code>
              </td>
              <td>{f.message}</td>
              <td>
                {f.drawn_in.length === 0
                  ? f.subject === 'document'
                    ? 'the whole document'
                    : 'not drawn: no part or link of this id'
                  : f.drawn_in.map((v, i) => (
                      <span key={v}>
                        {i > 0 && ', '}
                        <a href={`#view-${v}`}>{title(v)}</a>
                      </span>
                    ))}
              </td>
            </tr>
          ))}
        </tbody>
      </table>
    </div>
  );
}
