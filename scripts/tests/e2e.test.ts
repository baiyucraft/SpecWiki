/**
 * 这个文件覆盖 spec-wiki wiki <action> 的端到端 CLI forwarding。
 * 它验证共享 CLI 路径可以真正驱动当前阶段的 wiki-runtime。
 */
import { existsSync, mkdtempSync, rmSync, writeFileSync } from "node:fs";
import os from "node:os";
import path from "node:path";
import { fileURLToPath } from "node:url";

import { expect, test } from "vitest";

import { runCli } from "../../packages/spec-wiki/src/index.ts";

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);
const rootDir = path.resolve(__dirname, "..", "..");
const e2eTargetDir = path.join(rootDir, "target", "e2e");

function parseJsonLine(text: string) {
  return JSON.parse(text.trim());
}

async function runWikiCli(repoRoot: string, ...args: string[]) {
  const stdout: string[] = [];
  const stderr: string[] = [];
  const exitCode = await runCli(args, {
    cwd: repoRoot,
    env: process.env,
    stdin: process.stdin,
    stdout: (text) => stdout.push(text),
    stderr: (text) => stderr.push(text),
  });

  return {
    exitCode,
    stdout: stdout.join(""),
    stderr: stderr.join(""),
  };
}

test("spec-wiki wiki init, status, update, and query keep v0.2 knowledge runtime contract end to end", async () => {
  const repoRoot = mkdtempSync(path.join(os.tmpdir(), "spec-wiki-e2e-"));
  const previousBinary = process.env.SPEC_WIKI_RUNTIME_BIN;
  const previousStructuralRuntime = process.env.SPEC_WIKI_ALLOW_STRUCTURAL_RUNTIME;

  try {
    writeFileSync(path.join(repoRoot, "package.json"), JSON.stringify({ name: "demo-repo" }));
    writeFileSync(path.join(repoRoot, "src.ts"), "export const version = 1;\n");

    const { resolveBuiltBinary } = await import("../build/core-paths.mjs");
    let binaryPath;

    try {
      binaryPath = resolveBuiltBinary(rootDir, { targetDir: e2eTargetDir });
    } catch (error) {
      throw new Error(
        error instanceof Error
          ? error.message
          : "wiki-runtime binary is required before running the e2e test",
      );
    }

    process.env.SPEC_WIKI_RUNTIME_BIN = binaryPath;
    process.env.SPEC_WIKI_ALLOW_STRUCTURAL_RUNTIME = "1";

    const initResult = await runWikiCli(repoRoot, "wiki", "init");
    expect(initResult.exitCode).toBe(0);
    const initLines = initResult.stdout.trim().split(/\r?\n/).filter(Boolean);
    expect(initLines.some((line) => line.includes("\"type\":\"progress\""))).toBe(true);
    const initTerminal = parseJsonLine(initLines[initLines.length - 1]);
    expect(initTerminal.type).toBe("result");
    expect(initTerminal.response.data.state).toBe("fresh");
    expect((initTerminal.response.data.generated_pages ?? []).length).toBeGreaterThan(0);
    expect(existsSync(path.join(repoRoot, ".wiki", ".cache", "wiki-cache.db"))).toBe(true);
    expect(existsSync(path.join(repoRoot, ".wiki", "INDEX.md"))).toBe(true);
    expect(existsSync(path.join(repoRoot, ".wiki", "wiki.metadata.json"))).toBe(true);

    const statusResult = await runWikiCli(repoRoot, "wiki", "status");
    expect(statusResult.exitCode).toBe(0);
    const statusJson = parseJsonLine(statusResult.stdout);
    expect(statusJson.data.state).toBe("fresh");
    expect(statusJson.data.facts_ready).toBe(true);
    expect(statusJson.data.query_readiness).toBe("ready");
    expect(statusJson.data.recommended_action).toBe("none");
    expect(statusJson.data.runtime_summary ?? null).not.toBeNull();

    writeFileSync(path.join(repoRoot, "src.ts"), "export const version = 2;\n");

    const updateResult = await runWikiCli(repoRoot, "wiki", "update");
    expect(updateResult.exitCode).toBe(0);
    const updateLines = updateResult.stdout.trim().split(/\r?\n/).filter(Boolean);
    expect(updateLines.some((line) => line.includes("\"type\":\"progress\""))).toBe(true);
    const updateTerminal = parseJsonLine(updateLines[updateLines.length - 1]);
    expect(updateTerminal.type).toBe("result");
    expect(updateTerminal.response.data.state).toBe("fresh");
    expect((updateTerminal.response.data.updated_pages ?? []).length).toBeGreaterThan(0);
    expect(existsSync(path.join(repoRoot, ".wiki", "wiki.metadata.json"))).toBe(true);

    const queryResult = await runWikiCli(repoRoot, "wiki", "query", "src.ts");
    expect(queryResult.exitCode).toBe(0);
    const queryJson = parseJsonLine(queryResult.stdout);
    expect(queryJson.ok).toBe(true);
    expect(queryJson.data.provenance_summary).toContain("index_hit");
    expect(queryJson.data.provenance_summary).toContain("knowledge_hit");
    expect((queryJson.data.matched_pages ?? []).length).toBeGreaterThan(0);
    expect(queryJson.data.summary).toBeDefined();
    expect(Array.isArray(queryJson.data.hits)).toBe(true);
    expect(
      (queryJson.data.hits ?? []).some((hit: {
        hit_type: string;
        title: string;
        location: string;
        summary?: string;
        reasons?: string[];
      }) => {
        expect(typeof hit.summary).toBe("string");
        expect(Array.isArray(hit.reasons ?? [])).toBe(true);
        return (
          (hit.hit_type === "symbol" && hit.location === "src.ts")
          || (hit.hit_type === "source" && hit.location.endsWith("src.ts"))
          || (hit.hit_type === "page" && hit.location.endsWith("INDEX.md"))
        );
      }),
    ).toBe(true);
  } finally {
    if (previousBinary === undefined) {
      delete process.env.SPEC_WIKI_RUNTIME_BIN;
    } else {
      process.env.SPEC_WIKI_RUNTIME_BIN = previousBinary;
    }

    if (previousStructuralRuntime === undefined) {
      delete process.env.SPEC_WIKI_ALLOW_STRUCTURAL_RUNTIME;
    } else {
      process.env.SPEC_WIKI_ALLOW_STRUCTURAL_RUNTIME = previousStructuralRuntime;
    }

    rmSync(repoRoot, { recursive: true, force: true });
  }
});
