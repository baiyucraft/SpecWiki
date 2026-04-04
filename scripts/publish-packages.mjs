/**
 * 这个脚本负责基于当前源码构建 staged package，并执行 publish 或 dry-run 验证。
 * 它只收口 package evidence 与发布动作，不重新定义公开 workflow 或 runtime 合同。
 */
import { spawn } from "node:child_process";
import { pathToFileURL, fileURLToPath } from "node:url";
import path from "node:path";

import { assertStagedPackageEvidence, buildDistribution, collectStagedPackageEvidence } from "./build-dist.mjs";

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

/**
 * 构建 staged package、执行一致性校验，并在目标目录运行 publish/dry-run。
 *
 * @param options 发布根目录与 dry-run 开关。
 * @returns 返回 staged package 结果与对应的 evidence 检查摘要。
 */
export async function publishPackages({
  rootDir = DEFAULT_ROOT_DIR,
  dryRun = process.argv.includes("--dry-run"),
} = {}) {
  const built = await buildDistribution({ rootDir });
  const evidence = await collectStagedPackageEvidence({
    rootDir,
    packageDir: built.package.packageDir,
  });
  assertStagedPackageEvidence(evidence);
  const publishArgs = dryRun ? ["publish", "--dry-run"] : ["publish"];

  await runCommand("npm", publishArgs, { cwd: built.package.packageDir });

  return { ...built, evidence };
}

if (import.meta.url === pathToFileURL(process.argv[1]).href) {
  const built = await publishPackages();

  console.log(`${process.argv.includes("--dry-run") ? "Dry-run checked" : "Published"} package from ${built.package.packageDir}`);
  console.log(`Evidence checks: ${Object.keys(built.evidence.checks).length}`);
}

