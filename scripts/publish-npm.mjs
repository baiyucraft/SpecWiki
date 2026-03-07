import path from "node:path";

import { stagePackages } from "./stage-binaries.mjs";

const rootDir = process.cwd();

const staged = await stagePackages({ rootDir });

console.log(`Staged npm packages in ${path.join(rootDir, "dist", "npm")}`);
console.log(`Main package: ${staged.mainManifest.name}@${staged.mainManifest.version}`);
console.log(`Platform package: ${staged.platformManifest.name}`);
console.log(`Binary source: ${staged.binaryPath}`);
