import { expect, test } from "vitest";

import { inspectAoci, runAociBootstrap } from "./runner.js";

test("runs official AOCI init scan and doctor commands with argv arrays", async () => {
  const calls: string[][] = [];
  const result = await runAociBootstrap({
    executablePath: "C:/tools/aoci.exe",
    projectRoot: "C:/project;safe",
    language: "zh",
    env: {},
    runner: async (command, args) => {
      calls.push([command, ...args]);
      return { code: 0, stdout: args.includes("--version") ? "aoci version 0.1.0-rc12" : "{}", stderr: "" };
    },
  });
  expect(result.initialized).toBe(true);
  expect(calls).toContainEqual(["C:/tools/aoci.exe", "--repo", "C:/project;safe", "init", "--locale", "zh-CN", "--agent", "codex"]);
  expect(calls).toContainEqual(["C:/tools/aoci.exe", "--repo", "C:/project;safe", "scan"]);
  expect(calls).toContainEqual(["C:/tools/aoci.exe", "--repo", "C:/project;safe", "doctor", "--json"]);
});

test("fails closed when official AOCI gates are malformed", async () => {
  const result = await inspectAoci({
    executablePath: "aoci",
    projectRoot: "C:/project",
    env: {},
    runner: async (_command, args) => ({
      code: 0,
      stdout: args.includes("--version") ? "aoci version 0.1.0-rc12" : "not-json",
      stderr: "",
    }),
  });
  expect(result.governanceAligned).toBe(false);
  expect(result.nextActions.length).toBeGreaterThan(0);
});
