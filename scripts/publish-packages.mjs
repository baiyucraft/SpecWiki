import { spawn } from "node:child_process";
import { pathToFileURL, fileURLToPath } from "node:url";
import path from "node:path";

import { buildDistribution } from "./build-dist.mjs";

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);
const DEFAULT_ROOT_DIR = path.resolve(__dirname, "..");

async function runCommand(command, args, { cwd = DEFAULT_ROOT_DIR } = {}) {
  await new Promise((resolve, reject) => {
    const child = spawn(command, args, {
      cwd,
      stdio: "inherit",
      shell: process.platform === "win32",
    });

    child.on("error", reject);
    child.on("close", (code) => {
      if (code === 0) {
        resolve();
        return;
      }

      reject(new Error(`${command} ${args.join(" ")} exited with code ${code}`));
    });
  });
}

export async function publishPackages({
  rootDir = DEFAULT_ROOT_DIR,
  dryRun = process.argv.includes("--dry-run"),
} = {}) {
  const built = await buildDistribution({ rootDir });
  const publishArgs = dryRun ? ["publish", "--dry-run"] : ["publish"];

  await runCommand("npm", publishArgs, { cwd: built.package.packageDir });

  return built;
}

if (import.meta.url === pathToFileURL(process.argv[1]).href) {
  const built = await publishPackages();

  console.log(`Published package from ${built.package.packageDir}`);
}
