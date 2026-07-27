import { mkdirSync, mkdtempSync, rmSync, writeFileSync } from "node:fs";
import os from "node:os";
import path from "node:path";

import { afterEach, expect, test } from "vitest";

import { showChange } from "./show.js";
import { getChangeStatus } from "./status.js";
import { validateChange } from "./validate.js";

const roots: string[] = [];
const artifactFiles: Record<string, string> = {
  "proposal": "proposal.md",
  "design": "design.md",
  "cases": "system-tests.md",
  "tasks": "tasks.md",
  "review-report": "review-report.md",
  "test-report": "test-report.md",
  "metadata": "meta.yaml",
};

function expectedArtifactsFor(stage: string): string[] {
  const order = ["proposal", "design", "cases", "tasks", "review-report", "test-report"];
  const count: Record<string, number> = {
    exploration: 0,
    proposal: 1,
    delivery: 1,
    design: 2,
    cases: 3,
    tasks: 4,
    implementation: 4,
    review: 4,
    verification: 6,
    archive: 6,
  };
  return [...order.slice(0, count[stage]), "metadata"];
}

function createStageFixture(stage: string, deliveryShape = "single-change"): { id: string; root: string } {
  const root = mkdtempSync(path.join(os.tmpdir(), "spec-wiki-lite-change-"));
  roots.push(root);
  const id = "document-api";
  const changeRoot = path.join(root, ".spec", "changes", id);
  mkdirSync(changeRoot, { recursive: true });
  writeFileSync(path.join(changeRoot, "meta.yaml"), [
    `id: ${id}`,
    `stage: ${stage}`,
    `deliveryShape: ${deliveryShape}`,
  ].join("\n"), "utf8");
  for (const artifact of expectedArtifactsFor(stage).filter(artifact => artifact !== "metadata")) {
    writeFileSync(path.join(changeRoot, artifactFiles[artifact]), `# ${artifact}\n`, "utf8");
  }
  return { id, root };
}

afterEach(() => {
  for (const root of roots.splice(0)) {
    rmSync(root, { recursive: true, force: true });
  }
});

test.each(["proposal", "design", "cases", "tasks", "implementation", "review", "verification", "archive"])(
  "requires the artifact prefix for %s",
  async (stage) => {
    const fixture = createStageFixture(stage);
    const result = await validateChange(fixture.root, fixture.id);
    expect(result.requiredArtifacts).toEqual(expectedArtifactsFor(stage));
  },
);

test("rejects unknown stage and delivery shape", async () => {
  const fixture = createStageFixture("unknown", "unknown-shape");
  const result = await validateChange(fixture.root, fixture.id);
  expect(result.valid).toBe(false);
  expect(result.issues.map(issue => issue.kind)).toEqual(expect.arrayContaining([
    "invalid_stage",
    "invalid_delivery_shape",
  ]));
});

test("status, show, and validate share the artifact registry", async () => {
  const fixture = createStageFixture("tasks");
  const changeRoot = path.join(fixture.root, ".spec", "changes", fixture.id);
  rmSync(path.join(changeRoot, "tasks.md"));

  const validation = await validateChange(fixture.root, fixture.id);
  const status = await getChangeStatus(fixture.root);
  const shown = await showChange(fixture.root, fixture.id, "proposal");

  expect(validation.valid).toBe(false);
  expect(validation.issues).toContainEqual(expect.objectContaining({ kind: "missing_artifact" }));
  expect(status.activeChanges[0].requiredArtifacts).toEqual(validation.requiredArtifacts);
  expect(shown.artifact?.content).toBe("# proposal\n");
  await expect(showChange(fixture.root, fixture.id, "../../outside")).rejects.toThrow("unknown artifact");
});

test("blocks verification unless both reports are full pass", async () => {
  const fixture = createStageFixture("verification");
  const changeRoot = path.join(fixture.root, ".spec", "changes", fixture.id);
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
    "verification-result: skipped",
    "scope: full",
    "---",
    "",
    "# Verification",
  ].join("\n"), "utf8");

  const result = await validateChange(fixture.root, fixture.id);

  expect(result.valid).toBe(false);
  expect(result.issues).toContainEqual(expect.objectContaining({ kind: "verification_not_passed" }));
});

test("accepts BOM and CRLF only when both reports are full pass", async () => {
  const fixture = createStageFixture("verification");
  const changeRoot = path.join(fixture.root, ".spec", "changes", fixture.id);
  writeFileSync(
    path.join(changeRoot, "review-report.md"),
    "\uFEFF---\r\nreview-result: pass\r\nscope: full\r\n---\r\n\r\n# Review\r\n",
    "utf8",
  );
  writeFileSync(
    path.join(changeRoot, "test-report.md"),
    "---\r\nverification-result: pass\r\nscope: full\r\n---",
    "utf8",
  );

  const result = await validateChange(fixture.root, fixture.id);

  expect(result.valid).toBe(true);
  expect(result.issues).toEqual([]);
});

test("strict validation rejects empty required artifacts", async () => {
  const fixture = createStageFixture("tasks");
  const design = path.join(fixture.root, ".spec", "changes", fixture.id, "design.md");
  writeFileSync(design, "", "utf8");

  expect((await validateChange(fixture.root, fixture.id)).valid).toBe(true);
  const strict = await validateChange(fixture.root, fixture.id, { strict: true });
  expect(strict.valid).toBe(false);
  expect(strict.issues).toContainEqual(expect.objectContaining({ kind: "empty_artifact" }));
});

test("parent changes require only split and metadata", async () => {
  const fixture = createStageFixture("verification", "multi-change");
  const changeRoot = path.join(fixture.root, ".spec", "changes", fixture.id);
  writeFileSync(path.join(changeRoot, "meta.yaml"), [
    `id: ${fixture.id}`,
    "stage: verification",
    "deliveryShape: multi-change",
    "multiChange:",
    "  role: parent",
    "  children:",
    "    - id: document-api-child",
    "      order: 1",
  ].join("\n"), "utf8");
  writeFileSync(path.join(changeRoot, "split.md"), "# Split\n", "utf8");

  const result = await validateChange(fixture.root, fixture.id);

  expect(result.requiredArtifacts).toEqual(["split", "metadata"]);
  expect(result.valid).toBe(true);
});
