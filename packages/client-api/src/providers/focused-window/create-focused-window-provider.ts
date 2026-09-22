import { z } from 'zod';

import { onProviderEmit } from '~/desktop';
import { createBaseProvider } from '../create-base-provider';
import type {
  FocusedWindowOutput,
  FocusedWindowProvider,
  FocusedWindowProviderConfig,
} from './focused-window-provider-types';

const focusedWindowProviderConfigSchema = z.object({
  type: z.literal('focusedWindow'),
  refreshInterval: z.coerce.number().default(1000),
});

export function createFocusedWindowProvider(
  config: FocusedWindowProviderConfig,
): FocusedWindowProvider {
  const mergedConfig = focusedWindowProviderConfigSchema.parse(config);

  return createBaseProvider(mergedConfig, async queue => {
    return onProviderEmit<FocusedWindowOutput>(
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
