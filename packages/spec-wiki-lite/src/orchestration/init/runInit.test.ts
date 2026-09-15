import { expect, test } from "vitest";

import { runBootstrapInit } from "./runInit.js";

test("core init succeeds but reports incomplete when tools are deferred", async () => {
  const result = await runBootstrapInit({
    repoRoot: ".",
    hosts: "codex",
    env: {},
    codegraph: { mode: "skip" },
    aoci: { mode: "skip" },
  });
  expect(result.outcome).toBe("incomplete");
  expect(result.tools?.codegraph.requested).toBe(false);
  expect(result.tools?.aoci.requested).toBe(false);
  expect(result.tools?.ready).toBe(false);
});
