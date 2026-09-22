import type { Provider } from '../create-base-provider';

export interface TemperatureProviderConfig {
  type: 'temperature';

  /**
   * How often this provider refreshes in milliseconds.
   */
  refreshInterval?: number;
}

export type TemperatureProvider = Provider<
  TemperatureProviderConfig,
  TemperatureOutput
>;

export interface TemperatureOutput {
  /**
   * Every sensor the OS reports.
   */
  components: TemperatureComponent[];

  /**
   * Hottest sensor, or `null` when the OS reports no sensors at all.
   *
   * Which sensor that is varies by machine, so a bar that just wants one
   * number has something to show without knowing the labels.
   */
  hottest: TemperatureComponent | null;
}

export interface TemperatureComponent {
  /**
   * Sensor name as the OS reports it, e.g. `CPU`.
   */
  label: string;

  /**
   * Current temperature in celsius.
   */
  celsiusTemp: number;

  /**
   * Current temperature in fahrenheit.
   */
  fahrenheitTemp: number;

  /**
   * Highest temperature seen since the process started.
   */
  maxCelsiusTemp: number;

  /**
   * Temperature the OS considers critical, if it reports one.
   */
  criticalCelsiusTemp: number | null;
}
