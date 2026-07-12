import { readFileSync, readdirSync, statSync } from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

import { expect, test } from "vitest";

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..", "..");
const excluded = new Set([".git", ".upstream", "dist", "node_modules", "target"]);
const scanRoots = [
  path.join(root, "README.md"),
  path.join(root, "README-CN.md"),
  path.join(root, ".wiki", "04-对外方法"),
  path.join(root, ".wiki", "05-规格基线"),
  path.join(root, "packages", "spec-wiki", "src"),
  path.join(root, "scripts"),
];
const forbidden = [
  ["spec-wiki", "wiki"].join(" "),
  ["spec-wiki", "governance"].join(" "),
  ["spec-wiki", "init", "--tool"].join(" "),
  ["spec-wiki", "init", "--tools"].join(" "),
];

function currentProductFiles(directory: string): string[] {
  if (statSync(directory).isFile())
return [directory];
  const relative = path.relative(root, directory).replaceAll("\\", "/");
  if (relative.startsWith(".spec/archive/") || relative.startsWith(".docs/release"))
return [];
  return readdirSync(directory, { withFileTypes: true }).flatMap((entry) => {
    if (excluded.has(entry.name))
return [];
    const fullPath = path.join(directory, entry.name);
    if (entry.isDirectory())
return currentProductFiles(fullPath);
    if (!entry.isFile() || entry.name.endsWith(".test.ts") || statSync(fullPath).size > 1_000_000)
return [];
    return [fullPath];
  });
}

test("current product surface contains no legacy CLI invocations", () => {
  const violations = scanRoots.flatMap(currentProductFiles).flatMap((file) => {
    const content = readFileSync(file, "utf8");
    return forbidden
      .filter((pattern) => content.includes(pattern))
      .map((pattern) => `${path.relative(root, file)}: ${pattern}`);
  });

  expect(violations).toEqual([]);
});
