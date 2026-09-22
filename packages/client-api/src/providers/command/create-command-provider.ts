import { z } from 'zod';

import { onProviderEmit } from '~/desktop';
import { createBaseProvider } from '../create-base-provider';
import type {
  CommandOutput,
  CommandProvider,
  CommandProviderConfig,
} from './command-provider-types';

const commandProviderConfigSchema = z.object({
  type: z.literal('command'),
  program: z.string(),
  args: z.array(z.string()).default([]),
  cwd: z.string().optional(),
  refreshInterval: z.coerce.number().default(5 * 1000),
});

export function createCommandProvider(
  config: CommandProviderConfig,
): CommandProvider {
  const mergedConfig = commandProviderConfigSchema.parse(config);

  return createBaseProvider(mergedConfig, async queue => {
    return onProviderEmit<CommandOutput>(mergedConfig, ({ result }) => {
      if ('error' in result) {
        queue.error(result.error);
      } else {
        queue.output(result.output);
      }
    });
  });
}
