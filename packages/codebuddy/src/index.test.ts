import assert from "node:assert/strict";
import test from "node:test";

import { createTools } from "./index.js";
import { resolveBinary } from "./runtime/resolveBinary.js";

test("wikiStatus delegates to the core invoker", async () => {
  let received: unknown;
  const tools = createTools(async (command) => {
    received = command;
    return { ok: true, data: { state: "fresh" } };
  });

  const result = await tools.wikiStatus({ repoRoot: "demo-repo" });

  assert.deepEqual(received, { action: "status", repoRoot: "demo-repo" });
  assert.deepEqual(result, { ok: true, data: { state: "fresh" } });
});

test("resolveBinary prefers CODEBUDDY_WIKI_CORE_BIN", () => {
  const previous = process.env.CODEBUDDY_WIKI_CORE_BIN;
  process.env.CODEBUDDY_WIKI_CORE_BIN = "C:/custom/wiki-core.exe";

  try {
    assert.equal(resolveBinary(), "C:/custom/wiki-core.exe");
  } finally {
    if (previous === undefined) {
      delete process.env.CODEBUDDY_WIKI_CORE_BIN;
    } else {
      process.env.CODEBUDDY_WIKI_CORE_BIN = previous;
    }
  }
});
