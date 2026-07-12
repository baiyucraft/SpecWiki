import { expect, test } from "vitest";

import { CoreEventStream } from "./coreEventStream.js";

test("coreEventStream accepts chunk boundaries and one terminal", () => {
  const stream = new CoreEventStream();
  stream.push("{\"type\":\"progress\",\"action\":\"update\",\"phase\":\"scan\",\"message\":\"scan\",\"elapsed_ms\":1,\"processed\":0");
  stream.push(",\"total\":1}\n{\"type\":\"result\",\"response\":{\"ok\":true,\"data\":{\"outcome\":\"ready\"}}}\n");
  expect(stream.finish().ok).toBe(true);
});

test("coreEventStream rejects missing and post-terminal events", () => {
  expect(() => new CoreEventStream().finish()).toThrow("without terminal");
  const stream = new CoreEventStream();
  stream.push("{\"type\":\"result\",\"response\":{\"ok\":true}}\n");
  expect(() => stream.push("{\"type\":\"progress\",\"action\":\"x\",\"phase\":\"x\",\"message\":\"x\",\"elapsed_ms\":0,\"processed\":null,\"total\":null}\n")).toThrow("after terminal");
});

test("coreEventStream rejects malformed agent bridge events", () => {
  const stream = new CoreEventStream();
  expect(() => stream.push(`${JSON.stringify({ type: "agent_final", requestId: "req-1" })}\n`))
    .toThrow(/agent final event/);
});
