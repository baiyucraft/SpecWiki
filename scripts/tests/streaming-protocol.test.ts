import { mkdtempSync, rmSync, writeFileSync } from "node:fs";
import os from "node:os";
import path from "node:path";
import { spawnSync } from "node:child_process";
import { fileURLToPath } from "node:url";

import { expect, test } from "vitest";

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);
const rootDir = path.resolve(__dirname, "..", "..");
const e2eTargetDir = path.join(rootDir, "target", "e2e");

test("built wiki-core streams NDJSON progress for init", async () => {
  const repoRoot = mkdtempSync(path.join(os.tmpdir(), "wiki-core-stream-"));

  try {
    writeFileSync(path.join(repoRoot, "package.json"), JSON.stringify({ name: "stream-demo" }));
    writeFileSync(path.join(repoRoot, "src.ts"), "export const version = 1;\n");

    const { resolveBuiltBinary } = await import("../build/core-paths.mjs");
    const binaryPath = resolveBuiltBinary(rootDir, { targetDir: e2eTargetDir });
    const command = JSON.stringify({
      action: "init",
      repoRoot,
    });

    const result = spawnSync(binaryPath, ["--json"], {
      input: command,
      encoding: "utf8",
    });

    expect(result.status).toBe(0);
    const lines = result.stdout
      .trim()
      .split(/\r?\n/)
      .filter((line) => line.length > 0)
      .map((line) => JSON.parse(line));

    expect(lines.length).toBeGreaterThan(1);
    expect(lines.some((event) => event.type === "progress")).toBe(true);
    expect(lines.at(-1)?.type).toBe("result");
    expect(lines.at(-1)?.response?.ok).toBe(true);
  } finally {
    rmSync(repoRoot, { recursive: true, force: true });
  }
});
