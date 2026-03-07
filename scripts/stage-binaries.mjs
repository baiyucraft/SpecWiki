import { cpSync, mkdirSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import path from "node:path";

import { resolveBuiltBinary } from "./build-core.mjs";

const MAIN_PACKAGE_NAME = "codebuddy-wiki";
const PLATFORM_PACKAGE_NAME = "codebuddy-wiki-win32-x64-msvc";

export async function stagePackages({ rootDir }) {
  const distDir = path.join(rootDir, "dist", "npm");
  const mainDir = path.join(distDir, MAIN_PACKAGE_NAME);
  const platformDir = path.join(distDir, PLATFORM_PACKAGE_NAME);
  const binaryPath = resolveBuiltBinary(rootDir);
  const templatePath = path.join(
    rootDir,
    "packages",
    "codebuddy",
    "templates",
    "platform-package.json",
  );

  rmSync(distDir, { recursive: true, force: true });
  mkdirSync(path.join(mainDir, "bin"), { recursive: true });
  mkdirSync(path.join(platformDir, "bin"), { recursive: true });

  const mainManifest = {
    name: MAIN_PACKAGE_NAME,
    version: "0.1.0",
    private: false,
    bin: {
      "codebuddy-wiki": "./bin/codebuddy.js",
    },
    optionalDependencies: {
      [PLATFORM_PACKAGE_NAME]: "0.1.0",
    },
  };

  const template = readFileSync(templatePath, "utf8").replaceAll(
    "__PACKAGE_NAME__",
    PLATFORM_PACKAGE_NAME,
  );

  writeFileSync(
    path.join(mainDir, "package.json"),
    `${JSON.stringify(mainManifest, null, 2)}\n`,
  );
  writeFileSync(path.join(mainDir, "bin", "codebuddy.js"), "console.log('codebuddy-wiki');\n");
  writeFileSync(path.join(platformDir, "package.json"), `${template}\n`);
  cpSync(binaryPath, path.join(platformDir, "bin", path.basename(binaryPath)));
}
