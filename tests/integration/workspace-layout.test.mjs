import test from "node:test";
import assert from "node:assert/strict";
import { existsSync } from "node:fs";

const requiredPaths = [
  "Cargo.toml",
  "package.json",
  "pnpm-workspace.yaml",
  "README.md",
  "crates/wiki-core/Cargo.toml",
  "crates/wiki-core/src/lib.rs",
  "crates/wiki-core/src/main.rs",
  "packages/codebuddy/package.json",
  "packages/codebuddy/tsconfig.json",
  "packages/codebuddy/src/index.ts",
  "packages/codebuddy/bin/codebuddy.js",
];

test("workspace skeleton files exist", () => {
  for (const path of requiredPaths) {
    assert.equal(existsSync(path), true, `${path} should exist`);
  }
});
