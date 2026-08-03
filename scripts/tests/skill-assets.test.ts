import { existsSync, readFileSync, readdirSync } from "node:fs";
import path from "node:path";

import { expect, test } from "vitest";

const root = path.resolve(import.meta.dirname, "../..");
const skillsRoot = path.join(root, "packages", "spec-wiki-lite", "assets", "skills");
const skillNames = [
  "wiki-continue",
  "wiki-explore",
  "wiki-propose",
  "wiki-design",
  "wiki-plan",
  "wiki-apply",
  "wiki-review",
  "wiki-archive",
] as const;

function markdownFiles(directory: string): string[] {
  return readdirSync(directory, { recursive: true, withFileTypes: true })
    .filter(entry => entry.isFile() && entry.name.endsWith(".md"))
    .map(entry => path.join(entry.parentPath, entry.name));
}

test.each(["zh", "en"] as const)("ships a closed %s Skill and reference tree", (language) => {
  const localeRoot = path.join(skillsRoot, language);
  const files = markdownFiles(localeRoot);
  expect(files).toHaveLength(24);
  expect(readdirSync(localeRoot).sort()).toEqual([...skillNames].sort());

  for (const name of skillNames) {
    const skillRoot = path.join(localeRoot, name);
    const skillFile = path.join(skillRoot, "SKILL.md");
    const content = readFileSync(skillFile, "utf8");
    expect(content).toMatch(new RegExp(`^---\\nname: ${name}\\n`, "u"));
    expect(language === "zh" ? /[\u3400-\u9FFF]/u.test(content) : !/[\u3400-\u9FFF]/u.test(content)).toBe(true);

    for (const file of markdownFiles(skillRoot)) {
      const markdown = readFileSync(file, "utf8");
      for (const match of markdown.matchAll(/references\/([a-z0-9.-]+\.md)/gu)) {
        expect(existsSync(path.join(skillRoot, "references", match[1]!)), `${file}: ${match[0]}`).toBe(true);
      }
    }
  }
});

test("keeps Lite Skills free of incompatible workflow contracts", () => {
  const content = markdownFiles(skillsRoot).map(file => readFileSync(file, "utf8")).join("\n");
  expect(content).not.toMatch(/\bunispec\s+(?:status|show|validate|archive|init)\b/iu);
  expect(content).not.toMatch(/CodeBuddy|\.codebuddy\/|\.claude\/skills/iu);
  expect(content).not.toMatch(/frontend-interaction-standard/iu);
  expect(content).not.toMatch(/generate(?:d|s|ing)?\s+agents?/iu);

  const zhBrowser = readFileSync(path.join(skillsRoot, "zh", "wiki-plan", "references", "browser-automation.md"), "utf8");
  const enBrowser = readFileSync(path.join(skillsRoot, "en", "wiki-plan", "references", "browser-automation.md"), "utf8");
  expect(zhBrowser).toContain("可选验证方式");
  expect(enBrowser).toContain("optional evidence");
  expect(`${zhBrowser}\n${enBrowser}`).not.toMatch(/(?:must|required to)\s+(?:use|run)\s+Playwright/iu);
  expect(`${zhBrowser}\n${enBrowser}`).not.toMatch(/Lite (?:includes|provides|ships) (?:a )?browser runner/iu);
});
