import { expect, test, vi } from "vitest";

import { invokeTool } from "./command.js";

test("passes untrusted values as argv without constructing a shell command", async () => {
  const runner = vi.fn(async () => ({ code: 0, stdout: "ok", stderr: "" }));
  await invokeTool(runner, "tool", ["--repo", "C:/project;Remove-Item anything"], {
    cwd: "C:/safe",
    env: {},
    timeoutMs: 100,
  });
  expect(runner).toHaveBeenCalledWith(
    "tool",
    ["--repo", "C:/project;Remove-Item anything"],
    { cwd: "C:/safe", env: {}, timeoutMs: 100 },
  );
});

test("normalizes injected runner failures", async () => {
  const result = await invokeTool(async () => { throw new Error("boom"); }, "tool", [], {
    cwd: ".",
    env: {},
  });
  expect(result).toEqual({ code: 1, stdout: "", stderr: "boom" });
});
