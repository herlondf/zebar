interface Window {
  // TODO: Add typing.
  __ZEBAR_STATE: any;

  /**
   * Ports resolved at startup, injected into every widget by the desktop
   * app before any widget code runs.
   */
  __ZEBAR_PORTS?: {
    /** Port the local asset server is listening on. */
    assetServer: number;

    /** Port of the GlazeWM instance in this session. */
    glazewmIpc: number;
  };
}
