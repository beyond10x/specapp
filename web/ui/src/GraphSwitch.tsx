import {useState, type ReactNode} from 'react';
import {GraphView} from './GraphView.js';
import type {GraphSwitchData, Presentation} from './types.js';

/**
 * A view with hubs and its view `<id>/edges` as one view with a switch: the first draws each link
 * to a hub as a badge on its caller, the second every link as an edge. Both views keep an anchor,
 * so a finding row that names either view links here.
 */
export function GraphSwitch({data, presentation}: {data: GraphSwitchData; presentation?: Presentation}): ReactNode {
  const [shown, setShown] = useState(0);
  const views = data.views;
  const choices = ['Hubs as badges', 'Every link an edge'];
  return (
    <div className="sa-switch">
      <span id={`view-${views[1 - shown].view.id}`} />
      <div className="sa-switch__choices" role="group" aria-label={`How ${views[0].view.title} draws the links to its hubs`}>
        {choices.map((label, i) => (
          <button
            key={label}
            type="button"
            className={i === shown ? 'sa-switch__choice sa-switch__choice--on' : 'sa-switch__choice'}
            aria-pressed={i === shown}
            onClick={() => setShown(i)}>
            {label}
          </button>
        ))}
      </div>
      <GraphView key={views[shown].view.id} data={views[shown]} presentation={presentation} />
    </div>
  );
}
