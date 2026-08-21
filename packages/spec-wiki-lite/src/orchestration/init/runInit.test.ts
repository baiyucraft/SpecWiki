import { expect, test } from "vitest";

import { runBootstrapInit } from "./runInit.js";

test("core init remains ready when CodeGraph reports warnings", async () => {
  const result = await runBootstrapInit({
    repoRoot: ".",
    hosts: "codex",
    env: {},
    codegraph: {
      enabled: false,
      runner: async () => ({ code: 1, stdout: "", stderr: "not used" }),
    },
  });
  expect(result.outcome).toBe("ready");
  expect(result.codegraph?.requested).toBe(false);
});
