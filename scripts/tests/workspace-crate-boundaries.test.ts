import { readFileSync } from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

import { expect, test } from "vitest";

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);
const rootDir = path.resolve(__dirname, "..", "..");

function readWorkspaceFile(...segments: string[]) {
  return readFileSync(path.join(rootDir, ...segments), "utf8");
}

test("workspace declares the four crate members", () => {
  const manifest = readWorkspaceFile("Cargo.toml");

  expect(manifest).toContain('"crates/wiki-model"');
  expect(manifest).toContain('"crates/wiki-index"');
  expect(manifest).toContain('"crates/wiki-knowledge"');
  expect(manifest).toContain('"crates/wiki-runtime"');
});

test("crate dependency direction stays one-way", () => {
  const indexManifest = readWorkspaceFile("crates", "wiki-index", "Cargo.toml");
  const knowledgeManifest = readWorkspaceFile("crates", "wiki-knowledge", "Cargo.toml");
  const runtimeManifest = readWorkspaceFile("crates", "wiki-runtime", "Cargo.toml");

  expect(indexManifest).toContain('wiki-model = { path = "../wiki-model" }');
  expect(indexManifest).not.toContain("wiki-runtime");

  expect(knowledgeManifest).toContain('wiki-model = { path = "../wiki-model" }');
  expect(knowledgeManifest).toContain('wiki-index = { path = "../wiki-index" }');
  expect(knowledgeManifest).not.toContain("wiki-runtime");

  expect(runtimeManifest).toContain('wiki-index = { path = "../wiki-index" }');
  expect(runtimeManifest).toContain('wiki-knowledge = { path = "../wiki-knowledge" }');
});

test("wiki-runtime does not re-export wiki-index or wiki-knowledge internals", () => {
  const runtimeLib = readWorkspaceFile("crates", "wiki-runtime", "src", "lib.rs");

  expect(runtimeLib).not.toMatch(/pub use\s+wiki_index::/);
  expect(runtimeLib).not.toMatch(/pub use\s+wiki_knowledge::/);
});
