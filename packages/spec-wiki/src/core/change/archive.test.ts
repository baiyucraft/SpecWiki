import { existsSync, mkdirSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import os from "node:os";
import path from "node:path";

import { afterEach, expect, test } from "vitest";

import { parse } from "yaml";

import { archiveChange } from "./archive.js";

const roots: string[] = [];
const fixedClock = () => new Date("2026-07-27T12:00:00.000Z");

function createValidChange(root: string, id = "document-api"): string {
  const changeRoot = path.join(root, ".spec", "changes", id);
  mkdirSync(changeRoot, { recursive: true });
  mkdirSync(path.join(root, ".spec", "archive"), { recursive: true });
  writeFileSync(path.join(changeRoot, "meta.yaml"), [
    `id: ${id}`,
    "stage: verification",
    "deliveryShape: single-change",
    "futureField: preserved",
  ].join("\n"), "utf8");
  for (const file of ["proposal.md", "design.md", "system-tests.md", "tasks.md"]) {
    writeFileSync(path.join(changeRoot, file), `# ${file}\n`, "utf8");
  }
  writeFileSync(path.join(changeRoot, "review-report.md"), [
    "---",
    "review-result: pass",
    "scope: full",
    "---",
    "",
    "# Review",
  ].join("\n"), "utf8");
  writeFileSync(path.join(changeRoot, "test-report.md"), [
    "---",
    "verification-result: pass",
    "scope: full",
    "---",
    "",
    "# Test",
  ].join("\n"), "utf8");
  return changeRoot;
}

function createParentChildFixture(root: string): {
  childId: string;
  childRoot: string;
  parentId: string;
  parentRoot: string;
} {
  const parentId = "document-suite";
  const childId = "document-suite-api";
  const parentRoot = path.join(root, ".spec", "changes", parentId);
  mkdirSync(parentRoot, { recursive: true });
  writeFileSync(path.join(parentRoot, "meta.yaml"), [
    "# keep this comment",
    `id: ${parentId}`,
    "stage: exploration",
    "deliveryShape: multi-change",
    "futureField: preserved",
    "multiChange:",
    "  role: parent",
    "  children:",
    `    - id: ${childId}`,
    "      order: 1",
    "      dependsOn: []",
    "      futureChildField: preserved",
  ].join("\n"), "utf8");
  writeFileSync(path.join(parentRoot, "split.md"), [
    "# Split",
    "",
    `### 1. ${childId}`,
    "",
    "- 归档状态：[ ] pending",
  ].join("\n"), "utf8");
  const childRoot = createValidChange(root, childId);
  writeFileSync(path.join(childRoot, "meta.yaml"), [
    `id: ${childId}`,
    "stage: verification",
    "deliveryShape: single-change",
    "multiChange:",
    "  role: child",
    `  parent: ${parentId}`,
    "  order: 1",
    "  dependsOn: []",
  ].join("\n"), "utf8");
  return { childId, childRoot, parentId, parentRoot };
}

afterEach(() => {
  for (const root of roots.splice(0)) {
    rmSync(root, { recursive: true, force: true });
  }
});

test("moves a valid change once and refuses an existing target", async () => {
  const root = mkdtempSync(path.join(os.tmpdir(), "spec-wiki-lite-archive-"));
  roots.push(root);
  const source = createValidChange(root);

  const first = await archiveChange(root, "document-api", fixedClock);

  expect(first.archivedTo).toBe(".spec/archive/2026-07-27-document-api");
  expect(existsSync(source)).toBe(false);
  expect(readFileSync(path.join(root, first.archivedTo, "meta.yaml"), "utf8")).toContain("futureField: preserved");

  createValidChange(root);
  await expect(archiveChange(root, "document-api", fixedClock)).rejects.toThrow("already exists");
  expect(existsSync(source)).toBe(true);
});

test("archives a child and records the same target in its active parent", async () => {
  const root = mkdtempSync(path.join(os.tmpdir(), "spec-wiki-lite-archive-"));
  roots.push(root);
  const { childId, parentRoot } = createParentChildFixture(root);

  const report = await archiveChange(root, childId, fixedClock);
  const parentText = readFileSync(path.join(parentRoot, "meta.yaml"), "utf8");
  const parent = parse(parentText) as { multiChange: { children: Array<Record<string, unknown>> } };
  const recorded = parent.multiChange.children[0];

  expect(recorded.archiveStatus).toBe("archived");
  expect(recorded.archivedAt).toBe(report.archivedAt);
  expect(recorded.archivedTo).toBe(report.archivedTo);
  expect(parentText).toContain("# keep this comment");
  expect(parentText).toContain("futureChildField: preserved");
  expect(readFileSync(path.join(parentRoot, "split.md"), "utf8")).toContain("- 归档状态：[x] archived");
});

test("blocks parent archive until every child is archived", async () => {
  const root = mkdtempSync(path.join(os.tmpdir(), "spec-wiki-lite-archive-"));
  roots.push(root);
  const { childId, parentId, parentRoot } = createParentChildFixture(root);

  await expect(archiveChange(root, parentId, fixedClock)).rejects.toThrow("incomplete");
  expect(existsSync(parentRoot)).toBe(true);

  await archiveChange(root, childId, fixedClock);
  const parentReport = await archiveChange(root, parentId, fixedClock);

  expect(parentReport.archivedTo).toBe(".spec/archive/2026-07-27-document-suite");
  expect(existsSync(parentRoot)).toBe(false);
});

test("rolls back the child and parent when the second parent write fails", async () => {
  const root = mkdtempSync(path.join(os.tmpdir(), "spec-wiki-lite-archive-"));
  roots.push(root);
  const { childId, childRoot, parentRoot } = createParentChildFixture(root);
  const metaPath = path.join(parentRoot, "meta.yaml");
  const splitPath = path.join(parentRoot, "split.md");
  const originalMeta = readFileSync(metaPath, "utf8");
  const originalSplit = readFileSync(splitPath, "utf8");

  await expect(archiveChange(root, childId, fixedClock, {
    writeParentFile(filePath, content) {
      if (filePath === splitPath) {
        throw new Error("injected split write failure");
      }
      writeFileSync(filePath, content, "utf8");
    },
  })).rejects.toThrow("injected split write failure");

  expect(existsSync(childRoot)).toBe(true);
  expect(existsSync(path.join(root, ".spec", "archive", `2026-07-27-${childId}`))).toBe(false);
  expect(readFileSync(metaPath, "utf8")).toBe(originalMeta);
  expect(readFileSync(splitPath, "utf8")).toBe(originalSplit);
});
