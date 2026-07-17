import { expect, test } from "vitest";

import { HOSTS } from "./hosts.js";

test("declares Codex as the only reference host with complete trigger capabilities", () => {
  expect(HOSTS.filter(host => host.compatibilityRole === "reference").map(host => host.id))
    .toEqual(["codex"]);
  expect(HOSTS).toMatchObject([
    {
      id: "codex",
      compatibilityRole: "reference",
      triggerCapabilities: {
        nativeSkillDiscovery: "supported",
        promptInspection: "none",
        sessionStartContext: "none",
        settingsIntegration: "none",
        deterministicDelivery: "skill_guidance",
      },
    },
    {
      id: "claude",
      compatibilityRole: "compatible",
      triggerCapabilities: {
        nativeSkillDiscovery: "supported",
        promptInspection: "none",
        sessionStartContext: "none",
        settingsIntegration: "none",
        deterministicDelivery: "skill_guidance",
      },
    },
    {
      id: "codebuddy",
      compatibilityRole: "compatible",
      triggerCapabilities: {
        nativeSkillDiscovery: "supported",
        promptInspection: "user_prompt_submit",
        sessionStartContext: "orientation",
        settingsIntegration: "managed_settings",
        deterministicDelivery: "generated_hook",
      },
    },
  ]);
});
