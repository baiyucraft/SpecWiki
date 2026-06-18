/**
 * 这个文件覆盖 spec-wiki 共享工具、响应解析和二进制定位的基础行为。
 * 它保证主包公开 API 的最小合同不会在后续迭代里漂移。
 */
import { mkdtempSync, mkdirSync, rmSync, writeFileSync } from "node:fs";
import os from "node:os";
import path from "node:path";

import { expect, test } from "vitest";

import { createTools } from "./tools.js";
import { parseEventLine, parseResult } from "./runtime/parseResult.js";
import { resolveBinary } from "./runtime/resolveBinary.js";

test("public wiki tools delegate all current public actions", async () => {
  const received: unknown[] = [];
  const tools = createTools(async (command) => {
    received.push(command);
    return { ok: true, data: { action: command.action } };
  }) as any;

  const results = await Promise.all([
    tools.wikiInit({ repoRoot: "demo-repo" }),
    tools.wikiStatus({ repoRoot: "demo-repo" }),
    tools.wikiUpdate({ repoRoot: "demo-repo" }),
    tools.wikiQuery({ repoRoot: "demo-repo", term: "overview" }),
    tools.wikiSync({ repoRoot: "demo-repo" }),
    tools.wikiRebuild({ repoRoot: "demo-repo" }),
  ]);

  expect(received).toEqual([
    { action: "init", repoRoot: "demo-repo" },
    { action: "status", repoRoot: "demo-repo" },
    { action: "update", repoRoot: "demo-repo" },
    { action: "query", repoRoot: "demo-repo", term: "overview" },
    { action: "sync", repoRoot: "demo-repo" },
    { action: "rebuild", repoRoot: "demo-repo" },
  ]);
  expect(results.map((result) => result.data)).toEqual([
    { action: "init" },
    { action: "status" },
    { action: "update" },
    { action: "query" },
    { action: "sync" },
    { action: "rebuild" },
  ]);
  expect("wikiSync" in tools).toBe(true);
  expect("wikiRebuild" in tools).toBe(true);
});

test("resolveBinary prefers SPEC_WIKI_RUNTIME_BIN on supported Windows x64 runtime", () => {
  const previous = process.env.SPEC_WIKI_RUNTIME_BIN;
  process.env.SPEC_WIKI_RUNTIME_BIN = "C:/custom/wiki-runtime.exe";

  try {
    expect(resolveBinary()).toBe("C:/custom/wiki-runtime.exe");
  } finally {
    if (previous === undefined) {
      delete process.env.SPEC_WIKI_RUNTIME_BIN;
    } else {
      process.env.SPEC_WIKI_RUNTIME_BIN = previous;
    }
  }
});

test("resolveBinary rejects unsupported platforms by default", () => {
  expect(() =>
    resolveBinary({
      platform: "linux",
    }),
  ).toThrow(/only supports win32/);
});

test("resolveBinary rejects unsupported architectures by default", () => {
  expect(() =>
    resolveBinary({
      platform: "win32",
      arch: "arm64",
    }),
  ).toThrow(/only supports x64/);
});

test("resolveBinary prefers bundled runtime under lib/x64-win32", () => {
  const packageRoot = mkdtempSync(path.join(os.tmpdir(), "spec-wiki-bin-"));
  const previous = process.env.SPEC_WIKI_RUNTIME_BIN;
  delete process.env.SPEC_WIKI_RUNTIME_BIN;

  try {
    const binaryDir = path.join(packageRoot, "lib", "x64-win32");
    const binaryPath = path.join(binaryDir, "wiki-runtime.exe");
    mkdirSync(binaryDir, { recursive: true });
    writeFileSync(binaryPath, "mock-binary");

    expect(resolveBinary({ packageRoot, platform: "win32", arch: "x64" })).toBe(binaryPath);
  } finally {
    if (previous === undefined) {
      delete process.env.SPEC_WIKI_RUNTIME_BIN;
    } else {
      process.env.SPEC_WIKI_RUNTIME_BIN = previous;
    }
    rmSync(packageRoot, { recursive: true, force: true });
  }
});

test("resolveBinary falls back to the workspace release path on Windows x64", () => {
  const previous = process.env.SPEC_WIKI_RUNTIME_BIN;
  delete process.env.SPEC_WIKI_RUNTIME_BIN;

  try {
    expect(resolveBinary({ platform: "win32", arch: "x64" })).toMatch(
      /target[\\/]release[\\/]wiki-runtime\.exe$/,
    );
  } finally {
    if (previous === undefined) {
      delete process.env.SPEC_WIKI_RUNTIME_BIN;
    } else {
      process.env.SPEC_WIKI_RUNTIME_BIN = previous;
    }
  }
});

test("parseResult preserves explicit core errors", () => {
  const parsed = parseResult(
    JSON.stringify({ ok: false, error: "repo root must be a git repository" }),
  );

  expect(parsed).toEqual({
    ok: false,
    error: "repo root must be a git repository",
  });
});

test("parseResult parses status preflight payload", () => {
  const parsed = parseResult(
    JSON.stringify({
      ok: true,
      data: {
        state: "stale",
        dirty_sources: ["src/app.ts"],
        dirty_pages: [".wiki/项目概述.md"],
        needs_rebuild_reason: null,
        readiness: {
          index: "missing",
          knowledge: "ready",
          projection: "ready",
          fusion: "degraded",
          restored_level: "level1",
          reasons: ["runtime_incomplete"],
        },
        recommended_action: "update",
        llm_mode_hint: "provider_configured",
        runtime_summary: {
          workflow_action: "update",
          runtime_state: "completed",
          researched_units: 3,
          compose_ready_units: 3,
          composed_units: 3,
          assembled_pages: 2,
          blocked_units: [],
        },
        gate_summary: {
          total_units: 3,
          ready_for_compose_units: 3,
          composed_units: 3,
          assembled_units: 2,
          blocked_units: 0,
          blockers: [],
        },
      },
    }),
  );

  expect(parsed).toEqual({
    ok: true,
    data: {
      state: "stale",
      dirty_sources: ["src/app.ts"],
      dirty_pages: [".wiki/项目概述.md"],
      needs_rebuild_reason: null,
      readiness: {
        index: "missing",
        knowledge: "ready",
        projection: "ready",
        fusion: "degraded",
        restored_level: "level1",
        snapshot_id: undefined,
        reasons: ["runtime_incomplete"],
      },
      recommended_action: "update",
      llm_mode_hint: "provider_configured",
      runtime_summary: {
        workflow_action: "update",
        runtime_state: "completed",
        researched_units: 3,
        compose_ready_units: 3,
        composed_units: 3,
        assembled_pages: 2,
        blocked_units: [],
        last_ready_stage: undefined,
        summary_reason: undefined,
      },
      gate_summary: {
        total_units: 3,
        ready_for_compose_units: 3,
        composed_units: 3,
        assembled_units: 2,
        blocked_units: 0,
        blockers: [],
      },
    },
  });
});

test("parseResult parses query payload readiness contract", () => {
  const parsed = parseResult(
    JSON.stringify({
      ok: true,
      data: {
        term: "wiki",
        runtime_state: "fresh",
        readiness: {
          index: "ready",
          knowledge: "ready",
          projection: "ready",
          fusion: "ready",
          restored_level: "level2",
          snapshot_id: "snapshot-1",
          reasons: [],
        },
        query_mode: "mixed",
        query_trust: "ready",
        recommended_action: "none",
        matched_pages: [],
        provenance_summary: "index_hit",
        governance_readiness: "not_enabled",
        route_groups: [
          {
            route_tag: "index_symbol_hit",
            results: [
              {
                route_tag: "index_symbol_hit",
                ref_kind: "source_symbol",
                ref_id: "symbol:handleCheckout",
                label: "handleCheckout",
                score: 0.9,
                provenance: { layer: "index", state: "ready" },
                confidence: "high",
                recommended_action: "open_source_ref",
                source_refs: [
                  {
                    ref_kind: "source_path",
                    ref_id: "src/controller.ts",
                    label: "src/controller.ts",
                  },
                ],
                extra_field: "keep-me",
              },
            ],
          },
        ],
        results: [
          {
            route_tag: "index_symbol_hit",
            ref_kind: "source_symbol",
            ref_id: "symbol:handleCheckout",
            label: "handleCheckout",
            score: 0.9,
            provenance: { layer: "index", state: "ready" },
            confidence: "high",
            recommended_action: "open_source_ref",
            source_refs: [
              {
                ref_kind: "source_path",
                ref_id: "src/controller.ts",
                label: "src/controller.ts",
              },
            ],
            extra_field: "keep-me",
          },
        ],
      },
    }),
  );

  expect(parsed.data).toEqual({
    term: "wiki",
    runtime_state: "fresh",
    readiness: {
      index: "ready",
      knowledge: "ready",
      projection: "ready",
      fusion: "ready",
      restored_level: "level2",
      snapshot_id: "snapshot-1",
      reasons: [],
    },
    query_mode: "mixed",
    query_trust: "ready",
    recommended_action: "none",
    matched_pages: [],
    provenance_summary: "index_hit",
    governance_readiness: "not_enabled",
    route_groups: [
      {
        route_tag: "index_symbol_hit",
        results: [
          {
            route_tag: "index_symbol_hit",
            ref_kind: "source_symbol",
            ref_id: "symbol:handleCheckout",
            label: "handleCheckout",
            score: 0.9,
            provenance: { layer: "index", state: "ready" },
            confidence: "high",
            recommended_action: "open_source_ref",
            source_refs: [
              {
                ref_kind: "source_path",
                ref_id: "src/controller.ts",
                label: "src/controller.ts",
              },
            ],
            extra_field: "keep-me",
          },
        ],
      },
    ],
    results: [
      {
        route_tag: "index_symbol_hit",
        ref_kind: "source_symbol",
        ref_id: "symbol:handleCheckout",
        label: "handleCheckout",
        score: 0.9,
        provenance: { layer: "index", state: "ready" },
        confidence: "high",
        recommended_action: "open_source_ref",
        source_refs: [
          {
            ref_kind: "source_path",
            ref_id: "src/controller.ts",
            label: "src/controller.ts",
          },
        ],
        extra_field: "keep-me",
      },
    ],
  });
});

test("parseResult rejects unknown query route tags", () => {
  expect(() =>
    parseResult(
      JSON.stringify({
        ok: true,
        data: {
          term: "wiki",
          runtime_state: "fresh",
          readiness: {
            index: "ready",
            knowledge: "ready",
            projection: "ready",
            fusion: "ready",
            restored_level: "level2",
            reasons: [],
          },
          query_mode: "mixed",
          query_trust: "ready",
          recommended_action: "none",
          matched_pages: [],
          provenance_summary: "index_hit",
          governance_readiness: "not_enabled",
          route_groups: [{ route_tag: "index_hit", results: [] }],
          results: [],
        },
      }),
    ),
  ).toThrow(/query route tag/i);
});

test("parseEventLine validates progress events with usage", () => {
  const event = parseEventLine(
    JSON.stringify({
      type: "progress",
      action: "init",
      phase: "parse_symbols",
      message: "解析源码符号 1/3",
      elapsed_ms: 5,
      processed: 1,
      total: 3,
      usage: {
        request_count: 1,
        input_tokens: 10,
        output_tokens: 20,
        total_tokens: 30,
        by_prompt_type: [],
        by_provider_model: [],
      },
    }),
  );

  expect(event).toEqual({
    type: "progress",
    action: "init",
    phase: "parse_symbols",
    message: "解析源码符号 1/3",
    elapsed_ms: 5,
    processed: 1,
    total: 3,
    usage: {
      request_count: 1,
      input_tokens: 10,
      output_tokens: 20,
      total_tokens: 30,
      by_prompt_type: [],
      by_provider_model: [],
    },
  });
});
