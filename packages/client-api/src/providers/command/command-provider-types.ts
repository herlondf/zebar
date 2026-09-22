import type { Provider } from '../create-base-provider';

export interface CommandProviderConfig {
  type: 'command';

  /**
   * Program name (if in PATH) or full path to the program.
   *
   * The widget needs a matching entry in its `privileges.shellCommands`,
   * the same as for {@link shellExec}.
   */
  program: string;

  /**
   * Arguments to pass to the program.
   */
  args?: string[];

  /**
   * Directory to run the program in.
   */
  cwd?: string;

  /**
   * How often this provider refreshes in milliseconds.
   */
  refreshInterval?: number;
}

export type CommandProvider = Provider<
  CommandProviderConfig,
  CommandOutput
>;

export interface CommandOutput {
  /**
   * What the program wrote to stdout, with trailing newlines removed.
   */
  stdout: string;

  /**
   * What the program wrote to stderr, with trailing newlines removed.
   */
  stderr: string;

  /**
   * Exit code, or `null` if the program was killed by a signal.
   */
  exitCode: number | null;

  /**
   * Whether the program exited successfully.
   */
  success: boolean;
}
