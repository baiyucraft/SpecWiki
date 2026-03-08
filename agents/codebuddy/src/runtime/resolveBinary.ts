/**
 * 这个文件负责定位 `wiki-core` 可执行文件。
 * 它优先使用已安装的平台包，其次回退到工作区里的本地构建产物。
 */
import { existsSync } from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

/** 当前运行文件的真实绝对路径。 */
const __filename = fileURLToPath(import.meta.url);
/** 运行文件所在目录，用于向上回溯 package 根目录。 */
const __dirname = path.dirname(__filename);
/** 主 npm 包名称，也是平台包命名的前缀。 */
const MAIN_PACKAGE_NAME = "codebuddy-wiki";
/** Windows 平台包名称；当前阶段只支持这个平台包。 */
const WIN32_PLATFORM_PACKAGE_NAME = `${MAIN_PACKAGE_NAME}-win32-x64-msvc`;
/** `wiki-core` 在 Windows 下的可执行文件名。 */
const WIN32_BINARY_NAME = "wiki-core.exe";

/**
 * 从当前文件位置向上回溯 package 根目录。
 * 这样源码目录和构建产物目录都能走同一套定位逻辑。
 *
 * @param startDir 开始向上查找 `package.json` 的目录。
 * @returns 返回当前 codebuddy 包的根目录路径。
 */
function findPackageRoot(startDir: string): string {
  // 运行时既可能从源码目录执行，也可能从构建后的 dist 目录执行。
  // 一直向上找到 package.json，可以让这两种布局复用同一套定位逻辑。
  let currentDir = startDir;

  while (true) {
    if (existsSync(path.join(currentDir, "package.json"))) {
      return currentDir;
    }

    const parentDir = path.dirname(currentDir);
    if (parentDir === currentDir) {
      throw new Error(`failed to locate package root from ${startDir}`);
    }

    currentDir = parentDir;
  }
}

/**
 * 列出所有可能的二进制位置，按“已发布平台包 -> 本地开发构建”顺序回退。
 *
 * @param packageRoot 当前 codebuddy 包的根目录。
 * @returns 返回按优先级排序的 `wiki-core` 二进制候选路径列表。
 */
function collectBinaryCandidates(packageRoot: string): string[] {
  // 已发布包优先使用可选平台包里的二进制。
  // 本地开发时则回退到工作区里的 Rust 构建产物。
  return [
    path.join(packageRoot, "node_modules", WIN32_PLATFORM_PACKAGE_NAME, "bin", WIN32_BINARY_NAME),
    path.join(packageRoot, "..", WIN32_PLATFORM_PACKAGE_NAME, "bin", WIN32_BINARY_NAME),
    path.join(packageRoot, "..", "..", WIN32_PLATFORM_PACKAGE_NAME, "bin", WIN32_BINARY_NAME),
    path.join(packageRoot, "..", "..", "crates", "wiki-core", "target", "debug", WIN32_BINARY_NAME),
  ];
}

/**
 * 解析 `wiki-core` 二进制位置。
 * 当前只支持 Windows，因此其他平台直接在这里给出明确错误。
 *
 * @param platform 当前运行平台；默认取 `process.platform`。
 * @returns 返回最终应被 `spawn` 调用的 `wiki-core` 路径。
 */
export function resolveBinary(platform = process.platform): string {
  if (process.env.CODEBUDDY_WIKI_CORE_BIN) {
    return process.env.CODEBUDDY_WIKI_CORE_BIN;
  }

  if (platform !== "win32") {
    throw new Error(`CodeBuddy Agent currently supports Windows only (received ${platform})`);
  }

  const packageRoot = findPackageRoot(__dirname);
  const candidatePaths = collectBinaryCandidates(packageRoot);
  const existingBinary = candidatePaths.find((candidatePath) => existsSync(candidatePath));

  // 即使开发兜底路径还不存在，也先把它返回出去。
  // 这样调用方拿到的报错里会直接出现“下一步该编译哪个路径”。
  return existingBinary ?? candidatePaths[candidatePaths.length - 1];
}
