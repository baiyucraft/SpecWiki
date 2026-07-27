import { spawn } from "node:child_process";
import path from "node:path";
import { fileURLToPath } from "node:url";

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..");
const packageRoot = path.join(root, "packages", "spec-wiki");

function run(command, args, cwd = root) {
  return new Promise((resolve, reject) => {
    const child = spawn(command, args, {
      cwd,
      stdio: "inherit",
      shell: process.platform === "win32",
    });
    child.on("error", reject);
    child.on("close", (code) => code === 0
      ? resolve()
      : reject(new Error(`${command} ${args.join(" ")} exited with code ${code}`)));
  });
}

await run("pnpm", ["test"], packageRoot);
await run("pnpm", ["build"], packageRoot);
await run("pnpm", ["exec", "vitest", "run"], root);
