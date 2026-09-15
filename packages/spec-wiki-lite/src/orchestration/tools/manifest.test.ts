import { expect, test } from "vitest";

import { AOCI_RELEASE, CODEGRAPH_RELEASE, selectAociAsset } from "./manifest.js";

test("pins compatible CodeGraph and AOCI releases", () => {
  expect(CODEGRAPH_RELEASE).toEqual(expect.objectContaining({ package: "@colbymchenry/codegraph", version: "1.6.0" }));
  expect(AOCI_RELEASE.version).toBe("0.1.0-rc12");
  expect(Object.keys(AOCI_RELEASE.assets)).toHaveLength(6);
  expect(selectAociAsset("win32", "x64")).toEqual(expect.objectContaining({
    archive: "aoci_0.1.0-rc12_windows_amd64.zip",
    sha256: "3b6a80f66ab2411b1cafa8142275622c862bae0a55e6efb9f27bd589faf790da",
  }));
});

test("rejects unsupported AOCI platforms", () => {
  expect(() => selectAociAsset("freebsd", "x64")).toThrow(/unsupported AOCI platform/iu);
});
