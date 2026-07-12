/**
 * 这个文件负责 `spec-wiki init` 的主编排。
 * 它只决定本次要写哪些宿主资产，不承载宿主路径差异或 runtime forwarding。
 */
import { existsSync, mkdirSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import path from "node:path";

import {
  buildHostBootstrapAssets,
  listObsoleteHostBootstrapFiles,
} from "../../agents/shared/hostAssets.js";
import {
  resolveSelectedHosts,
  type SupportedHost,
} from "../../agents/shared/hosts.js";

export type ManagedFileStatus = "created" | "updated" | "unchanged";

export type ManagedFileResult = {
  /** 写入的宿主文件路径。 */
  path: string;
  /** 文件承担的资产类型。 */
  kind: "skill" | "hook" | "settings";
  /** 本次执行对该文件造成的状态变化。 */
  status: ManagedFileStatus;
};

export type BootstrapHostResult = {
  /** 本次执行的宿主。 */
  host: SupportedHost;
  /** 当前宿主下被管理的文件结果。 */
  files: ManagedFileResult[];
  /** 当前宿主的 bootstrap 结果。 */
  status: BootstrapOutcome;
  /** 写入失败时的目标路径。 */
  failedTarget?: string;
  /** 写入失败时的错误信息。 */
  error?: string;
};

export type BootstrapOutcome = "ready" | "partial" | "failed";

export type BootstrapInitResult = {
  /** 跨宿主聚合后的 bootstrap 结果。 */
  outcome: BootstrapOutcome;
  /** 最终执行 bootstrap 的宿主集合。 */
  hosts: BootstrapHostResult[];
  /** partial/failed 时供用户恢复的提示。 */
  recoveryHint?: string;
};

export type BootstrapInitOptions = {
  /** 当前仓库根目录。 */
  repoRoot: string;
  /** 显式传入的宿主参数。 */
  hosts?: string;
  /** 当前进程环境变量。 */
  env: NodeJS.ProcessEnv;
  /** 测试可注入的受管文件写入函数。 */
  writeFile?: (filePath: string, content: string) => ManagedFileStatus;
};

function writeManagedFile(filePath: string, content: string): ManagedFileStatus {
  const previousContent = existsSync(filePath)
    ? readFileSync(filePath, "utf8")
    : undefined;

  mkdirSync(path.dirname(filePath), { recursive: true });
  if (previousContent === content) {
    return "unchanged";
  }

  writeFileSync(filePath, content);
  return previousContent === undefined ? "created" : "updated";
}

function isWithinRepoRoot(repoRoot: string, filePath: string): boolean {
  const resolvedRepoRoot = path.resolve(repoRoot);
  const resolvedTarget = path.resolve(filePath);
  const relativePath = path.relative(resolvedRepoRoot, resolvedTarget);

  return relativePath === ""
    || (!relativePath.startsWith("..") && !path.isAbsolute(relativePath));
}

/**
 * 执行 `spec-wiki init` 的 orchestration，为目标宿主生成 skill、hook 与 settings 资产。
 *
 * @param options init 编排所需的项目根目录与环境变量。
 * @returns 返回按宿主聚合的文件写入结果。
 */
export async function runBootstrapInit(
  options: BootstrapInitOptions,
): Promise<BootstrapInitResult> {
  const selectedHosts = resolveSelectedHosts(options.repoRoot, options.hosts);
  const writeFile = options.writeFile ?? writeManagedFile;

  const hosts = selectedHosts.map((host) => {
    const files: ManagedFileResult[] = [];
    const assets = buildHostBootstrapAssets(options.repoRoot, options.env, host);
    const obsoleteFiles = listObsoleteHostBootstrapFiles(options.repoRoot, options.env, host);

    for (const filePath of obsoleteFiles) {
      if (!isWithinRepoRoot(options.repoRoot, filePath)) {
        continue;
      }

      rmSync(filePath, { recursive: true, force: true });
    }

    for (const asset of assets) {
      try {
        const status = writeFile(asset.filePath, asset.content);
        files.push({
          path: asset.filePath,
          kind: asset.kind,
          status,
        });
      } catch (error) {
        const message = error instanceof Error ? error.message : String(error);
        return {
          host,
          files,
          status: files.length > 0 ? "partial" as const : "failed" as const,
          failedTarget: asset.filePath,
          error: message,
        };
      }
    }

    return {
      host,
      files,
      status: "ready" as const,
    };
  });

  const outcome = hosts.every((host) => host.status === "ready")
    ? "ready"
    : hosts.some((host) => host.status === "ready" || host.status === "partial")
      ? "partial"
      : "failed";

  return {
    outcome,
    hosts,
    ...(outcome === "ready"
      ? {}
      : { recoveryHint: "resolve the failed host asset write and rerun spec-wiki init" }),
  };
}
