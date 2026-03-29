/**
 * 这个文件负责解析 `spec-wiki` 包根目录并读取发布资产。
 * CLI 在源码目录和构建产物目录下都要能稳定找到模板与配置。
 */
import { existsSync, readFileSync } from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

/** 当前模块所在目录，用于向上回溯 package 根目录。 */
const __dirname = path.dirname(fileURLToPath(import.meta.url));

/**
 * 从当前模块目录向上回溯 `package.json`，返回 `spec-wiki` 包根目录。
 *
 * @returns 返回当前 package 根目录。
 */
export function findPackageRoot(): string {
  let currentDir = __dirname;

  while (true) {
    if (existsSync(path.join(currentDir, "package.json"))) {
      return currentDir;
    }

    const parentDir = path.dirname(currentDir);
    if (parentDir === currentDir) {
      throw new Error(`failed to locate package root from ${__dirname}`);
    }

    currentDir = parentDir;
  }
}

/**
 * 读取当前包内的模板资产。
 *
 * @param relativePath 相对于 `assets/` 目录的路径。
 * @returns 返回模板文件内容。
 */
export function readPackageAsset(relativePath: string): string {
  return readFileSync(path.join(findPackageRoot(), "assets", relativePath), "utf8");
}
