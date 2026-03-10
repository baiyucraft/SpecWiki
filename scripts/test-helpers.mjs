// wiki-core 测试脚本共享工具。
// 提供二进制路径解析、JSON IPC 调用、断言辅助等。

import { execFileSync, execSync } from "node:child_process";
import { existsSync, readdirSync, readFileSync } from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

export const ROOT_DIR = path.resolve(__dirname, "..");
export const TMP_DIR = path.join(ROOT_DIR, "tmp");
export const TEST_DIR = path.join(TMP_DIR, "test");

const BINARY_NAME = process.platform === "win32" ? "wiki-core.exe" : "wiki-core";
const BINARY_PATH = path.join(ROOT_DIR, "target", "release", BINARY_NAME);

// 初始化、更新和重建会真正跑完整 workflow，monorepo 项目明显比 query/status 更慢。
// 测试脚本在这些 action 上使用更长超时，避免因为固定 60s 误判失败。
const DEFAULT_TIMEOUT_MS = 60_000;
const HEAVY_ACTION_TIMEOUT_MS = 180_000;

// -------------------------------------------------------------------------
// 二进制
// -------------------------------------------------------------------------

/**
 * 确保测试脚本使用的是可执行的 release binary。
 *
 * @param options 运行选项；`fresh` 为真时总是先做一次 release build。
 * @returns 无返回值；如果构建失败会直接抛错。
 */
export function ensureBinary(options = {}) {
  if (options.fresh || !existsSync(BINARY_PATH)) {
    console.log(`[build] ${options.fresh ? "refreshing" : "release binary not found, building"}...`);
    execSync("cargo build --release -p wiki-core", { cwd: ROOT_DIR, stdio: "inherit" });
  }
}

// -------------------------------------------------------------------------
// JSON IPC
// -------------------------------------------------------------------------

/**
 * 通过 release binary 执行 JSON IPC 调用。
 *
 * @param command 要发送给 core 的命令对象。
 * @param options 运行选项；可覆盖默认超时。
 * @returns 返回 core 的 JSON 响应。
 */
export function callCore(command, options = {}) {
  const input = JSON.stringify(command);
  const timeout =
    options.timeoutMs
    ?? (["init", "update", "rebuild"].includes(command.action)
      ? HEAVY_ACTION_TIMEOUT_MS
      : DEFAULT_TIMEOUT_MS);
  const output = execFileSync(BINARY_PATH, ["--json"], {
    cwd: ROOT_DIR,
    input,
    encoding: "utf-8",
    timeout,
    maxBuffer: 50 * 1024 * 1024,
  });
  return JSON.parse(output.trim());
}

// -------------------------------------------------------------------------
// 文件工具
// -------------------------------------------------------------------------

export function countMdFiles(dir) {
  if (!existsSync(dir)) return 0;
  let count = 0;
  const walk = (d) => {
    for (const entry of readdirSync(d, { withFileTypes: true })) {
      if (entry.isDirectory()) walk(path.join(d, entry.name));
      else if (entry.name.endsWith(".md")) count++;
    }
  };
  walk(dir);
  return count;
}

export function countFilesWithMarker(dir) {
  if (!existsSync(dir)) return 0;
  let count = 0;
  const walk = (d) => {
    for (const entry of readdirSync(d, { withFileTypes: true })) {
      const full = path.join(d, entry.name);
      if (entry.isDirectory()) walk(full);
      else if (entry.name.endsWith(".md")) {
        const content = readFileSync(full, "utf-8");
        if (content.includes("<!-- wiki:managed:start")) count++;
      }
    }
  };
  walk(dir);
  return count;
}

// -------------------------------------------------------------------------
// 断言
// -------------------------------------------------------------------------

export class TestRunner {
  constructor() {
    this.total = 0;
    this.passed = 0;
    this.failed = 0;
  }

  pass(label) {
    this.total++;
    this.passed++;
    console.log(`    \x1b[32mPASS\x1b[0m ${label}`);
  }

  fail(label, detail) {
    this.total++;
    this.failed++;
    console.log(`    \x1b[31mFAIL\x1b[0m ${label}`);
    if (detail) console.log(`         ${detail}`);
  }

  skip(label) {
    console.log(`    \x1b[33mSKIP\x1b[0m ${label}`);
  }

  assertOk(label, result) {
    result.ok ? this.pass(label) : this.fail(label, result.error);
  }

  assertEqual(label, actual, expected) {
    actual === expected
      ? this.pass(label)
      : this.fail(label, `expected ${JSON.stringify(expected)}, got ${JSON.stringify(actual)}`);
  }

  assertContains(label, obj, key, value) {
    const actual = obj?.[key];
    actual === value
      ? this.pass(label)
      : this.fail(label, `expected ${key}=${JSON.stringify(value)}, got ${JSON.stringify(actual)}`);
  }

  assertFileExists(label, filePath) {
    existsSync(filePath) ? this.pass(label) : this.fail(label, `not found: ${filePath}`);
  }

  assertMarkerCoverage(label, wikiDir) {
    const totalMd = countMdFiles(wikiDir);
    const withMarker = countFilesWithMarker(wikiDir);
    totalMd > 0 && totalMd === withMarker
      ? this.pass(`${label} (${withMarker}/${totalMd})`)
      : this.fail(`${label} (${withMarker}/${totalMd})`);
  }

  summary() {
    console.log("");
    console.log("=== Results ===");
    console.log(
      `Total: ${this.total}  \x1b[32mPassed: ${this.passed}\x1b[0m  \x1b[31mFailed: ${this.failed}\x1b[0m`,
    );
    return this.failed === 0;
  }
}
