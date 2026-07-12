import {
  parseEventLine,
  responseFromTerminalEvent,
  type CoreResponse,
  type CoreStreamEvent,
} from "./parseResult.js";

export type CoreEventStreamHandlers = {
  onEvent?: (event: CoreStreamEvent) => void;
  onProgress?: (event: Extract<CoreStreamEvent, { type: "progress" }>) => void;
  onLlmRequest?: (event: Extract<CoreStreamEvent, { type: "llm_request" }>) => void;
};

/** Incrementally validates a runtime NDJSON stream and enforces one terminal event. */
export class CoreEventStream {
  private buffer = "";
  private terminal: CoreResponse | undefined;
  private ended = false;

  public constructor(private readonly handlers: CoreEventStreamHandlers = {}) {}

  public push(chunk: string): void {
    if (this.ended) {
      throw new Error("received event after stream ended");
    }
    this.buffer += chunk;
    this.drain(false);
  }

  public finish(): CoreResponse {
    this.drain(true);
    this.ended = true;
    if (!this.terminal) {
      throw new Error("wiki-runtime stream ended without terminal event");
    }
    return this.terminal;
  }

  private drain(flushRemainder: boolean): void {
    while (true) {
      const newline = this.buffer.indexOf("\n");
      if (newline < 0)
break;
      const line = this.buffer.slice(0, newline).trim();
      this.buffer = this.buffer.slice(newline + 1);
      if (line)
this.consume(line);
    }
    if (flushRemainder && this.buffer.trim()) {
      const line = this.buffer.trim();
      this.buffer = "";
      this.consume(line);
    }
  }

  private consume(line: string): void {
    const event = parseEventLine(line);
    if (this.terminal) {
      throw new Error("received event after terminal event");
    }
    this.handlers.onEvent?.(event);
    if (event.type === "progress") {
      this.handlers.onProgress?.(event);
      return;
    }
    if (event.type === "llm_request") {
      this.handlers.onLlmRequest?.(event);
      return;
    }
    if (event.type === "result" || event.type === "error") {
      this.terminal = responseFromTerminalEvent(event);
    }
  }
}

export function consumeCoreEventStream(
  chunks: Iterable<string>,
  handlers: CoreEventStreamHandlers = {},
): CoreResponse {
  const stream = new CoreEventStream(handlers);
  for (const chunk of chunks) stream.push(chunk);
  return stream.finish();
}
