import { existsSync, readFileSync, readdirSync } from "node:fs";
import path from "node:path";

import { expect, test } from "vitest";

const root = path.resolve(import.meta.dirname, "../..");

function readTextTree(relative: string): string {
  const target = path.join(root, relative);
  if (!existsSync(target)) {
    return "";
  }
  const entries = readdirSync(target, { recursive: true, withFileTypes: true });
  return entries
    .filter(entry => entry.isFile() && /\.(?:md|ts)$/u.test(entry.name))
    .map(entry => readFileSync(path.join(entry.parentPath, entry.name), "utf8"))
    .join("\n");
}

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

test("current documentation and Skills expose only the SpecWiki Lite contract", () => {
  const currentSurface = [
    readFileSync(path.join(root, "README.md"), "utf8"),
    readFileSync(path.join(root, "README-CN.md"), "utf8"),
    readFileSync(path.join(root, "AGENTS.md"), "utf8"),
    readTextTree(".wiki"),
    readTextTree(".agents/skills/wiki-continue"),
    readTextTree(".agents/skills/wiki-explore"),
    readTextTree(".agents/skills/wiki-propose"),
    readTextTree(".agents/skills/wiki-design"),
    readTextTree(".agents/skills/wiki-plan"),
    readTextTree(".agents/skills/wiki-apply"),
    readTextTree(".agents/skills/wiki-review"),
    readTextTree(".agents/skills/wiki-archive"),
    readTextTree("packages/spec-wiki-lite/assets/skills"),
  ].join("\n");

  expect(currentSurface).not.toMatch(/\bspec-wiki\s+(?:init|status|query|sync|rebuild|update|changes?|validate|archive)\b/u);
  expect(currentSurface).not.toMatch(/wiki-index|KnowledgeUnit|route_groups|SQLite|spec-wiki@0\.2\.0/u);
  expect(currentSurface).not.toMatch(/\.wiki\/(?:\.knowledge|\.cache|pages)(?:\/|\*\*)|wiki\.metadata\.json/u);
  expect(currentSurface).not.toMatch(/CodeBuddy|\.codebuddy\/|\.claude\/skills/u);
});
