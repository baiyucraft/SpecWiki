import { createHash } from "node:crypto";
import { mkdirSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import os from "node:os";
import path from "node:path";

import { afterEach, expect, test } from "vitest";

import { installAociRelease } from "./installer.js";

const roots: string[] = [];

afterEach(() => {
  for (const root of roots.splice(0)) rmSync(root, { recursive: true, force: true });
});

test("verifies bytes before publishing an AOCI executable", async () => {
  const root = mkdtempSync(path.join(os.tmpdir(), "spec-wiki-aoci-install-"));
  roots.push(root);
  const bytes = Buffer.from("verified archive");
  const sha256 = createHash("sha256").update(bytes).digest("hex");
  const executed: string[] = [];
  const result = await installAociRelease({
    installRoot: root,
    asset: { archive: "fixture.zip", sha256, executable: "aoci.exe", url: "https://example.invalid/fixture.zip" },
    version: "test",
    download: async () => bytes,
    extract: async (_archive, destination) => {
      mkdirSync(destination, { recursive: true });
      writeFileSync(path.join(destination, "aoci.exe"), "fixture", "utf8");
    },
    verifyExecutable: async executable => { executed.push(executable); return "aoci version test"; },
  });
  expect(result.installed).toBe(true);
  expect(readFileSync(result.executablePath, "utf8")).toBe("fixture");
  expect(executed).toHaveLength(1);
  expect(executed[0]).toMatch(/[\\/]extracted[\\/]aoci\.exe$/u);
  expect(executed[0]).not.toBe(result.executablePath);
});

test("rejects a mismatched archive without executing it", async () => {
  const root = mkdtempSync(path.join(os.tmpdir(), "spec-wiki-aoci-hash-"));
  roots.push(root);
  let executed = false;
  await expect(installAociRelease({
    installRoot: root,
    asset: { archive: "fixture.zip", sha256: "0".repeat(64), executable: "aoci.exe", url: "https://example.invalid/fixture.zip" },
    version: "test",
    download: async () => Buffer.from("wrong"),
    extract: async () => {},
    verifyExecutable: async () => { executed = true; return ""; },
  })).rejects.toThrow(/checksum/iu);
  expect(executed).toBe(false);
});
