import { existsSync, readFileSync } from "node:fs";

import { expect, test } from "vitest";

import { evaluateHostTrigger } from "./triggerContract.js";

const parserPath = new URL("./triggerCorpus.ts", import.meta.url);
const fixturePath = new URL("../../../tests/fixtures/host-trigger-corpus.v1.json", import.meta.url);

async function loadParser() {
  return import(parserPath.href) as Promise<typeof import("./triggerCorpus.js")>;
}

test("validates v1 corpus and applies every semantic case", async () => {
  expect(existsSync(parserPath), "host trigger corpus parser must exist").toBe(true);
  const { parseHostTriggerCorpus } = await loadParser();
  const corpus = parseHostTriggerCorpus(JSON.parse(readFileSync(fixturePath, "utf8")));

  expect(corpus.semanticCases.length).toBeGreaterThanOrEqual(20);
  expect(new Set(corpus.semanticCases.map(testCase => testCase.locale))).toEqual(new Set(["zh-CN", "en"]));
  for (const testCase of corpus.semanticCases) {
    expect(evaluateHostTrigger(testCase.input), testCase.id).toMatchObject(testCase.expected);
  }
});

test("rejects unsupported versions and duplicate case ids", async () => {
  expect(existsSync(parserPath), "host trigger corpus parser must exist").toBe(true);
  const { parseHostTriggerCorpus } = await loadParser();
  const fixture = JSON.parse(readFileSync(fixturePath, "utf8"));

  expect(() => parseHostTriggerCorpus({ ...fixture, contractVersion: "host-trigger/v2" }))
    .toThrow(/contractVersion/);
  expect(() => parseHostTriggerCorpus({
    ...fixture,
    adapterCases: [{ ...fixture.adapterCases[0], id: fixture.semanticCases[0].id }],
  })).toThrow(/duplicate/);
  expect(() => parseHostTriggerCorpus({
    ...fixture,
    semanticCases: [{
      ...fixture.semanticCases[0],
      expected: { decision: "should_trigger", targetAction: null, reason: "semantic_query_request" },
    }],
  })).toThrow(/targetAction/);
});
