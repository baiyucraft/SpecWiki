import assert from "node:assert/strict";
import { existsSync, mkdirSync, mkdtempSync, rmSync, writeFileSync } from "node:fs";
import os from "node:os";
import path from "node:path";
import test from "node:test";
import { fileURLToPath } from "node:url";

import { resolveBuiltBinary } from "../../scripts/build-core.mjs";

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);
const rootDir = path.resolve(__dirname, "..", "..");
const e2eTargetDir = path.join(rootDir, "target", "e2e");

test("adapter init, status, update, query, sync, and rebuild work end to end", async () => {
  const repoRoot = mkdtempSync(path.join(os.tmpdir(), "codebuddy-wiki-e2e-"));
  const previousBinary = process.env.CODEBUDDY_WIKI_CORE_BIN;

  try {
    mkdirSync(path.join(repoRoot, ".git"));
    writeFileSync(path.join(repoRoot, "package.json"), JSON.stringify({ name: "demo-repo" }));
    writeFileSync(path.join(repoRoot, "src.ts"), "export const version = 1;\n");

    let binaryPath;
    try {
      binaryPath = resolveBuiltBinary(rootDir, { targetDir: e2eTargetDir });
    } catch (error) {
      assert.fail(
        error instanceof Error
          ? error.message
          : "wiki-core binary is required before running the e2e test",
      );
    }

    process.env.CODEBUDDY_WIKI_CORE_BIN = binaryPath;

    const { createTools } = await import("../../packages/codebuddy/src/index.ts");
    const tools = createTools();

    const initResult = await tools.wikiInit({ repoRoot });
    assert.equal(initResult.ok, true);
    assert.equal(existsSync(path.join(repoRoot, ".wiki", "项目概述.md")), true);
    assert.equal(existsSync(path.join(repoRoot, ".wiki", "wiki.metadata.json")), true);
    assert.equal(existsSync(path.join(repoRoot, ".wiki", ".cache", "repo-scan.json")), true);

    const freshStatus = await tools.wikiStatus({ repoRoot });
    assert.equal(freshStatus.ok, true);
    assert.equal(freshStatus.data.state, "fresh");

    writeFileSync(path.join(repoRoot, "src.ts"), "export const version = 2;\n");

    const staleStatus = await tools.wikiStatus({ repoRoot });
    assert.equal(staleStatus.ok, true);
    assert.equal(staleStatus.data.state, "stale");
    assert.ok(staleStatus.data.dirty_sources.length > 0);

    const updateResult = await tools.wikiUpdate({ repoRoot });
    assert.equal(updateResult.ok, true);
    assert.equal(updateResult.data.previous_state, "stale");
    assert.equal(updateResult.data.state, "fresh");
    assert.ok(updateResult.data.updated_pages.length > 0);

    const queryResult = await tools.wikiQuery({ repoRoot, term: "项目概述" });
    assert.equal(queryResult.ok, true);
    assert.ok(
      queryResult.data.matched_pages.some((page) => page.endsWith("项目概述.md")),
    );
    assert.ok(queryResult.data.matches.length > 0);

    writeFileSync(path.join(repoRoot, ".wiki", "项目概述.md"), "# 项目概述\n\n手动补充说明\n");
    const syncResult = await tools.wikiSync({ repoRoot });
    assert.equal(syncResult.ok, true);
    assert.equal(syncResult.data.state, "fresh");
    assert.ok(syncResult.data.synced_pages.some((page) => page.endsWith("项目概述.md")));

    const rebuildResult = await tools.wikiRebuild({ repoRoot });
    assert.equal(rebuildResult.ok, true);
    assert.equal(rebuildResult.data.state, "fresh");
    assert.ok(rebuildResult.data.updated_pages.some((page) => page.endsWith("项目概述.md")));
  } finally {
    if (previousBinary === undefined) {
      delete process.env.CODEBUDDY_WIKI_CORE_BIN;
    } else {
      process.env.CODEBUDDY_WIKI_CORE_BIN = previousBinary;
    }

    rmSync(repoRoot, { recursive: true, force: true });
  }
});
