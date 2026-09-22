import type { Provider } from '../create-base-provider';

export interface GpuProviderConfig {
  type: 'gpu';

  /**
   * How often this provider refreshes in milliseconds.
   */
  refreshInterval?: number;
}

export type GpuProvider = Provider<GpuProviderConfig, GpuOutput>;

export interface GpuOutput {
  /**
   * Every NVIDIA GPU the driver reports.
   *
   * Reading these goes through NVIDIA's NVML library, so AMD and Intel
   * GPUs are not covered and the provider errors on a machine with no
   * NVIDIA driver.
   */
  devices: GpuDevice[];
}

export interface GpuDevice {
  /**
   * Model name, e.g. `NVIDIA GeForce RTX 3050 Laptop GPU`.
   */
  name: string;

  /**
   * Percentage of time the GPU was busy over the last sample period.
   */
  usage: number;

  /**
   * Percentage of time GPU memory was being read or written.
   */
  memoryUsage: number;

  /**
   * Memory in use, in bytes.
   */
  usedMemory: number;

  /**
   * Total memory, in bytes.
   */
  totalMemory: number;

  /**
   * Core temperature in celsius.
   */
  celsiusTemp: number;

  /**
   * Fan speed as a percentage, or `null` on cards with no controllable
   * fan, which includes many laptop GPUs.
   */
  fanSpeed: number | null;

  /**
   * Current core clock in MHz.
   */
  coreClockMhz: number | null;

  /**
   * Current power draw in milliwatts.
   */
  powerUsageMw: number | null;
}
