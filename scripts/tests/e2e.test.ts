import { existsSync, mkdtempSync, rmSync, writeFileSync } from "node:fs";
import os from "node:os";
import path from "node:path";
import { fileURLToPath } from "node:url";

import { expect, test } from "vitest";

import { createTools } from "../../agents/codebuddy/src/index.ts";

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);
const rootDir = path.resolve(__dirname, "..", "..");
const e2eTargetDir = path.join(rootDir, "crates", "wiki-core", "target", "e2e");

test("agent init, status, update, query, sync, and rebuild work end to end", async () => {
  const repoRoot = mkdtempSync(path.join(os.tmpdir(), "codebuddy-wiki-e2e-"));
  const previousBinary = process.env.CODEBUDDY_WIKI_CORE_BIN;

  try {
    writeFileSync(path.join(repoRoot, "package.json"), JSON.stringify({ name: "demo-repo" }));
    writeFileSync(path.join(repoRoot, "src.ts"), "export const version = 1;\n");

    const { resolveBuiltBinary } = await import("../core-paths.mjs");
    let binaryPath;

    try {
      binaryPath = resolveBuiltBinary(rootDir, { targetDir: e2eTargetDir });
    } catch (error) {
      throw new Error(
        error instanceof Error
          ? error.message
          : "wiki-core binary is required before running the e2e test",
      );
    }

    process.env.CODEBUDDY_WIKI_CORE_BIN = binaryPath;

    const tools = createTools() as any;

    const initResult = await tools.wikiInit({ repoRoot });
    expect(initResult.ok).toBe(true);
    expect(existsSync(path.join(repoRoot, ".wiki", "项目概述.md"))).toBe(true);
    expect(existsSync(path.join(repoRoot, ".wiki", "系统架构.md"))).toBe(true);
    expect(existsSync(path.join(repoRoot, ".wiki", "wiki.metadata.json"))).toBe(true);
    expect(existsSync(path.join(repoRoot, ".wiki", ".cache", "repo-scan.json"))).toBe(true);
    expect(existsSync(path.join(repoRoot, ".wiki", ".cache", "module-tree.json"))).toBe(true);

    const freshStatus = await tools.wikiStatus({ repoRoot });
    expect(freshStatus.ok).toBe(true);
    expect(freshStatus.data.state).toBe("fresh");

    writeFileSync(path.join(repoRoot, "src.ts"), "export const version = 2;\n");

    const staleStatus = await tools.wikiStatus({ repoRoot });
    expect(staleStatus.ok).toBe(true);
    expect(staleStatus.data.state).toBe("stale");
    expect(staleStatus.data.dirty_sources.length).toBeGreaterThan(0);

    const updateResult = await tools.wikiUpdate({ repoRoot });
    expect(updateResult.ok).toBe(true);
    expect(updateResult.data.previous_state).toBe("stale");
    expect(updateResult.data.state).toBe("fresh");
    expect(updateResult.data.updated_pages.length).toBeGreaterThan(0);

    const queryResult = await tools.wikiQuery({ repoRoot, term: "项目概述" });
    expect(queryResult.ok).toBe(true);
    expect(
      queryResult.data.matched_pages.some((page: string) => page.endsWith("项目概述.md")),
    ).toBe(true);
    expect(queryResult.data.matches.length).toBeGreaterThan(0);
    expect(queryResult.data.matched_modules.length).toBeGreaterThan(0);
    expect(queryResult.data.matched_sources.length).toBeGreaterThan(0);

    writeFileSync(path.join(repoRoot, ".wiki", "项目概述.md"), "# 项目概述\n\n手动补充说明\n");
    const syncResult = await tools.wikiSync({ repoRoot });
    expect(syncResult.ok).toBe(true);
    expect(syncResult.data.state).toBe("fresh");
    expect(
      syncResult.data.synced_pages.some((page: string) => page.endsWith("项目概述.md")),
    ).toBe(true);

    const rebuildResult = await tools.wikiRebuild({ repoRoot });
    expect(rebuildResult.ok).toBe(true);
    expect(rebuildResult.data.state).toBe("fresh");
    expect(
      rebuildResult.data.updated_pages.some((page: string) => page.endsWith("项目概述.md")),
    ).toBe(true);
  } finally {
    if (previousBinary === undefined) {
      delete process.env.CODEBUDDY_WIKI_CORE_BIN;
    } else {
      process.env.CODEBUDDY_WIKI_CORE_BIN = previousBinary;
    }

    rmSync(repoRoot, { recursive: true, force: true });
  }
});
