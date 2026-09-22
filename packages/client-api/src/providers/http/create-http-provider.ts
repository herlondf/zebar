import { z } from 'zod';

import { onProviderEmit } from '~/desktop';
import { createBaseProvider } from '../create-base-provider';
import type {
  HttpOutput,
  HttpProvider,
  HttpProviderConfig,
} from './http-provider-types';

const httpProviderConfigSchema = z.object({
  type: z.literal('http'),
  url: z.string(),
  method: z.string().optional(),
  headers: z.record(z.string()).default({}),
  refreshInterval: z.coerce.number().default(60 * 1000),
});

export function createHttpProvider(
  config: HttpProviderConfig,
): HttpProvider {
  const mergedConfig = httpProviderConfigSchema.parse(config);

  return createBaseProvider(mergedConfig, async queue => {
    return onProviderEmit<HttpOutput>(mergedConfig, ({ result }) => {
      if ('error' in result) {
        queue.error(result.error);
      } else {
        queue.output(result.output);
      }
    });
  });
}
