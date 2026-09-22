import type { Provider } from '../create-base-provider';

export interface BluetoothProviderConfig {
  type: 'bluetooth';

  /**
   * How often this provider refreshes in milliseconds.
   */
  refreshInterval?: number;
}

export type BluetoothProvider = Provider<
  BluetoothProviderConfig,
  BluetoothOutput
>;

export interface BluetoothOutput {
  /**
   * Every paired Bluetooth device.
   */
  devices: BluetoothDevice[];

  /**
   * The subset that is connected right now.
   */
  connectedDevices: BluetoothDevice[];
}

export interface BluetoothDevice {
  /**
   * Device name as Windows shows it.
   */
  name: string;

  /**
   * Whether the device is connected right now.
   */
  isConnected: boolean;

  /**
   * Battery percentage, or `null` for the many devices that do not
   * report one.
   */
  batteryPercent: number | null;
}
