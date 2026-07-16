import { readFileSync } from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

import { expect, test } from "vitest";

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..", "..");
const paths = {
  runtime: ".wiki/06-设计文档/01-Runtime设计.md",
  extensions: ".wiki/06-设计文档/04-扩展场景.md",
  query: ".wiki/06-设计文档/06-Runtime查询合同.md",
  health: ".wiki/05-规格基线/capabilities/knowledge-runtime-health-signals/spec.md",
  declared: ".wiki/05-规格基线/capabilities/declared-knowledge-lifecycle/spec.md",
  decomposition: ".wiki/05-规格基线/capabilities/knowledge-unit-decomposition/spec.md",
};

function read(relativePath: string): string {
  return readFileSync(path.join(root, relativePath), "utf8");
}

test("reliability lifecycle authority classifies A1-A10 and states honest boundaries", () => {
  const runtime = read(paths.runtime);
  const extensions = read(paths.extensions);
  const query = read(paths.query);
  const capabilities = [read(paths.health), read(paths.declared), read(paths.decomposition)].join("\n");
  const currentAuthority = [runtime, extensions, query, capabilities].join("\n");

  const classification = Object.fromEntries(
    [...extensions.matchAll(/^scenario\.(A\d+) = (baseline|next|non-goal)$/gm)]
      .map(match => [match[1], match[2]]),
  );
  expect(classification).toEqual({
    A1: "baseline",
    A2: "next",
    A3: "next",
    A4: "baseline",
    A5: "next",
    A6: "baseline",
    A7: "non-goal",
    A8: "baseline",
    A9: "baseline",
    A10: "next",
  });

  for (const marker of [
    "provider_session.scope = request_local",
    "provider_session.persistence = forbidden",
    "compose_resume.granularity = knowledge_unit",
    "compose_resume.current_unit = restart",
    "decomposition.kind = deterministic_heuristic",
    "decomposition.generic_typed_surface = not_complete",
    "large_repository.full_compose = bounded_fixture_only",
  ]) {
    expect(currentAuthority).toContain(marker);
  }

  const a4 = extensions.match(/## 6\. 场景 A4：[\s\S]*?(?=\n## 7\.)/)?.[0] ?? "";
  expect(a4).toContain("[06-Runtime查询合同](./06-Runtime查询合同.md)");
  expect(a4).not.toMatch(/route_groups|QueryResponse|ranking_basis|score_direction/);

  expect(currentAuthority).not.toMatch(/durable provider session/i);
  expect(currentAuthority).not.toMatch(/generic typed surface.{1,20}\bcomplete\b/i);
  expect(currentAuthority).not.toMatch(/(?:已完成|已稳定完成).{0,20}任意大仓|任意大仓.{0,20}(?:已完成|已稳定完成)/);
});
