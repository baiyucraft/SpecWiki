/**
 * 这个文件负责定位 `wiki-runtime` 可执行文件。
 * `v0.2.0` 的正式发布只支持 Windows x64，因此这里必须先收紧平台边界，再解析包内或工作区二进制。
 */
import { existsSync } from "node:fs";
import path from "node:path";

import { findPackageRoot } from "../packageRoot.js";

const SUPPORTED_PLATFORMS = ["win32"] as const;
const SUPPORTED_ARCHS = ["x64"] as const;
const WINDOWS_BINARY_NAME = "wiki-runtime.exe";
const WINDOWS_BUNDLED_RUNTIME_DIR = path.join("lib", "x64-win32");

export type ResolveBinaryOptions = {
  /** 当前主包根目录；默认从当前 package 自动解析。 */
  packageRoot?: string;
  /** 显式覆盖环境变量名。 */
  envVarName?: string;
  /** 当前平台；默认取 `process.platform`。 */
  platform?: string;
  /** 当前架构；默认取 `process.arch`。 */
  arch?: string;
  /** 允许的平台白名单。 */
  supportedPlatforms?: readonly string[];
  /** 允许的架构白名单。 */
  supportedArchs?: readonly string[];
};

function assertSupportedRuntime(
  platform: string,
  arch: string,
  supportedPlatforms: readonly string[],
  supportedArchs: readonly string[],
): void {
  if (!supportedPlatforms.includes(platform)) {
    throw new Error(
      `spec-wiki v0.2.0 only supports ${supportedPlatforms.join(", ")}; current platform: ${platform}`,
    );
  }

  if (!supportedArchs.includes(arch)) {
    throw new Error(
      `spec-wiki v0.2.0 only supports ${supportedArchs.join(", ")}; current architecture: ${arch}`,
    );
  }
}

/**
 * 列出所有可能的 runtime 二进制位置，按“包内 runtime -> 工作区 release -> 工作区 debug”顺序回退。
 *
 * @param packageRoot 当前主包根目录。
 * @param platform 当前平台。
 * @param arch 当前架构。
 * @returns 返回按优先级排序的候选路径列表。
 */
export function collectBinaryCandidates(
  packageRoot: string,
  platform: string,
  arch = "x64",
): string[] {
  assertSupportedRuntime(platform, arch, SUPPORTED_PLATFORMS, SUPPORTED_ARCHS);

  return [
    path.join(packageRoot, WINDOWS_BUNDLED_RUNTIME_DIR, WINDOWS_BINARY_NAME),
    path.join(packageRoot, "..", "..", "target", "release", WINDOWS_BINARY_NAME),
    path.join(packageRoot, "..", "..", "target", "debug", WINDOWS_BINARY_NAME),
  ];
}

/**
 * 解析 `wiki-runtime` 二进制位置。
 *
 * @param options 当前平台和主包目录等解析选项。
 * @returns 返回最终应被 `spawn` 调用的 `wiki-runtime` 路径。
 */
export function resolveBinary(options: ResolveBinaryOptions = {}): string {
  const platform = options.platform ?? process.platform;
  const arch = options.arch ?? process.arch;
  const envVarName = options.envVarName ?? "SPEC_WIKI_RUNTIME_BIN";
  const supportedPlatforms = options.supportedPlatforms ?? SUPPORTED_PLATFORMS;
  const supportedArchs = options.supportedArchs ?? SUPPORTED_ARCHS;

  assertSupportedRuntime(platform, arch, supportedPlatforms, supportedArchs);

  if (process.env[envVarName]) {
    return process.env[envVarName] as string;
  }

  const packageRoot = options.packageRoot ?? findPackageRoot();
  const candidatePaths = collectBinaryCandidates(packageRoot, platform, arch);
  const existingBinary = candidatePaths.find((candidatePath) => existsSync(candidatePath));

  return existingBinary ?? candidatePaths[1];
}
