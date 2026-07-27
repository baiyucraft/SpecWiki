import { mkdirSync, mkdtempSync, rmSync, symlinkSync, writeFileSync } from "node:fs";
import os from "node:os";
import path from "node:path";

import { afterEach, expect, test } from "vitest";

import { resolveSafePath } from "./path.js";

const roots: string[] = [];

function createRoot(prefix: string): string {
  const root = mkdtempSync(path.join(os.tmpdir(), prefix));
  roots.push(root);
  return root;
}

afterEach(() => {
  for (const root of roots.splice(0)) {
    rmSync(root, { recursive: true, force: true });
  }
});

test.each(["../escape", "/absolute", "C:\\escape", "wiki/../../escape", "\\\\server\\share"])(
  "rejects unsafe identifier or path %s",
  (input) => {
    const root = createRoot("spec-wiki-lite-path-");
    expect(() => resolveSafePath(root, input)).toThrow("unsafe path");
  },
);

test("rejects an existing symlink or junction that escapes the project", () => {
  const root = createRoot("spec-wiki-lite-path-");
  const outside = createRoot("spec-wiki-lite-outside-");
  writeFileSync(path.join(outside, "secret.md"), "outside", "utf8");
  const inside = path.join(root, ".spec", "changes");
  mkdirSync(inside, { recursive: true });
  symlinkSync(outside, path.join(inside, "escaped"), "junction");

  expect(() => resolveSafePath(root, ".spec/changes/escaped/secret.md")).toThrow("unsafe path");
});
