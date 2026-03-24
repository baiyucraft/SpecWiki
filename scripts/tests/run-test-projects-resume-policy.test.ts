import { expect, test } from "vitest";

import { shouldResumeInitFromFailure } from "../run-test-projects.mjs";
import { isPreserveResumeEligibleInitErrorMessage } from "../testing/helpers.mjs";

test("init preserve-resume 错误分类只接受 timeout 与 provider 重试型错误", () => {
  expect(isPreserveResumeEligibleInitErrorMessage("wiki-core init timed out after 3600000ms")).toBe(true);
  expect(isPreserveResumeEligibleInitErrorMessage("provider returned 429")).toBe(true);
  expect(isPreserveResumeEligibleInitErrorMessage("provider returned 503")).toBe(true);
  expect(isPreserveResumeEligibleInitErrorMessage("error sending request for url")).toBe(true);
  expect(isPreserveResumeEligibleInitErrorMessage("error decoding response body")).toBe(true);
  expect(isPreserveResumeEligibleInitErrorMessage("os error 32")).toBe(false);
  expect(isPreserveResumeEligibleInitErrorMessage("business validation failed")).toBe(false);
});

test("run-test-projects 仅在真实 incomplete runtime 快照下放行 preserve-resume", () => {
  const runtimeSnapshot = {
    cacheDbExists: true,
    metadataExists: false,
    markdownPageCount: 0,
    runtimeState: "runtime_incomplete",
    runtimeSummary: {
      runtime_state: "researching",
      current_research_unit_id: "unit-runtime",
    },
  };

  expect(shouldResumeInitFromFailure({
    message: "wiki-core init timed out after 3600000ms",
    checkpoint: null,
    runtimeSnapshot,
    attempt: 1,
    maxAttempts: 4,
  })).toBe(true);

  expect(shouldResumeInitFromFailure({
    message: "provider returned 429",
    checkpoint: null,
    runtimeSnapshot,
    attempt: 1,
    maxAttempts: 4,
  })).toBe(false);

  expect(shouldResumeInitFromFailure({
    message: "provider returned 429",
    checkpoint: { stage: "research_unit", targetId: "unit-runtime" },
    runtimeSnapshot,
    attempt: 1,
    maxAttempts: 4,
  })).toBe(true);

  expect(shouldResumeInitFromFailure({
    message: "wiki-core init timed out after 3600000ms",
    checkpoint: null,
    runtimeSnapshot: {
      ...runtimeSnapshot,
      cacheDbExists: false,
      metadataExists: true,
      runtimeState: "ready",
      runtimeSummary: {
        runtime_state: "completed",
      },
    },
    attempt: 1,
    maxAttempts: 4,
  })).toBe(false);
});
