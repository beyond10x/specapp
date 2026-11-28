import {useEffect, useState, type RefObject} from 'react';

/**
 * The attributes of a box that may scroll sideways: while its content is wider than the box it is
 * a focusable region named `label`, so a keyboard user can reach it and scroll it with the arrow
 * keys (WCAG 2.1.1; axe-core scrollable-region-focusable); while it fits it adds no tab stop.
 */
export function useScrollRegion(
  ref: RefObject<HTMLElement | null>,
  label: string,
): {tabIndex?: number; role?: 'region'; 'aria-label'?: string} {
  const [scrolls, setScrolls] = useState(false);
  useEffect(() => {
    const el = ref.current;
    if (!el) return undefined;
    const measure = () => setScrolls(el.scrollWidth > el.clientWidth + 1);
    measure();
    if (typeof ResizeObserver === 'undefined') return undefined;
    const observer = new ResizeObserver(measure);
    observer.observe(el);
    if (el.firstElementChild) observer.observe(el.firstElementChild);
    return () => observer.disconnect();
  }, [ref]);
  return scrolls ? {tabIndex: 0, role: 'region', 'aria-label': label} : {};
}
