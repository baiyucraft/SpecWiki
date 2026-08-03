import { existsSync, mkdirSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import os from "node:os";
import path from "node:path";

import { afterEach, expect, test } from "vitest";

import { readPackageAsset } from "../../packageRoot.js";
import {
  LEGACY_EN_V0_ASSETS,
  PROJECT_SKILL_NAMES,
  projectSkillAssetsForLanguage,
} from "./registry.js";
import { syncProjectAssets } from "./sync.js";

const roots: string[] = [];

function createTempProject(): string {
  const root = mkdtempSync(path.join(os.tmpdir(), "spec-wiki-lite-assets-"));
  roots.push(root);
  return root;
}

afterEach(() => {
  for (const root of roots.splice(0)) {
    rmSync(root, { recursive: true, force: true });
  }
});

test("preserves scaffold and user pages while force only replaces managed baselines", async () => {
  const root = createTempProject();
  await syncProjectAssets(root, { force: false });
  const custom = path.join(root, ".wiki", "custom.md");
  writeFileSync(custom, "custom", "utf8");
  const index = path.join(root, ".wiki", "INDEX.md");
  writeFileSync(index, "user-owned index", "utf8");
  const managed = path.join(root, ".wiki", "00-文档约定", "00-边界与SSOT规则.md");
  writeFileSync(managed, "locally changed baseline", "utf8");
  const codeComments = path.join(root, ".wiki", "02-开发指南", "00-代码注释规范.md");
  writeFileSync(codeComments, "locally completed code rules", "utf8");

  const preserved = await syncProjectAssets(root);
  expect(readFileSync(managed, "utf8")).toBe("locally changed baseline");
  expect(preserved.preserved).toContain(".wiki/00-文档约定/00-边界与SSOT规则.md");

  const report = await syncProjectAssets(root, { force: true });

  expect(readFileSync(custom, "utf8")).toBe("custom");
  expect(readFileSync(index, "utf8")).toBe("user-owned index");
  expect(readFileSync(codeComments, "utf8")).toBe("locally completed code rules");
  expect(readFileSync(managed, "utf8")).toContain("SpecWiki Lite managed baseline");
  expect(report.preserved).toContain(".wiki/custom.md");
  expect(report.preserved).toContain(".wiki/INDEX.md");
  expect(report.preserved).toContain(".wiki/02-开发指南/00-代码注释规范.md");
  expect(report.updated).toContain(".wiki/00-文档约定/00-边界与SSOT规则.md");
  expect(report.unchanged).toContain(".agents/skills/wiki-continue/SKILL.md");
});

test("is idempotent and restores package-owned skills", async () => {
  const root = createTempProject();
  const first = await syncProjectAssets(root);
  const second = await syncProjectAssets(root);
  const skill = path.join(root, ".agents", "skills", "wiki-continue", "SKILL.md");
  writeFileSync(skill, "stale skill", "utf8");

  const repaired = await syncProjectAssets(root);

  expect(first.created).toHaveLength(35);
  expect(second.unchanged).toHaveLength(35);
  expect(repaired.updated).toContain(".agents/skills/wiki-continue/SKILL.md");
  expect(readFileSync(skill, "utf8")).toContain("name: wiki-continue");
});

test("initializes English explicitly and migrates unchanged current assets to Chinese", async () => {
  const root = createTempProject();
  await syncProjectAssets(root, { language: "en" });
  expect(readFileSync(path.join(root, ".wiki", "config.yaml"), "utf8")).toContain("language: en");
  expect(readFileSync(path.join(root, ".wiki", "INDEX.md"), "utf8")).toContain("Wiki Bootstrap Task");
  expect(readFileSync(path.join(root, ".wiki", "01-quick-start", "INDEX.md"), "utf8")).toContain("Quick Start");
  const custom = path.join(root, ".wiki", "custom.md");
  writeFileSync(custom, "user content", "utf8");

  writeFileSync(path.join(root, ".wiki", "config.yaml"), [
    "version: 1",
    "wiki:",
    "  language: zh",
    "llm:",
    "  model: provider/model",
    "",
  ].join("\n"), "utf8");
  const report = await syncProjectAssets(root);

  expect(readFileSync(path.join(root, ".wiki", "INDEX.md"), "utf8")).toContain("Wiki 初始化任务");
  expect(readFileSync(path.join(root, ".wiki", "01-快速上手", "INDEX.md"), "utf8")).toContain("快速上手");
  expect(readFileSync(path.join(root, ".wiki", "config.yaml"), "utf8")).toContain("model: provider/model");
  expect(readFileSync(custom, "utf8")).toBe("user content");
  expect(report.preserved).toContain(".wiki/custom.md");
  expect(report.removed).toContain(".wiki/01-quick-start/INDEX.md");
  expect(() => readFileSync(path.join(root, ".wiki", "01-quick-start", "INDEX.md"), "utf8")).toThrow();
});

test("migrates unchanged current Chinese assets to English symmetrically", async () => {
  const root = createTempProject();
  await syncProjectAssets(root);
  writeFileSync(path.join(root, ".wiki", "config.yaml"), "version: 1\nwiki:\n  language: en\n", "utf8");

  const report = await syncProjectAssets(root);

  expect(readFileSync(path.join(root, ".wiki", "INDEX.md"), "utf8")).toContain("Wiki Bootstrap Task");
  expect(readFileSync(path.join(root, ".wiki", "03-module-guide", "INDEX.md"), "utf8")).toContain("Module Guide");
  expect(report.removed).toContain(".wiki/03-模块指南/INDEX.md");
  expect(() => readFileSync(path.join(root, ".wiki", "03-模块指南", "INDEX.md"), "utf8")).toThrow();
});

test("fails a language migration before writes when scaffold content was changed", async () => {
  const root = createTempProject();
  await syncProjectAssets(root, { language: "en" });
  const changed = path.join(root, ".wiki", "01-quick-start", "INDEX.md");
  writeFileSync(changed, "user-owned English quick start", "utf8");
  writeFileSync(path.join(root, ".wiki", "config.yaml"), "version: 1\nwiki:\n  language: zh\n", "utf8");
  const beforeRoot = readFileSync(path.join(root, ".wiki", "INDEX.md"), "utf8");

  await expect(syncProjectAssets(root)).rejects.toThrow("migration conflict");
  await expect(syncProjectAssets(root, { force: true })).rejects.toThrow("migration conflict");

  expect(readFileSync(changed, "utf8")).toBe("user-owned English quick start");
  expect(readFileSync(path.join(root, ".wiki", "INDEX.md"), "utf8")).toBe(beforeRoot);
  expect(() => readFileSync(path.join(root, ".wiki", "01-快速上手", "INDEX.md"), "utf8")).toThrow();
});

test("fails a language migration before writes when a target path conflicts", async () => {
  const root = createTempProject();
  await syncProjectAssets(root, { language: "en" });
  const source = path.join(root, ".wiki", "01-quick-start", "INDEX.md");
  const beforeSource = readFileSync(source, "utf8");
  const target = path.join(root, ".wiki", "01-快速上手", "INDEX.md");
  mkdirSync(path.dirname(target), { recursive: true });
  writeFileSync(target, "user-owned Chinese quick start", "utf8");
  writeFileSync(path.join(root, ".wiki", "config.yaml"), "version: 1\nwiki:\n  language: zh\n", "utf8");

  await expect(syncProjectAssets(root)).rejects.toThrow(".wiki/01-快速上手/INDEX.md");

  expect(readFileSync(source, "utf8")).toBe(beforeSource);
  expect(readFileSync(target, "utf8")).toBe("user-owned Chinese quick start");
});

test.each([
  ["invalid YAML", "version: ["],
  ["unknown version", "version: 2\nwiki:\n  language: zh\n"],
  ["invalid language", "version: 1\nwiki:\n  language: fr\n"],
])("fails before Wiki writes for %s config", async (_name, content) => {
  const root = createTempProject();
  const config = path.join(root, ".wiki", "config.yaml");
  mkdirSync(path.dirname(config), { recursive: true });
  writeFileSync(config, content, "utf8");
  const sentinel = path.join(root, ".wiki", "sentinel.md");
  writeFileSync(sentinel, "user content", "utf8");

  await expect(syncProjectAssets(root)).rejects.toThrow("invalid .wiki/config.yaml");

  expect(readFileSync(config, "utf8")).toBe(content);
  expect(readFileSync(sentinel, "utf8")).toBe("user content");
  expect(existsSync(path.join(root, ".wiki", "01-快速上手", "INDEX.md"))).toBe(false);
  expect(existsSync(path.join(root, ".agents"))).toBe(false);
});

test("rolls back localized Skill files when sync fails mid-inventory", async () => {
  const root = createTempProject();

  await expect(syncProjectAssets(root, {}, {
    beforeOperation: (_operation, index) => {
      if (index === 15) {
        throw new Error("injected Skill sync failure");
      }
    },
  })).rejects.toThrow("injected Skill sync failure");

  expect(existsSync(path.join(root, ".agents", "skills", "wiki-continue", "SKILL.md"))).toBe(false);
  expect(existsSync(path.join(root, ".agents", "skills", "wiki-plan", "references", "tasks-template.md"))).toBe(false);
  expect(existsSync(path.join(root, ".wiki", "config.yaml"))).toBe(false);
});

test("rolls back registered files when a migration operation fails", async () => {
  const root = createTempProject();
  await syncProjectAssets(root, { language: "en" });
  writeFileSync(path.join(root, ".wiki", "config.yaml"), "version: 1\nwiki:\n  language: zh\n", "utf8");
  const rootIndex = path.join(root, ".wiki", "INDEX.md");
  const quickStart = path.join(root, ".wiki", "01-quick-start", "INDEX.md");
  const beforeRoot = readFileSync(rootIndex, "utf8");
  const beforeQuickStart = readFileSync(quickStart, "utf8");

  await expect(syncProjectAssets(root, {}, {
    beforeOperation: (_operation, index) => {
      if (index === 2) {
        throw new Error("injected migration failure");
      }
    },
  })).rejects.toThrow("injected migration failure");

  expect(readFileSync(rootIndex, "utf8")).toBe(beforeRoot);
  expect(readFileSync(quickStart, "utf8")).toBe(beforeQuickStart);
  expect(existsSync(path.join(root, ".wiki", "01-快速上手", "INDEX.md"))).toBe(false);
});

test("migrates the unmodified legacy English scaffold to the default Chinese Wiki", async () => {
  const root = createTempProject();
  for (const asset of LEGACY_EN_V0_ASSETS) {
    const target = path.join(root, asset.target);
    mkdirSync(path.dirname(target), { recursive: true });
    writeFileSync(target, readPackageAsset(asset.source), "utf8");
  }

  const report = await syncProjectAssets(root);

  expect(readFileSync(path.join(root, ".wiki", "config.yaml"), "utf8")).toContain("language: zh");
  expect(readFileSync(path.join(root, ".wiki", "01-快速上手", "INDEX.md"), "utf8")).toContain("快速上手");
  expect(report.removed).toContain(".wiki/01-project/INDEX.md");
  expect(() => readFileSync(path.join(root, ".wiki", "01-project", "INDEX.md"), "utf8")).toThrow();
});

test("preserves modified legacy English pages after the user configures English", async () => {
  const root = createTempProject();
  for (const asset of LEGACY_EN_V0_ASSETS) {
    const target = path.join(root, asset.target);
    mkdirSync(path.dirname(target), { recursive: true });
    writeFileSync(target, readPackageAsset(asset.source), "utf8");
  }
  const changed = path.join(root, ".wiki", "01-project", "00-overview.md");
  writeFileSync(changed, "user-owned legacy English overview", "utf8");

  await expect(syncProjectAssets(root)).rejects.toThrow("migration conflict");
  writeFileSync(
    path.join(root, ".wiki", "config.yaml"),
    "version: 1\nwiki:\n  language: en\n",
    "utf8",
  );
  const report = await syncProjectAssets(root);

  expect(readFileSync(changed, "utf8")).toBe("user-owned legacy English overview");
  expect(existsSync(path.join(root, ".wiki", "01-quick-start", "INDEX.md"))).toBe(true);
  expect(report.preserved).toContain(".wiki/01-project/00-overview.md");
  expect(report.removed).not.toContain(".wiki/01-project/00-overview.md");
});

test("does not treat an isolated legacy path as a legacy English project", async () => {
  const root = createTempProject();
  await syncProjectAssets(root);
  const rootIndex = path.join(root, ".wiki", "INDEX.md");
  writeFileSync(rootIndex, "user-owned Chinese home", "utf8");
  const residual = path.join(root, ".wiki", "00-conventions", "INDEX.md");
  mkdirSync(path.dirname(residual), { recursive: true });
  writeFileSync(
    residual,
    readPackageAsset("migrations/wiki-en-v0/00-conventions/INDEX.md"),
    "utf8",
  );

  const report = await syncProjectAssets(root);

  expect(readFileSync(rootIndex, "utf8")).toBe("user-owned Chinese home");
  expect(readFileSync(residual, "utf8")).toContain("Documentation Conventions");
  expect(report.preserved).toContain(".wiki/00-conventions/INDEX.md");
  expect(report.removed).not.toContain(".wiki/00-conventions/INDEX.md");
});

test("installs every packaged Codex skill byte-for-byte into .agents/skills", async () => {
  const root = createTempProject();

  await syncProjectAssets(root);

  const assets = projectSkillAssetsForLanguage("zh");
  expect(assets).toHaveLength(24);
  for (const asset of assets) {
    const packaged = readPackageAsset(asset.source);
    const installed = readFileSync(path.join(root, asset.target), "utf8");
    expect(installed).toBe(packaged);
    expect(packaged).not.toMatch(/\bspec-wiki\s/u);
  }
  expect(PROJECT_SKILL_NAMES).toHaveLength(8);
});

test("switches all registered Skill files with wiki.language and preserves user additions", async () => {
  const root = createTempProject();
  await syncProjectAssets(root);
  const custom = path.join(root, ".agents", "skills", "wiki-plan", "custom.md");
  writeFileSync(custom, "user extension", "utf8");
  writeFileSync(path.join(root, ".wiki", "config.yaml"), "version: 1\nwiki:\n  language: en\n", "utf8");

  await syncProjectAssets(root);

  for (const asset of projectSkillAssetsForLanguage("en")) {
    expect(readFileSync(path.join(root, asset.target), "utf8")).toBe(readPackageAsset(asset.source));
  }
  expect(readFileSync(custom, "utf8")).toBe("user extension");
});
