import { mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import os from "node:os";
import path from "node:path";

import { afterEach, expect, test } from "vitest";

import {
  DEFAULT_WIKI_LANGUAGE,
  readProjectConfig,
  writeProjectLanguage,
} from "./config.js";

const roots: string[] = [];

function makeRoot(): string {
  const root = mkdtempSync(path.join(os.tmpdir(), "spec-wiki-lite-config-"));
  roots.push(root);
  return root;
}

afterEach(() => {
  for (const root of roots.splice(0)) {
    rmSync(root, { recursive: true, force: true });
  }
});

test("defaults a missing project config to Chinese", () => {
  expect(readProjectConfig(makeRoot())).toEqual({
    exists: false,
    language: DEFAULT_WIKI_LANGUAGE,
    version: 1,
  });
  expect(DEFAULT_WIKI_LANGUAGE).toBe("zh");
});

test("writes language while preserving unknown fields and comments", () => {
  const root = makeRoot();
  writeProjectLanguage(root, "zh");
  const configPath = path.join(root, ".wiki", "config.yaml");
  writeFileSync(configPath, [
    "# shared project settings",
    "version: 1",
    "wiki:",
    "  language: zh",
    "llm:",
    "  model: provider/model",
    "",
  ].join("\n"), "utf8");

  expect(writeProjectLanguage(root, "en")).toBe("updated");
  const content = readFileSync(configPath, "utf8");
  expect(content).toContain("# shared project settings");
  expect(content).toContain("language: en");
  expect(content).toContain("model: provider/model");
  expect(readProjectConfig(root)).toEqual(expect.objectContaining({
    exists: true,
    language: "en",
    version: 1,
  }));
});

test.each([
  ["invalid YAML", "version: ["],
  ["unknown version", "version: 2\nwiki:\n  language: zh\n"],
  ["invalid language", "version: 1\nwiki:\n  language: fr\n"],
  ["missing wiki", "version: 1\n"],
])("rejects %s without rewriting it", (_name, content) => {
  const root = makeRoot();
  const configPath = path.join(root, ".wiki", "config.yaml");
  writeProjectLanguage(root, "zh");
  writeFileSync(configPath, content, "utf8");

  expect(() => readProjectConfig(root)).toThrow();
  expect(() => writeProjectLanguage(root, "en")).toThrow();
  expect(readFileSync(configPath, "utf8")).toBe(content);
});
