import { z } from 'zod';

import { onProviderEmit } from '~/desktop';
import { createBaseProvider } from '../create-base-provider';
import type {
  BluetoothOutput,
  BluetoothProvider,
  BluetoothProviderConfig,
} from './bluetooth-provider-types';

const bluetoothProviderConfigSchema = z.object({
  type: z.literal('bluetooth'),
  refreshInterval: z.coerce.number().default(5 * 1000),
});

export function createBluetoothProvider(
  config: BluetoothProviderConfig,
): BluetoothProvider {
  const mergedConfig = bluetoothProviderConfigSchema.parse(config);

  return createBaseProvider(mergedConfig, async queue => {
    return onProviderEmit<BluetoothOutput>(mergedConfig, ({ result }) => {
      if ('error' in result) {
        queue.error(result.error);
      } else {
        queue.output(result.output);
      }
    });
  });
}
