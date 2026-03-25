/**
 * 这个文件收口 `wiki-runtime` 的最小响应协议校验。
 * Agent 只依赖稳定的外壳字段，不把 core 的内部数据结构写死在这里。
 */
/** `wiki-runtime` 的最小终态响应外壳。 */
export type CoreResponse = {
  /** 是否执行成功。 */
  ok: boolean;
  /** 失败时的错误消息。 */
  error?: string | null;
  /** 成功或失败时附带的数据载荷。 */
  data?: unknown;
};

/** 长流程事件流里的阶段进度事件。 */
export type CoreProgressEvent = {
  type: "progress";
  action: string;
  phase: string;
  message: string;
  elapsed_ms: number;
  processed: number | null;
  total: number | null;
};

/** 长流程事件流里的 LLM 请求事件。 */
export type CoreLlmRequest = {
  request_id: string;
  prompt_type: string;
  prompt_version: string;
  input_hash: string;
  model?: string | null;
  system: string;
  instruction: string;
  input: unknown;
  response_schema: unknown;
};

/** 长流程事件流里的 `llm_request` 事件。 */
export type CoreLlmRequestEvent = {
  type: "llm_request";
  request: CoreLlmRequest;
};

/** 长流程事件流里的成功终态事件。 */
export type CoreResultEvent = {
  type: "result";
  response: CoreResponse;
};

/** 长流程事件流里的失败终态事件。 */
export type CoreErrorEvent = {
  type: "error";
  response: CoreResponse;
};

/** `wiki-runtime` 长流程允许输出的完整事件集合。 */
export type CoreStreamEvent =
  | CoreProgressEvent
  | CoreLlmRequestEvent
  | CoreResultEvent
  | CoreErrorEvent;

function parseCoreResponse(value: unknown): CoreResponse {
  const parsed = value as Partial<CoreResponse>;

  if (typeof parsed !== "object" || parsed === null || typeof parsed.ok !== "boolean") {
    throw new Error("invalid wiki-runtime response");
  }

  return parsed as CoreResponse;
}

/**
 * 校验 `wiki-runtime` 的最小响应协议。
 * 当前只强校验 `ok` 字段，其他字段允许随着 core 能力继续扩展。
 *
 * @param stdout `wiki-runtime` 写到标准输出的 JSON 文本。
 * @returns 返回通过最小协议校验的响应对象。
 */
export function parseResult(stdout: string): CoreResponse {
  return parseCoreResponse(JSON.parse(stdout));
}

/**
 * 校验 `wiki-runtime` 长流程使用的 NDJSON 事件行。
 *
 * @param line 单行事件 JSON。
 * @returns 返回结构化的 progress/result/error 事件。
 */
export function parseEventLine(line: string): CoreStreamEvent {
  const parsed = JSON.parse(line) as Partial<CoreStreamEvent> & {
    response?: unknown;
    request?: unknown;
  };

  if (parsed.type === "progress") {
    if (
      typeof parsed.action !== "string" ||
      typeof parsed.phase !== "string" ||
      typeof parsed.message !== "string" ||
      typeof parsed.elapsed_ms !== "number" ||
      !("processed" in parsed) ||
      !("total" in parsed)
    ) {
      throw new Error("invalid wiki-runtime progress event");
    }

    return {
      type: "progress",
      action: parsed.action,
      phase: parsed.phase,
      message: parsed.message,
      elapsed_ms: parsed.elapsed_ms,
      processed: parsed.processed ?? null,
      total: parsed.total ?? null,
    };
  }

  if (parsed.type === "llm_request") {
    const request = parsed.request as Partial<CoreLlmRequest> | undefined;

    if (
      typeof request !== "object" ||
      request === null ||
      typeof request.request_id !== "string" ||
      typeof request.prompt_type !== "string" ||
      typeof request.prompt_version !== "string" ||
      typeof request.input_hash !== "string" ||
      typeof request.system !== "string" ||
      typeof request.instruction !== "string"
    ) {
      throw new Error("invalid wiki-runtime llm_request event");
    }

    return {
      type: "llm_request",
      request: {
        request_id: request.request_id,
        prompt_type: request.prompt_type,
        prompt_version: request.prompt_version,
        input_hash: request.input_hash,
        model:
          typeof request.model === "string" || request.model == null ? request.model : null,
        system: request.system,
        instruction: request.instruction,
        input: request.input,
        response_schema: request.response_schema,
      },
    };
  }

  if (parsed.type === "result" || parsed.type === "error") {
    return {
      type: parsed.type,
      response: parseCoreResponse(parsed.response),
    };
  }

  throw new Error("invalid wiki-runtime stream event");
}

/**
 * 从终态事件恢复最终 `CoreResponse`。
 *
 * @param event `result` 或 `error` 事件。
 * @returns 返回与非流式模式等价的最终响应对象。
 */
export function responseFromTerminalEvent(
  event: CoreResultEvent | CoreErrorEvent,
): CoreResponse {
  return parseCoreResponse(event.response);
}
