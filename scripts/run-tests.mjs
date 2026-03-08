import { spawn } from "node:child_process";
import { fileURLToPath, pathToFileURL } from "node:url";
import path from "node:path";

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);
const DEFAULT_ROOT_DIR = path.resolve(__dirname, "..");
const TEST_COMMANDS = [
  ["pnpm", ["--dir", "crates/wiki-core", "run", "test"]],
  ["pnpm", ["--dir", "agents/codebuddy", "run", "test"]],
  ["pnpm", ["--dir", "crates/wiki-core", "run", "build"]],
  ["pnpm", ["--dir", "agents/codebuddy", "run", "build"]],
  ["cargo", ["build", "-p", "wiki-core", "--target-dir", "crates/wiki-core/target/e2e"]],
  ["pnpm", ["exec", "vitest", "run", "--config", "vitest.config.mjs"]],
];

async function runCommand(command, args, { cwd = DEFAULT_ROOT_DIR } = {}) {
  // The root test entry is intentionally narrow:
  // core self-tests stay in the core package, agent self-tests stay in the agent package,
  // and only end-to-end / distribution checks remain in the root vitest suite.
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

export async function runAllTests({ rootDir = DEFAULT_ROOT_DIR } = {}) {
  for (const [command, args] of TEST_COMMANDS) {
    await runCommand(command, args, { cwd: rootDir });
  }
}

if (import.meta.url === pathToFileURL(process.argv[1]).href) {
  await runAllTests();
}
