export type CoreResponse = {
  ok: boolean;
  error?: string | null;
  data?: unknown;
};

export function parseResult(stdout: string): CoreResponse {
  const parsed = JSON.parse(stdout) as Partial<CoreResponse>;

  if (typeof parsed !== "object" || parsed === null || typeof parsed.ok !== "boolean") {
    throw new Error("invalid wiki-core response");
  }

  return parsed as CoreResponse;
}
