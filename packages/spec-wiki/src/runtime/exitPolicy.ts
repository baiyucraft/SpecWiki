import type { CoreResponse } from "./parseResult.js";

export const EXIT_CODES = {
  success: 0,
  partial: 2,
  usage: 64,
  failure: 1,
} as const;

export type ExitOutcome = "success" | "partial" | "usage" | "failure";

/** Map a typed runtime response to the public CLI exit contract. */
export function exitCodeForResponse(response: CoreResponse): number {
  if (response.ok) {
    const data = response.data as unknown;
    if (isRecord(data) && data.outcome === "partial") {
      return EXIT_CODES.partial;
    }
    if (isRecord(data) && isRecord(data.validation) && data.validation.valid === false) {
      return EXIT_CODES.partial;
    }
    return EXIT_CODES.success;
  }

  if (response.errorKind === "invalid_argument")
return EXIT_CODES.usage;

  return EXIT_CODES.failure;
}

export const exitCodeFor = exitCodeForResponse;

export function exitCodeForOutcome(outcome: ExitOutcome): number {
  return EXIT_CODES[outcome];
}

function isRecord(value: unknown): value is Record<string, unknown> {
  return typeof value === "object" && value !== null;
}
