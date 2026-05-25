/**
 * 这个文件覆盖 spec-wiki init 的宿主选择策略。
 * 它保护 UniSpec 风格的“显式参数优先、交互多选、非交互兜底”流程。
 */
import { mkdirSync, mkdtempSync, rmSync } from "node:fs";
import os from "node:os";
import path from "node:path";

import { afterEach, expect, test, vi } from "vitest";

import { selectHostsForInit } from "./orchestration/init/selectHosts.js";

const tempDirs: string[] = [];

function makeTempDir(prefix: string): string {
  const dir = mkdtempSync(path.join(os.tmpdir(), prefix));
  tempDirs.push(dir);
  return dir;
}

afterEach(() => {
  while (tempDirs.length > 0) {
    rmSync(tempDirs.pop()!, { recursive: true, force: true });
  }
});

test("selectHostsForInit returns explicit hosts without prompting", async () => {
  const repoRoot = makeTempDir("spec-wiki-select-explicit-");
  const selectMultiple = vi.fn();

  await expect(
    selectHostsForInit({
      repoRoot,
      rawTools: "claude,codebuddy",
      interactive: true,
      selectMultiple,
    }),
  ).resolves.toEqual(["claude", "codebuddy"]);

  expect(selectMultiple).not.toHaveBeenCalled();
});

test("selectHostsForInit uses the only detected host in non-interactive mode", async () => {
  const repoRoot = makeTempDir("spec-wiki-select-single-");
  mkdirSync(path.join(repoRoot, ".claude"), { recursive: true });

  await expect(
    selectHostsForInit({
      repoRoot,
      interactive: false,
    }),
  ).resolves.toEqual(["claude"]);
});

test("selectHostsForInit rejects multiple detected hosts in non-interactive mode", async () => {
  const repoRoot = makeTempDir("spec-wiki-select-multi-");
  mkdirSync(path.join(repoRoot, ".claude"), { recursive: true });
  mkdirSync(path.join(repoRoot, ".codebuddy"), { recursive: true });

  await expect(
    selectHostsForInit({
      repoRoot,
      interactive: false,
    }),
  ).rejects.toThrow(/multiple hosts detected/);
});

test("selectHostsForInit lets interactive mode choose hosts with injected multi-select", async () => {
  const repoRoot = makeTempDir("spec-wiki-select-interactive-");
  const selectMultiple = vi.fn().mockResolvedValue(["codex", "codebuddy"]);

  await expect(
    selectHostsForInit({
      repoRoot,
      interactive: true,
      selectMultiple,
    }),
  ).resolves.toEqual(["codex", "codebuddy"]);

  expect(selectMultiple).toHaveBeenCalledWith(
    expect.objectContaining({
      message: expect.stringContaining("Select hosts to bootstrap"),
    }),
  );
});

test("selectHostsForInit preselects detected hosts for interactive selection", async () => {
  const repoRoot = makeTempDir("spec-wiki-select-default-");
  mkdirSync(path.join(repoRoot, ".codex"), { recursive: true });
  mkdirSync(path.join(repoRoot, ".codebuddy"), { recursive: true });
  const selectMultiple = vi.fn().mockResolvedValue(["codex", "codebuddy"]);

  await selectHostsForInit({
    repoRoot,
    interactive: true,
    selectMultiple,
  });

  const config = selectMultiple.mock.calls[0][0];
  expect(config.choices).toEqual(
    expect.arrayContaining([
      expect.objectContaining({ value: "codex", preSelected: true, detected: true }),
      expect.objectContaining({ value: "codebuddy", preSelected: true, detected: true }),
    ]),
  );
});
