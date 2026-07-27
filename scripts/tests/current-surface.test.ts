import { existsSync, readFileSync, readdirSync } from "node:fs";
import path from "node:path";

import { expect, test } from "vitest";

const root = path.resolve(import.meta.dirname, "../..");

test("current source contains no Rust, index, knowledge, or native runtime surface", () => {
  for (const relative of ["Cargo.toml", "Cargo.lock", "crates", "packages/spec-wiki", "packages/spec-wiki-lite/src/runtime", "wiki.dev.yaml"]) {
    expect(existsSync(path.join(root, relative)), relative).toBe(false);
  }
  const sourceRoot = path.join(root, "packages", "spec-wiki-lite", "src");
  const source = readdirSync(sourceRoot, { recursive: true, withFileTypes: true })
    .filter(entry => entry.isFile() && entry.name.endsWith(".ts"))
    .map(entry => readFileSync(path.join(entry.parentPath, entry.name), "utf8"))
    .join("\n");
  expect(source).not.toMatch(/forwardCoreCommand|wiki-runtime|KnowledgeUnit|SQLite|wiki-index/u);
});
