// wiki-core 测试脚本共享工具。
// 提供二进制路径解析、JSON IPC 调用、断言辅助等。

import { execFileSync, execSync, spawn } from "node:child_process";
import { existsSync, readdirSync, readFileSync, rmSync } from "node:fs";
import { availableParallelism } from "node:os";
import path from "node:path";
import { fileURLToPath } from "node:url";

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

export const ROOT_DIR = path.resolve(__dirname, "..", "..");
export const TMP_DIR = path.join(ROOT_DIR, "tmp");
export const TEST_DIR = path.join(TMP_DIR, "test");
const DEFAULT_PROJECT_JOBS = 8;

const BINARY_NAME = process.platform === "win32" ? "wiki-core.exe" : "wiki-core";
const BINARY_PATH = path.join(ROOT_DIR, "target", "release", BINARY_NAME);

// 初始化、更新和重建会真正跑完整 workflow，monorepo 项目明显比 query/status 更慢。
// 项目集脚本还会并行拉起多个长流程 worker，因此需要给重仓库留足超时窗口。
const DEFAULT_TIMEOUT_MS = 60_000;
const HEAVY_ACTION_TIMEOUT_MS = 600_000;
const REMOVE_RETRY_DELAY_MS = 500;
const REMOVE_RETRY_ATTEMPTS = 40;

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

/**
 * 解析测试项目脚本默认并行度。
 * 项目集验证更看重总耗时，默认允许 8 个项目并发；调用方也可以通过 `--jobs` 覆盖。
 *
 * @param requestedJobs 调用方显式传入的并行度。
 * @param totalProjects 本次要跑的项目总数。
 * @returns 返回裁剪到合法范围内的项目并行度。
 */
export function resolveProjectJobs(requestedJobs, totalProjects) {
  const platformLimit =
    typeof availableParallelism === "function" ? availableParallelism() : DEFAULT_PROJECT_JOBS;
  const parsedJobs = Number(requestedJobs);
  const desiredJobs =
    Number.isFinite(parsedJobs) && parsedJobs > 0
      ? Math.floor(parsedJobs)
      : Math.min(DEFAULT_PROJECT_JOBS, platformLimit);
  return Math.max(1, Math.min(totalProjects || 1, desiredJobs));
}

/**
 * 用固定并行度执行异步任务，并保持结果顺序与输入一致。
 *
 * @param items 待处理项目列表。
 * @param jobs 并行 worker 数。
 * @param worker 实际执行单项任务的异步函数。
 * @returns 返回与输入顺序一致的结果数组。
 */
export async function runTaskPool(items, jobs, worker) {
  const results = new Array(items.length);
  let nextIndex = 0;
  const workerCount = Math.max(1, Math.min(jobs, items.length || 1));

  await Promise.all(
    Array.from({ length: workerCount }, async () => {
      while (true) {
        const currentIndex = nextIndex;
        nextIndex += 1;
        if (currentIndex >= items.length) {
          return;
        }
        results[currentIndex] = await worker(items[currentIndex], currentIndex);
      }
    }),
  );

  return results;
}

/**
 * 启动子进程并收集 stdout/stderr，供项目级并行 worker 复用。
 *
 * @param command 要执行的命令。
 * @param args 命令参数数组。
 * @param options 运行选项；默认在仓库根目录执行且不走 shell。
 * @returns 返回退出码和捕获到的标准输出/错误。
 */
export async function runCommandCapture(
  command,
  args,
  { cwd = ROOT_DIR, shell = false } = {},
) {
  return await new Promise((resolve, reject) => {
    const child = spawn(command, args, {
      cwd,
      shell,
      stdio: ["ignore", "pipe", "pipe"],
    });

    let stdout = "";
    let stderr = "";

    child.stdout.on("data", (chunk) => {
      stdout += chunk.toString();
    });
    child.stderr.on("data", (chunk) => {
      stderr += chunk.toString();
    });
    child.on("error", reject);
    child.on("close", (code) => {
      resolve({
        code: code ?? -1,
        stdout,
        stderr,
      });
    });
  });
}

function sleepSync(ms) {
  Atomics.wait(new Int32Array(new SharedArrayBuffer(4)), 0, 0, ms);
}

/**
 * 在 Windows 文件句柄释放有滞后时，带重试地删除目录或文件。
 *
 * @param targetPath 待删除路径。
 * @param options 删除选项；支持覆盖重试次数和延迟。
 * @returns 无返回值；若重试后仍失败则抛出最后一次错误。
 */
export function removePathWithRetry(
  targetPath,
  { delayMs = REMOVE_RETRY_DELAY_MS, maxAttempts = REMOVE_RETRY_ATTEMPTS } = {},
) {
  if (!existsSync(targetPath)) {
    return;
  }

  let lastError = null;
  for (let attempt = 0; attempt < maxAttempts; attempt++) {
    try {
      rmSync(targetPath, { recursive: true, force: true });
      return;
    } catch (error) {
      lastError = error;
      const message = String(error?.code || error?.message || "");
      if (!["EBUSY", "EPERM", "ENOTEMPTY"].some((code) => message.includes(code))) {
        throw error;
      }
      sleepSync(delayMs);
    }
  }

  throw lastError;
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
  return parseCoreOutput(output);
}

function parseCoreOutput(output) {
  const trimmed = output.trim();
  if (!trimmed) {
    throw new Error("wiki-core returned empty stdout");
  }

  const lines = trimmed.split(/\r?\n/).filter(Boolean);
  if (lines.length === 1) {
    return JSON.parse(lines[0]);
  }

  let terminal = null;
  for (const line of lines) {
    const parsed = JSON.parse(line);
    if (parsed.type === "progress") {
      continue;
    }
    if ((parsed.type === "result" || parsed.type === "error") && parsed.response) {
      if (terminal) {
        throw new Error("wiki-core emitted multiple terminal events");
      }
      terminal = parsed.response;
      continue;
    }
    throw new Error(`unexpected wiki-core event: ${line}`);
  }

  if (!terminal) {
    throw new Error("wiki-core stream ended without terminal response");
  }

  return terminal;
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
