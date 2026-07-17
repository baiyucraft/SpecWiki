import { expect, test } from "vitest";

import { getHostDefinition, HOSTS } from "./hosts.js";

const modulePath = new URL("./hostAssets.ts", import.meta.url);

test("validates registry roles and built assets before returning a host plan", async () => {
  const hostAssets: typeof import("./hostAssets.js") = await import(modulePath.href);
  expect(typeof hostAssets.validateHostRegistry).toBe("function");
  expect(typeof hostAssets.validateHostAssetsAgainstCapabilities).toBe("function");

  expect(() => hostAssets.validateHostRegistry(HOSTS)).not.toThrow();
  for (const host of HOSTS) {
    expect(() => hostAssets.buildHostBootstrapAssets("E:\\demo", process.env, host.id)).not.toThrow();
  }

  const codeBuddy = getHostDefinition("codebuddy");
  const skillsOnly = hostAssets.buildHostBootstrapAssets("E:\\demo", process.env, "codex");
  expect(() => hostAssets.validateHostAssetsAgainstCapabilities(codeBuddy, skillsOnly))
    .toThrow(/UserPromptSubmit|hook/);

  expect(() => hostAssets.validateHostRegistry(HOSTS.map(host => ({
    ...host,
    compatibilityRole: "compatible",
  })))).toThrow(/only reference/);

  const codeBuddyAssets = hostAssets.buildHostBootstrapAssets("E:\\demo", process.env, "codebuddy");
  expect(() => hostAssets.validateHostAssetsAgainstCapabilities(
    codeBuddy,
    codeBuddyAssets.filter(asset => !asset.filePath.includes("wiki-query")),
  )).toThrow(/wiki-query/);
  expect(() => hostAssets.validateHostAssetsAgainstCapabilities(
    codeBuddy,
    codeBuddyAssets.filter(asset => asset.kind !== "settings"),
  )).toThrow(/settings/);
  expect(() => hostAssets.validateHostAssetsAgainstCapabilities(
    codeBuddy,
    codeBuddyAssets.map(asset => asset.kind === "settings"
      ? {
          ...asset,
          content: JSON.stringify({
            hooks: {
              SessionStart: [{ hooks: [{ type: "command", command: "node ./other-start.mjs" }] }],
              UserPromptSubmit: [{ hooks: [{ type: "command", command: "node ./other-prompt.mjs" }] }],
            },
          }),
        }
      : asset),
  )).toThrow(/managed hook command/);

  const codex = getHostDefinition("codex");
  expect(() => hostAssets.validateHostAssetsAgainstCapabilities(codex, [
    ...skillsOnly,
    {
      kind: "hook",
      filePath: "E:\\demo\\.codex\\hooks\\spec-wiki\\user-prompt-submit.mjs",
      content: "",
    },
  ])).toThrow(/trigger capabilities|skill guidance/);
});
