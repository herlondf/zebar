import type { Provider } from '../create-base-provider';

export interface FocusedWindowProviderConfig {
  type: 'focusedWindow';

  /**
   * How often this provider refreshes in milliseconds.
   */
  refreshInterval?: number;
}

export type FocusedWindowProvider = Provider<
  FocusedWindowProviderConfig,
  FocusedWindowOutput
>;

export interface FocusedWindowOutput {
  /**
   * Details of the focused window, or `null` when nothing has focus.
   *
   * Alt-tabbing and clicking the desktop both leave the foreground window
   * briefly unset, so expect this to be empty at times.
   */
  window: FocusedWindow | null;
}

export interface FocusedWindow {
  /**
   * Window title.
   */
  title: string;

  /**
   * Executable name, e.g. `firefox.exe`.
   */
  processName: string;

  /**
   * Full path to the executable.
   *
   * Empty for a process Zebar is not allowed to open, which is the case
   * for anything running elevated.
   */
  processPath: string;

  /**
   * ID of the process that owns the window.
   */
  processId: number;
}
