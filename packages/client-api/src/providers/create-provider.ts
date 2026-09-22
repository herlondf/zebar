import { createAudioProvider } from './audio/create-audio-provider';
import type {
  AudioProviderConfig,
  AudioProvider,
} from './audio/audio-provider-types';
import { createBatteryProvider } from './battery/create-battery-provider';
import type {
  BatteryProviderConfig,
  BatteryProvider,
} from './battery/battery-provider-types';
import { createCommandProvider } from './command/create-command-provider';
import type {
  CommandProviderConfig,
  CommandProvider,
} from './command/command-provider-types';
import { createCpuProvider } from './cpu/create-cpu-provider';
import type {
  CpuProviderConfig,
  CpuProvider,
} from './cpu/cpu-provider-types';
import { createDateProvider } from './date/create-date-provider';
import type {
  DateProviderConfig,
  DateProvider,
} from './date/date-provider-types';
import { createGlazeWmProvider } from './glazewm/create-glazewm-provider';
import type {
  GlazeWmProviderConfig,
  GlazeWmProvider,
} from './glazewm/glazewm-provider-types';
import { createFocusedWindowProvider } from './focused-window/create-focused-window-provider';
import type {
  FocusedWindowProviderConfig,
  FocusedWindowProvider,
} from './focused-window/focused-window-provider-types';
import { createGpuProvider } from './gpu/create-gpu-provider';
import type {
  GpuProviderConfig,
  GpuProvider,
} from './gpu/gpu-provider-types';
import { createHostProvider } from './host/create-host-provider';
import { createHttpProvider } from './http/create-http-provider';
import type {
  HttpProviderConfig,
  HttpProvider,
} from './http/http-provider-types';
import type {
  HostProviderConfig,
  HostProvider,
} from './host/host-provider-types';
import { createIpProvider } from './ip/create-ip-provider';
import type { IpProviderConfig, IpProvider } from './ip/ip-provider-types';
import { createKeyboardProvider } from './keyboard/create-keyboard-provider';
import type {
  KeyboardProviderConfig,
  KeyboardProvider,
} from './keyboard/keyboard-provider-types';
import { createKomorebiProvider } from './komorebi/create-komorebi-provider';
import type {
  KomorebiProviderConfig,
  KomorebiProvider,
} from './komorebi/komorebi-provider-types';
import type {
  MediaProviderConfig,
  MediaProvider,
} from './media/media-provider-types';
import { createMediaProvider } from './media/create-media-provider';
import { createMemoryProvider } from './memory/create-memory-provider';
import type {
  MemoryProviderConfig,
  MemoryProvider,
} from './memory/memory-provider-types';
import { createNetworkProvider } from './network/create-network-provider';
import type {
  NetworkProviderConfig,
  NetworkProvider,
} from './network/network-provider-types';
import { createTemperatureProvider } from './temperature/create-temperature-provider';
import type {
  TemperatureProviderConfig,
  TemperatureProvider,
} from './temperature/temperature-provider-types';
import { createWeatherProvider } from './weather/create-weather-provider';
import type {
  WeatherProviderConfig,
  WeatherProvider,
} from './weather/weather-provider-types';
import { createDiskProvider } from './disk/create-disk-provider';
import type {
  DiskProvider,
  DiskProviderConfig,
} from './disk/disk-provider-types';
import { createSystrayProvider } from './systray/create-systray-provider';
import type {
  SystrayProviderConfig,
  SystrayProvider,
} from './systray/systray-provider-types';

export interface ProviderConfigMap {
  audio: AudioProviderConfig;
  battery: BatteryProviderConfig;
  command: CommandProviderConfig;
  cpu: CpuProviderConfig;
  date: DateProviderConfig;
  glazewm: GlazeWmProviderConfig;
  focusedWindow: FocusedWindowProviderConfig;
  gpu: GpuProviderConfig;
  host: HostProviderConfig;
  http: HttpProviderConfig;
  ip: IpProviderConfig;
  komorebi: KomorebiProviderConfig;
  media: MediaProviderConfig;
  memory: MemoryProviderConfig;
  network: NetworkProviderConfig;
  temperature: TemperatureProviderConfig;
  weather: WeatherProviderConfig;
  keyboard: KeyboardProviderConfig;
  disk: DiskProviderConfig;
  systray: SystrayProviderConfig;
}

export interface ProviderMap {
  audio: AudioProvider;
  battery: BatteryProvider;
  command: CommandProvider;
  cpu: CpuProvider;
  date: DateProvider;
  glazewm: GlazeWmProvider;
  focusedWindow: FocusedWindowProvider;
  gpu: GpuProvider;
  host: HostProvider;
  http: HttpProvider;
  ip: IpProvider;
  komorebi: KomorebiProvider;
  media: MediaProvider;
  memory: MemoryProvider;
  network: NetworkProvider;
  temperature: TemperatureProvider;
  weather: WeatherProvider;
  keyboard: KeyboardProvider;
  disk: DiskProvider;
  systray: SystrayProvider;
}

export type ProviderType = keyof ProviderConfigMap;

export type ProviderConfig = ProviderConfigMap[keyof ProviderConfigMap];

export type ProviderOutput = ProviderMap[keyof ProviderMap]['output'];

/**
 * Creates a provider, which is a collection of functions and variables
 * that can change over time. Alternatively, multiple providers can be
 * created using {@link createProviderGroup}.
 *
 * The provider will continue to output until its `stop` function is
 * called.
 *
 * @throws If the provider config is invalid. Errors are emitted via the
 * `onError` method.
 */
export function createProvider<T extends ProviderConfig>(
  config: T,
): ProviderMap[T['type']] {
  switch (config.type) {
    case 'audio':
      return createAudioProvider(config) as any;
    case 'battery':
      return createBatteryProvider(config) as any;
    case 'command':
      return createCommandProvider(config) as any;
    case 'cpu':
      return createCpuProvider(config) as any;
    case 'date':
      return createDateProvider(config) as any;
    case 'glazewm':
      return createGlazeWmProvider(config) as any;
    case 'focusedWindow':
      return createFocusedWindowProvider(config) as any;
    case 'gpu':
      return createGpuProvider(config) as any;
    case 'host':
      return createHostProvider(config) as any;
    case 'http':
      return createHttpProvider(config) as any;
    case 'ip':
      return createIpProvider(config) as any;
    case 'komorebi':
      return createKomorebiProvider(config) as any;
    case 'media':
      return createMediaProvider(config) as any;
    case 'memory':
      return createMemoryProvider(config) as any;
    case 'network':
      return createNetworkProvider(config) as any;
    case 'temperature':
      return createTemperatureProvider(config) as any;
    case 'weather':
      return createWeatherProvider(config) as any;
    case 'keyboard':
      return createKeyboardProvider(config) as any;
    case 'disk':
      return createDiskProvider(config) as any;
    case 'systray':
      return createSystrayProvider(config) as any;
    default:
      throw new Error('Not a supported provider type.');
  }
}
