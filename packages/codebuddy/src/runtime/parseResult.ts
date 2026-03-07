export type CoreResponse = {
  ok: boolean;
  error?: string | null;
  data?: unknown;
};

export function parseResult(stdout: string): CoreResponse {
  return JSON.parse(stdout) as CoreResponse;
}
