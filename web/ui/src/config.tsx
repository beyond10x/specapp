import type {ComponentType, FocusEventHandler, MouseEventHandler, ReactNode} from 'react';

/** What a link of a component passes on: the route of a page, as the generator wrote it. */
export interface LinkProps {
  to: string;
  className?: string;
  title?: string;
  children?: ReactNode;
  onMouseEnter?: MouseEventHandler;
  onMouseLeave?: MouseEventHandler;
  onFocus?: FocusEventHandler;
  onBlur?: FocusEventHandler;
}

function Anchor({to, ...rest}: LinkProps): ReactNode {
  return <a href={to} {...rest} />;
}

let linkComponent: ComponentType<LinkProps> = Anchor;
let routeBase = '';

/**
 * Tells the components how a shell links: the link component (a router's, or a plain anchor)
 * and the path the routes of the generated pages start at. The docs preset calls it once in a
 * client module; tests and the studio may call it with their own.
 */
export function configure(options: {Link?: ComponentType<LinkProps>; base?: string}): void {
  if (options.Link) linkComponent = options.Link;
  if (options.base !== undefined) routeBase = options.base.replace(/\/+$/, '');
}

/** The address of a route of a generated page. */
export function route(to: string): string {
  if (/^[a-z]+:/.test(to) || to.startsWith('#')) return to;
  return `${routeBase}${to.startsWith('/') ? to : `/${to}`}`;
}

/** A link to a generated page with the shell's link component. */
export function SpecLink(props: LinkProps): ReactNode {
  const Component = linkComponent;
  return <Component {...props} to={route(props.to)} />;
}
