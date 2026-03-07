import path from "node:path";

import { stagePackages } from "./stage-binaries.mjs";

const rootDir = process.cwd();

await stagePackages({ rootDir });

console.log(`Staged npm packages in ${path.join(rootDir, "dist", "npm")}`);
