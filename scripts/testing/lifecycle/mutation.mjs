import { pathToFileURL } from "node:url";

import { runLifecycleTests } from "../../test-wiki-lifecycle.mjs";

if (import.meta.url === pathToFileURL(process.argv[1]).href) {
  const ok = await runLifecycleTests(process.argv.slice(2), { phase: "mutation" });
  if (!ok)
process.exit(1);
}
