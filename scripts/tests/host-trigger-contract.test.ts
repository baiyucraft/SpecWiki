import { mkdtempSync, readFileSync, rmSync } from "node:fs";
import { tmpdir } from "node:os";
import path from "node:path";
import { fileURLToPath, pathToFileURL } from "node:url";
import { spawnSync } from "node:child_process";

import { expect, test } from "vitest";

import { buildHostBootstrapAssets } from "../../packages/spec-wiki/src/agents/shared/hostAssets.js";
import { HOSTS } from "../../packages/spec-wiki/src/agents/shared/hosts.js";
import { parseHostTriggerCorpus } from "../../packages/spec-wiki/src/agents/shared/triggerCorpus.js";

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..", "..");

function readText(relativePath: string): string {
  return readFileSync(path.join(root, relativePath), "utf8");
}

function executeGeneratedHook(filePath: string, stdin: string): Record<string, unknown> {
  const result = spawnSync(process.execPath, [filePath], { input: stdin, encoding: "utf8" });
  expect(result.status, result.stderr).toBe(0);
  return JSON.parse(result.stdout) as Record<string, unknown>;
}

test("keeps Codex as the only reference host and aligns assets with capabilities", () => {
  const references = HOSTS.filter(host => host.compatibilityRole === "reference");
  expect(references.map(host => host.id)).toEqual(["codex"]);

  for (const host of HOSTS) {
    const assets = buildHostBootstrapAssets(root, process.env, host.id);
    expect(assets.length, host.id).toBeGreaterThan(0);
    if (host.id === "codebuddy") {
      expect(assets.some(asset => asset.kind === "hook" && asset.filePath.endsWith("user-prompt-submit.mjs"))).toBe(true);
    } else {
      expect(assets.some(asset => asset.kind === "hook")).toBe(false);
    }
  }
});

test("keeps trigger artifacts free of runtime-private and durable session fields", () => {
  const triggerOwnedFiles = [
    "packages/spec-wiki/src/agents/shared/triggerContract.ts",
    "packages/spec-wiki/src/agents/shared/triggerRuntime.ts",
    "packages/spec-wiki/src/agents/shared/triggerCorpus.ts",
    "packages/spec-wiki/src/agents/codebuddy/triggerAdapter.ts",
    "packages/spec-wiki/tests/fixtures/host-trigger-corpus.v1.json",
  ];
  const forbidden = /matched_pages|provenance_summary|session_summary|recent_turns|tool_artifact_refs/;

  for (const relativePath of triggerOwnedFiles) {
    expect(readText(relativePath), relativePath).not.toMatch(forbidden);
    expect(relativePath.startsWith(".spec/archive/")).toBe(false);
  }

  const corpus = parseHostTriggerCorpus(JSON.parse(readText("packages/spec-wiki/tests/fixtures/host-trigger-corpus.v1.json")));
  expect(corpus.contractVersion).toBe("host-trigger/v1");
  expect(corpus.adapterCases.length).toBeGreaterThan(0);
});

test("keeps query fields canonical and bridge/session concerns out of trigger ownership", () => {
  const querySkillAssets = HOSTS.flatMap(host => buildHostBootstrapAssets(root, process.env, host.id))
    .filter(asset =>
      asset.kind === "skill"
      && asset.filePath.replaceAll("\\", "/").endsWith("wiki-query/SKILL.md"),
    );
  expect(querySkillAssets).not.toHaveLength(0);
  for (const asset of querySkillAssets) {
    expect(asset.content).toContain("`route_groups`");
    expect(asset.content).toContain("`answer`");
    expect(asset.content).not.toMatch(/matched_pages|provenance_summary|owner|entrypoint|impact/);
  }

  const bridgeSpec = readText(".wiki/05-规格基线/capabilities/agent-session-bridge/spec.md");
  expect(bridgeSpec).toContain("production `research_page` 与多轮 agent-session bridge 尚未启用");
  expect(bridgeSpec).toContain("不参与 host trigger decision");
  expect(triggerOwnedPathSet().some(relativePath => relativePath.includes("agent-session-bridge"))).toBe(false);
});

test("keeps the built distribution hook aligned with the complete trigger corpus", async () => {
  const repoRoot = mkdtempSync(path.join(tmpdir(), "spec-wiki-dist-trigger-"));
  try {
    const distributionEntry = pathToFileURL(path.join(root, "packages", "spec-wiki", "dist", "index.js")).href;
    const distribution = await import(distributionEntry) as {
      runBootstrapInit: typeof import("../../packages/spec-wiki/src/orchestration/init/runInit.js")["runBootstrapInit"];
    };
    await distribution.runBootstrapInit({ repoRoot, hosts: "codebuddy", env: process.env });
    const hookPath = path.join(repoRoot, ".codebuddy", "hooks", "spec-wiki", "user-prompt-submit.mjs");
    const corpus = parseHostTriggerCorpus(JSON.parse(readText("packages/spec-wiki/tests/fixtures/host-trigger-corpus.v1.json")));

    for (const testCase of corpus.semanticCases) {
      const stdin = JSON.stringify({
        hook_event_name: "UserPromptSubmit",
        user_prompt: testCase.input.text,
      });
      const output = executeGeneratedHook(hookPath, stdin);
      const shouldHaveContext = testCase.expected.decision === "should_trigger"
        || (testCase.expected.decision === "ambiguous" && testCase.expected.reason !== "invalid_input");
      expect("additionalContext" in output, testCase.id).toBe(shouldHaveContext);
    }
    for (const testCase of corpus.adapterCases) {
      const output = executeGeneratedHook(hookPath, testCase.stdin);
      expect("additionalContext" in output, testCase.id).toBe(testCase.expectedDelivery !== "none");
    }
  } finally {
    rmSync(repoRoot, { recursive: true, force: true });
  }
});

function triggerOwnedPathSet(): string[] {
  return [
    "packages/spec-wiki/src/agents/shared/triggerContract.ts",
    "packages/spec-wiki/src/agents/shared/triggerRuntime.ts",
    "packages/spec-wiki/src/agents/shared/triggerCorpus.ts",
    "packages/spec-wiki/src/agents/codebuddy/triggerAdapter.ts",
  ];
}
