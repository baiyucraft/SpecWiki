import { existsSync, readFileSync } from "node:fs";
import path from "node:path";

import { expect, test } from "vitest";

const root = path.resolve(import.meta.dirname, "../..");
const locales = ["zh", "en"] as const;
const skills = ["wiki-plan", "wiki-continue", "wiki-apply"] as const;

function readPackageSkill(locale: string, name: string): string {
  return readFileSync(path.join(root, "packages", "spec-wiki-lite", "assets", "skills", locale, name, "SKILL.md"), "utf8");
}

test.each(locales)("%s workflow makes TDD optional and removes readiness confirmation", (locale) => {
  const plan = readPackageSkill(locale, "wiki-plan");
  const continuation = readPackageSkill(locale, "wiki-continue");
  const apply = readPackageSkill(locale, "wiki-apply");
  const tasks = readFileSync(path.join(root, "packages", "spec-wiki-lite", "assets", "skills", locale, "wiki-plan", "references", "tasks-template.md"), "utf8");

  expect(tasks).toMatch(/implementation-mode:\s*<tdd\|direct>/u);
  expect(tasks).toMatch(/tdd[\s\S]*direct|direct[\s\S]*tdd/u);
  expect(tasks).toMatch(/任务总览|Task Overview/iu);
  expect(tasks).toMatch(/##\s+1\.|##\s+1\s/u);
  expect(tasks).toMatch(/###\s+CheckList/iu);
  expect(tasks).toMatch(/用例到任务映射|Case-to-task mapping/iu);
  expect(tasks).toMatch(/执行顺序|Execution order/iu);
  expect(tasks).toMatch(/暂缓事项|Deferred items/iu);
  expect(tasks).toMatch(/direct[\s\S]*(模块|文件|接口|配置|数据流)|direct[\s\S]*(module|file|interface|config|data flow)/iu);
  expect(tasks).not.toContain("implementation-ready");
  expect(plan).toMatch(/tdd[\s\S]*(direct|选择|choose)|direct[\s\S]*(tdd|选择|choose)/iu);
  expect(plan).toMatch(/明确时不重复询问|明确不重复询问|不得重复询问|already specified|do not ask again|without asking/iu);
  expect(plan).not.toMatch(/implementation-ready/u);
  expect(continuation).toMatch(/tdd.*direct|direct.*tdd/isu);
  expect(continuation).toMatch(/合法.*apply|valid.*apply/iu);
  expect(continuation).toMatch(/缺失.*plan|invalid.*plan/iu);
  expect(continuation).not.toMatch(/implementation-ready/u);
  expect(apply).toMatch(/tdd.*red.*green.*refactor|red.*green.*refactor.*tdd/isu);
  expect(apply).toMatch(/direct.*implement.*verify.*refactor|implement.*verify.*refactor.*direct/isu);
  expect(apply).toMatch(/授权|authorized/iu);
  expect(apply).not.toMatch(/implementation-ready/u);
});

test("current Codex Skills are synced to the same contract", () => {
  for (const name of skills) {
    const installed = path.join(root, ".agents", "skills", name, "SKILL.md");
    expect(existsSync(installed), installed).toBe(true);
    expect(readFileSync(installed, "utf8")).not.toMatch(/implementation-ready/u);
  }
});

test("authorization and safety wording remains after readiness removal", () => {
  const content = [
    readPackageSkill("zh", "wiki-apply"),
    readPackageSkill("en", "wiki-apply"),
  ].join("\n");
  expect(content).toMatch(/授权|authorized/iu);
  expect(content).toMatch(/strict validate|strict validation/iu);
  expect(content).toMatch(/scope drift/iu);
  expect(content).toMatch(/path safety|路径安全/iu);
});

test("current surface documentation states that implementation mode is explicit", () => {
  const docs = [
    readFileSync(path.join(root, "README.md"), "utf8"),
    readFileSync(path.join(root, ".wiki", "06-设计文档", "02-Agents设计.md"), "utf8"),
  ].join("\n");
  expect(docs).toMatch(/implementation-mode|实现模式/iu);
  expect(docs).toMatch(/tdd.*direct|direct.*tdd/isu);
  expect(docs).not.toMatch(/implementation-ready/u);
});
