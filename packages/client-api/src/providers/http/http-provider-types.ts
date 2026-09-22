import type { Provider } from '../create-base-provider';

export interface HttpProviderConfig {
  type: 'http';

  /**
   * URL to request.
   */
  url: string;

  /**
   * HTTP method to use. Defaults to `GET`.
   */
  method?: string;

  /**
   * Headers to send with the request.
   */
  headers?: Record<string, string>;

  /**
   * How often this provider refreshes in milliseconds.
   */
  refreshInterval?: number;
}

export type HttpProvider = Provider<HttpProviderConfig, HttpOutput>;

export interface HttpOutput {
  /**
   * HTTP status code.
   */
  status: number;

  /**
   * Whether the status code is in the 200-299 range.
   */
  success: boolean;

  /**
   * Response body as text.
   */
  body: string;

  /**
   * Response body parsed as JSON, or `null` if it is not JSON.
   */
  json: unknown | null;
}
