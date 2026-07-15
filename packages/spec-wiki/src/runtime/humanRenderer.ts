import type { CoreProgressEvent, CoreResponse, CoreStreamEvent } from "./parseResult.js";

export type HumanRenderOptions = { action?: string };

/** Translate already-validated DTOs; never infer readiness or recommended actions. */
export function renderHumanResponse(
  response: CoreResponse,
  options: HumanRenderOptions = {},
): string {
  if (!response.ok) {
    const kind = response.errorKind ? ` [${response.errorKind}]` : "";
    return `Error${kind}: ${response.error ?? "wiki-runtime failed"}\n`;
  }

  const data = response.data as unknown;
  if (!isRecord(data))
return `${options.action ?? "done"}: ok\n`;
  if (isRecord(data.manifest)) {
    return renderArchiveManifest(data.manifest, options.action ?? "archive");
  }
  const lines = [`${options.action ?? "done"}: ${stringValue(data.outcome) ?? "ok"}`];
  for (const key of ["state", "runtime_state", "query_mode", "query_trust", "recommended_action"]) {
    const value = data[key];
    if (typeof value === "string")
lines.push(`${key}: ${value}`);
  }
  if (Array.isArray(data.updated_pages))
lines.push(`updated_pages: ${data.updated_pages.length}`);
  if (Array.isArray(data.route_groups)) {
    lines.push(`route_groups: ${data.route_groups.length}`);
    const resultCount = data.route_groups.reduce((count, group) => {
      if (!isRecord(group))
return count;
      if (typeof group.returned_count === "number")
return count + group.returned_count;
      return count + (Array.isArray(group.results) ? group.results.length : 0);
    }, 0);
    lines.push(`results: ${resultCount}`);
  }
  if (isRecord(data.answer)) {
    for (const [field, label] of [["answer_mode", "answer_mode"], ["answer_trust", "answer_trust"]] as const) {
      const value = data.answer[field];
      if (typeof value === "string")
lines.push(`${label}: ${value}`);
    }
    if (typeof data.answer.text === "string" && data.answer.text.trim())
lines.push(`answer: ${data.answer.text}`);
  }
  return `${lines.join("\n")}\n`;
}

function renderArchiveManifest(manifest: Record<string, unknown>, action: string): string {
  const lines = [`${action}: ${stringValue(manifest.outcome) ?? "ok"}`];
  for (const key of [
    "operation_id",
    "mode",
    "status",
    "step",
    "source_path",
    "target_path",
    "recovery_hint",
  ]) {
    const value = manifest[key];
    if (typeof value === "string")
lines.push(`${key}: ${value}`);
  }
  if (typeof manifest.persisted === "boolean")
lines.push(`persisted: ${manifest.persisted}`);
  if (typeof manifest.resumable === "boolean")
lines.push(`resumable: ${manifest.resumable}`);
  return `${lines.join("\n")}\n`;
}

export const render = renderHumanResponse;

export function renderHumanEvent(event: CoreStreamEvent): string {
  if (event.type === "progress")
return renderProgress(event);
  if (event.type === "llm_request")
return `llm_request: ${event.request.prompt_type}\n`;
  if (event.type === "result" || event.type === "error")
return renderHumanResponse(event.response);
  return `${event.type}\n`;
}

function renderProgress(event: CoreProgressEvent): string {
  const progress = event.processed == null || event.total == null
    ? ""
    : ` ${event.processed}/${event.total}`;
  return `${event.action} ${event.phase}${progress}: ${event.message}\n`;
}

function isRecord(value: unknown): value is Record<string, unknown> {
  return typeof value === "object" && value !== null;
}

function stringValue(value: unknown): string | undefined {
  return typeof value === "string" ? value : undefined;
}
