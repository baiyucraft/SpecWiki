import { expect, test } from "vitest";

import { createTools } from "./index.js";
import { parseResult } from "./runtime/parseResult.js";
import { resolveBinary } from "./runtime/resolveBinary.js";

test("all wiki tools delegate to the expected core action", async () => {
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
});

test("resolveBinary prefers CODEBUDDY_WIKI_CORE_BIN", () => {
  const previous = process.env.CODEBUDDY_WIKI_CORE_BIN;
  process.env.CODEBUDDY_WIKI_CORE_BIN = "C:/custom/wiki-core.exe";

  try {
    expect(resolveBinary()).toBe("C:/custom/wiki-core.exe");
  } finally {
    if (previous === undefined) {
      delete process.env.CODEBUDDY_WIKI_CORE_BIN;
    } else {
      process.env.CODEBUDDY_WIKI_CORE_BIN = previous;
    }
  }
});

test("resolveBinary rejects unsupported platforms without override", () => {
  const previous = process.env.CODEBUDDY_WIKI_CORE_BIN;
  delete process.env.CODEBUDDY_WIKI_CORE_BIN;

  try {
    expect(() => resolveBinary("linux")).toThrow(/Windows only/);
  } finally {
    if (previous === undefined) {
      delete process.env.CODEBUDDY_WIKI_CORE_BIN;
    } else {
      process.env.CODEBUDDY_WIKI_CORE_BIN = previous;
    }
  }
});

test("resolveBinary falls back to the workspace build path on Windows", () => {
  const previous = process.env.CODEBUDDY_WIKI_CORE_BIN;
  delete process.env.CODEBUDDY_WIKI_CORE_BIN;

  try {
    expect(resolveBinary("win32")).toMatch(
      /crates[\\/]wiki-core[\\/]target[\\/]debug[\\/]wiki-core\.exe$/,
    );
  } finally {
    if (previous === undefined) {
      delete process.env.CODEBUDDY_WIKI_CORE_BIN;
    } else {
      process.env.CODEBUDDY_WIKI_CORE_BIN = previous;
    }
  }
});

test("parseResult preserves explicit core errors", () => {
  const parsed = parseResult(JSON.stringify({ ok: false, error: "repo root must be a git repository" }));

  expect(parsed).toEqual({
    ok: false,
    error: "repo root must be a git repository",
  });
});
