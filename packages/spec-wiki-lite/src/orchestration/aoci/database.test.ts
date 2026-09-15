import { expect, test } from "vitest";

import { inspectAoci } from "./runner.js";

test("does not require Database Cognition when no source is declared", async () => {
  const result = await inspectAoci({
    executablePath: "aoci",
    projectRoot: "C:/project",
    env: {},
    runner: async (_command, args) => ({
      code: 0,
      stdout: args.includes("--version")
        ? "aoci version 0.1.0-rc12"
        : args.includes("list") ? JSON.stringify({ sources: [] }) : JSON.stringify({ ok: true }),
      stderr: "",
    }),
  });
  expect(result.database).toEqual({ required: false, sourceCount: 0, ready: true });
  expect(result.governanceAligned).toBe(true);
});

test("requires access and aligned Database Cognition for declared sources without exposing credentials", async () => {
  const secret = "postgres://user:password@example.invalid/db";
  const result = await inspectAoci({
    executablePath: "aoci",
    projectRoot: "C:/project",
    env: {},
    runner: async (_command, args) => {
      if (args.includes("--version")) return { code: 0, stdout: "aoci version 0.1.0-rc12", stderr: "" };
      if (args.includes("list")) return { code: 0, stdout: JSON.stringify({ sources: [{ id: "main" }] }), stderr: "" };
      if (args.includes("access")) return { code: 0, stdout: JSON.stringify({ ready: true, credential: secret }), stderr: "" };
      if (args.includes("cognition")) return { code: 0, stdout: JSON.stringify({ aligned: true }), stderr: "" };
      return { code: 0, stdout: JSON.stringify({ ok: true }), stderr: "" };
    },
  });
  expect(result.database).toEqual({ required: true, sourceCount: 1, ready: true });
  expect(JSON.stringify(result)).not.toContain(secret);
});
