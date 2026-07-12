import { expect, test } from "vitest";

import { renderHumanResponse } from "./humanRenderer.js";

test("human renderer relays DTO readiness and recommended action", () => {
  const text = renderHumanResponse({ ok: true, data: { state: "stale", recommended_action: "update" } }, { action: "status" });
  expect(text).toContain("state: stale");
  expect(text).toContain("recommended_action: update");
});
