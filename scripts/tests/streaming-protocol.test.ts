/**
 * 覆盖 JSON streaming 协议与测试脚本调度契约。
 *
 * 这里的附加用例只验证顺序执行、timeout 透传与瞬态错误识别，
 * 不把 orchestration 回归和真实仓库长流程耦合在一起。
 */

import { mkdtempSync, rmSync, writeFileSync } from "node:fs";
import os from "node:os";
import path from "node:path";
import { spawnSync } from "node:child_process";
import { setTimeout as delay } from "node:timers/promises";
import { fileURLToPath } from "node:url";

import { expect, test } from "vitest";

import {
  isTransientFsErrorMessage,
  runCommandCapture,
  runSequentialTasks,
} from "../testing/helpers.mjs";
import {
  buildRunTestProjectChildArgs,
  parseCliArgs as parseRunTestCliArgs,
} from "../run-test-projects.mjs";
import {
  buildLifecycleProjectChildArgs,
  parseCliArgs as parseLifecycleCliArgs,
} from "../test-wiki-lifecycle.mjs";

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);
const rootDir = path.resolve(__dirname, "..", "..");
const e2eTargetDir = path.join(rootDir, "target", "e2e");

test("built wiki-core streams NDJSON progress for init", async () => {
  const repoRoot = mkdtempSync(path.join(os.tmpdir(), "wiki-core-stream-"));

  try {
    writeFileSync(path.join(repoRoot, "package.json"), JSON.stringify({ name: "stream-demo" }));
    writeFileSync(path.join(repoRoot, "src.ts"), "export const version = 1;\n");

    const { resolveBuiltBinary } = await import("../build/core-paths.mjs");
    const binaryPath = resolveBuiltBinary(rootDir, { targetDir: e2eTargetDir });
    const command = JSON.stringify({
      action: "init",
      repoRoot,
    });

    const result = spawnSync(binaryPath, ["--json"], {
      input: command,
      encoding: "utf8",
    });

    expect(result.status).toBe(0);
    const lines = result.stdout
      .trim()
      .split(/\r?\n/)
      .filter((line) => line.length > 0)
      .map((line) => JSON.parse(line));

    expect(lines.length).toBeGreaterThan(1);
    expect(lines.some((event) => event.type === "progress")).toBe(true);
    expect(lines.at(-1)?.type).toBe("result");
    expect(lines.at(-1)?.response?.ok).toBe(true);
  } finally {
    rmSync(repoRoot, { recursive: true, force: true });
  }
});

test("runSequentialTasks 会严格串行执行 worker", async () => {
  let active = 0;
  let maxActive = 0;

  const results = await runSequentialTasks(["storybook", "dagger", "axum"], async (name) => {
    active += 1;
    maxActive = Math.max(maxActive, active);
    await delay(25);
    active -= 1;
    return name.toUpperCase();
  });

  expect(results).toEqual(["STORYBOOK", "DAGGER", "AXUM"]);
  expect(maxActive).toBe(1);
});

test("runCommandCapture 在超时时返回统一结果契约", async () => {
  const result = await runCommandCapture(
    process.execPath,
    ["-e", "setTimeout(() => {}, 30_000)"],
    { timeoutMs: 100 },
  );

  expect(result.timedOut).toBe(true);
  expect(result.code).toBe(-1);
  expect(result.signal).not.toBeUndefined();
  expect(result.stderr).toContain("timed out after 100ms");
});

test("run-test-projects 会解析并透传 timeout 参数", () => {
  const parsed = parseRunTestCliArgs([
    "--child-json",
    "--timeout-minutes",
    "90",
    "--run-mode",
    "warm",
    "storybook",
  ]);

  expect(parsed.childMode).toBe(true);
  expect(parsed.runMode).toBe("warm");
  expect(parsed.timeoutMs).toBe(90 * 60_000);
  expect(buildRunTestProjectChildArgs("storybook", parsed)).toEqual([
    expect.stringContaining("run-test-projects.mjs"),
    "--child-json",
    "--no-build",
    "--run-mode",
    "warm",
    "--timeout-minutes",
    "90",
    "storybook",
  ]);
});

test("lifecycle child args 会保留 phase 与 timeout", () => {
  const parsed = parseLifecycleCliArgs([
    "--phase",
    "steady",
    "--timeout-minutes",
    "120",
    "--child-json",
    "dagger",
  ]);

  expect(parsed.phase).toBe("steady");
  expect(parsed.timeoutMs).toBe(120 * 60_000);
  expect(buildLifecycleProjectChildArgs("dagger", parsed)).toEqual([
    expect.stringContaining("test-wiki-lifecycle.mjs"),
    "--child-json",
    "--phase",
    "steady",
    "--run-mode",
    "cold",
    "--no-build",
    "--timeout-minutes",
    "120",
    "dagger",
  ]);
});

test("瞬态错误识别会覆盖 os error 32", () => {
  expect(isTransientFsErrorMessage("EBUSY: resource busy")).toBe(true);
  expect(isTransientFsErrorMessage("另一个程序正在使用此文件，进程无法访问。 (os error 32)")).toBe(true);
  expect(isTransientFsErrorMessage("provider returned 429")).toBe(false);
});
