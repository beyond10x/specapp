import type {ReactNode} from 'react';
import type {BlockStyle, Presentation} from './types.js';

/** The style of a block; a block the table does not have is drawn in slate with no icon. */
export function styleOf(presentation: Presentation | undefined, block: string): BlockStyle {
  return (
    presentation?.blocks[block] ?? {
      label: block,
      colour: 'slate',
      icon: null,
      group: 'block the pack does not have',
      layer: null,
      layer_name: null,
      route: null,
    }
  );
}

/** A block or link name in the pack's words. */
export function word(presentation: Presentation | undefined, name: string): string {
  return presentation?.vocabulary[name] ?? name;
}

/**
 * An icon of the presentation table in a 14 by 14 box: its stroke path, or the first letter of
 * its name for an icon specapp does not draw. No icon: a small circle.
 */
export function Icon({
  name,
  presentation,
  className,
  title,
}: {
  name: string | null | undefined;
  presentation: Presentation | undefined;
  className: string;
  title?: string;
}): ReactNode {
  const path = name ? presentation?.icons[name] : undefined;
  return (
    <svg className={className} viewBox="0 0 14 14" aria-hidden={title ? undefined : true} role={title ? 'img' : undefined}>
      {title && <title>{title}</title>}
      {path ? (
        <path d={path} />
      ) : name ? (
        <text x="7" y="10.5" textAnchor="middle" className="sa-monogram">
          {name.slice(0, 1).toUpperCase()}
        </text>
      ) : (
        <circle cx="7" cy="7" r="4.5" />
      )}
    </svg>
  );
}
