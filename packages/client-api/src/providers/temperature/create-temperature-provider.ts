import { z } from 'zod';

import { onProviderEmit } from '~/desktop';
import { createBaseProvider } from '../create-base-provider';
import type {
  TemperatureOutput,
  TemperatureProvider,
  TemperatureProviderConfig,
} from './temperature-provider-types';

const temperatureProviderConfigSchema = z.object({
  type: z.literal('temperature'),
  refreshInterval: z.coerce.number().default(5 * 1000),
});

export function createTemperatureProvider(
  config: TemperatureProviderConfig,
): TemperatureProvider {
  const mergedConfig = temperatureProviderConfigSchema.parse(config);

  return createBaseProvider(mergedConfig, async queue => {
    return onProviderEmit<TemperatureOutput>(
      mergedConfig,
      ({ result }) => {
        if ('error' in result) {
          queue.error(result.error);
        } else {
          queue.output(result.output);
        }
      },
    );
  });
}
