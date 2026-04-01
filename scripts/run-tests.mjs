/**
 * 这个脚本是工作区级测试入口。
 * 它顺序执行 Rust runtime、自测主包和 root e2e / distribution 测试。
 */
import { spawn } from "node:child_process";
import { fileURLToPath, pathToFileURL } from "node:url";
import path from "node:path";

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);
const DEFAULT_ROOT_DIR = path.resolve(__dirname, "..");
const MAIN_PACKAGE_DIR = path.join(DEFAULT_ROOT_DIR, "packages", "spec-wiki");
const TEST_COMMANDS = [
  {
    command: "cargo",
    args: ["test", "-p", "wiki-runtime", "--target-dir", "target"],
  },
  {
    command: "pnpm",
    args: ["test"],
    cwd: MAIN_PACKAGE_DIR,
  },
  {
    command: "cargo",
    args: ["build", "-p", "wiki-runtime", "--target-dir", "target"],
  },
  {
    command: "pnpm",
    args: ["build"],
    cwd: MAIN_PACKAGE_DIR,
  },
  {
    command: "cargo",
    args: ["build", "-p", "wiki-runtime", "--target-dir", "target/e2e"],
  },
  {
    command: "pnpm",
    args: ["exec", "vitest", "run", "--config", "vitest.config.mjs"],
  },
];

async function runCommand(command, args, { cwd = DEFAULT_ROOT_DIR } = {}) {
  // The root test entry is intentionally narrow:
  // core self-tests stay in the core package, CLI self-tests stay in the main package,
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
  for (const { command, args, cwd } of TEST_COMMANDS) {
    const commandCwd = cwd ?? rootDir;
    await runCommand(command, args, { cwd: commandCwd });
  }
}

if (import.meta.url === pathToFileURL(process.argv[1]).href) {
  await runAllTests();
}
