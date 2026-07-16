/**
 * wiki-runtime 测试脚本共享工具。
 *
 * 这里集中维护二进制解析、子进程调用、超时清理与断言辅助，
 * 避免不同脚本各自复制一套 Windows 锁文件与长流程治理逻辑。
 */

import { execFileSync, execSync, spawn } from "node:child_process";
import { existsSync, readdirSync, readFileSync, rmSync, statSync, unlinkSync, writeFileSync } from "node:fs";
import { availableParallelism } from "node:os";
import path from "node:path";
import { fileURLToPath } from "node:url";

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

export const ROOT_DIR = path.resolve(__dirname, "..", "..");
export const TMP_DIR = path.join(ROOT_DIR, "tmp");
export const TEST_DIR = path.join(TMP_DIR, "test");
export const ROOT_DEV_CONFIG_PATH = path.join(ROOT_DIR, "wiki.dev.yaml");
const DEFAULT_PROJECT_JOBS = 8;

const BINARY_NAME = process.platform === "win32" ? "wiki-runtime.exe" : "wiki-runtime";
const DEBUG_BINARY_PATH = path.join(ROOT_DIR, "target", "debug", BINARY_NAME);
const RELEASE_BINARY_PATH = path.join(ROOT_DIR, "target", "release", BINARY_NAME);

// 初始化、更新和重建会真正跑完整 workflow，monorepo 项目明显比 query/status 更慢。
// 项目集脚本还会并行拉起多个长流程 worker，因此需要给重仓库留足超时窗口。
const DEFAULT_TIMEOUT_MS = 60_000;
const HEAVY_ACTION_TIMEOUT_MS = 3_600_000;
const REMOVE_RETRY_DELAY_MS = 500;
const REMOVE_RETRY_ATTEMPTS = 40;
const FILE_RETRY_DELAY_MS = 250;
const FILE_RETRY_ATTEMPTS = 20;

// 项目级 child worker 会先依赖内部 timeout 自己收尾，这里额外预留两分钟给日志冲刷和 finally 清理。
export const COMMAND_TIMEOUT_GRACE_MS = 2 * 60_000;

// -------------------------------------------------------------------------
// 二进制
// -------------------------------------------------------------------------

/**
 * 确保测试脚本至少有一个可执行的 `wiki-runtime` binary 可用。
 *
 * @param options 运行选项；`fresh` 为真时总是先做一次 release build。
 */
export function ensureBinary(options = {}) {
  if (options.fresh || (!existsSync(DEBUG_BINARY_PATH) && !existsSync(RELEASE_BINARY_PATH))) {
    console.log(`[build] ${options.fresh ? "refreshing" : "release binary not found, building"}...`);
    execSync("cargo build --release -p wiki-runtime", { cwd: ROOT_DIR, stdio: "inherit" });
  }
}

/**
 * 解析当前应该使用的 `wiki-runtime` binary。
 *
 * 优先使用较新的 build 产物，避免测试脚本继续调用过期的 release binary。
 *
 * @returns 返回 debug/release 中较新的可执行文件路径。
 */
function resolveBinaryPath() {
  const hasDebug = existsSync(DEBUG_BINARY_PATH);
  const hasRelease = existsSync(RELEASE_BINARY_PATH);

  if (!hasDebug && !hasRelease) {
    throw new Error(`缺少 wiki-runtime binary: ${DEBUG_BINARY_PATH} / ${RELEASE_BINARY_PATH}`);
  }
  if (!hasDebug) {
    return RELEASE_BINARY_PATH;
  }
  if (!hasRelease) {
    return DEBUG_BINARY_PATH;
  }

  return statSync(DEBUG_BINARY_PATH).mtimeMs >= statSync(RELEASE_BINARY_PATH).mtimeMs
    ? DEBUG_BINARY_PATH
    : RELEASE_BINARY_PATH;
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
  const platformLimit
    = typeof availableParallelism === "function" ? availableParallelism() : DEFAULT_PROJECT_JOBS;
  const parsedJobs = Number(requestedJobs);
  const desiredJobs
    = Number.isFinite(parsedJobs) && parsedJobs > 0
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
  const results = Array.from({ length: items.length });
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
 * 按输入顺序串行执行异步任务。
 *
 * @param items 待处理项目列表。
 * @param worker 实际执行单项任务的异步函数。
 * @returns 返回与输入顺序一致的结果数组。
 */
export async function runSequentialTasks(items, worker) {
  const results = [];
  for (let index = 0; index < items.length; index++) {
    results.push(await worker(items[index], index));
  }
  return results;
}

/**
 * 判断错误文本是否属于 Windows 常见的瞬态文件锁或目录占用问题。
 *
 * @param message 错误码或错误消息。
 * @returns 命中 `EBUSY`、`EPERM`、`ENOTEMPTY` 或 `os error 32` 时返回 `true`。
 */
export function isTransientFsErrorMessage(message) {
  const normalized = String(message ?? "");
  return (
    ["EBUSY", "EPERM", "ENOTEMPTY"].some((code) => normalized.includes(code))
    || normalized.includes("os error 32")
  );
}

export function isPreserveResumeEligibleInitErrorMessage(message) {
  const normalized = String(message ?? "").toLowerCase();
  return (
    normalized.includes("timed out after")
    || normalized.includes("error sending request for url")
    || normalized.includes("error decoding response body")
    || normalized.includes("provider returned 408")
    || normalized.includes("provider returned 429")
    || /provider returned 5\d\d/.test(normalized)
  );
}

/**
 * 启动子进程并收集 stdout/stderr，供项目级并行 worker 复用。
 *
 * @param command 要执行的命令。
 * @param args 命令参数数组。
 * @param options 运行选项；默认在仓库根目录执行且不走 shell，并可附加超时。
 * @param options.cwd 执行命令时使用的工作目录。
 * @param options.shell 是否通过 shell 启动子进程。
 * @param options.timeoutMs 子进程超时时间。
 * @param options.killTreeOnTimeout Windows 超时时是否回收整棵进程树。
 * @returns 返回退出码、退出信号、是否超时以及捕获到的标准输出/错误。
 */
export async function runCommandCapture(
  command,
  args,
  { cwd = ROOT_DIR, shell = false, timeoutMs, killTreeOnTimeout = false } = {},
) {
  return await new Promise((resolve, reject) => {
    const child = spawn(command, args, {
      cwd,
      shell,
      stdio: ["ignore", "pipe", "pipe"],
    });

    let stdout = "";
    let stderr = "";
    let timedOut = false;
    const timer
      = Number.isFinite(timeoutMs) && timeoutMs > 0
        ? setTimeout(() => {
          timedOut = true;
          stderr = appendProcessMessage(
            stderr,
            `command timed out after ${timeoutMs}ms`,
          );
          terminateChildProcess(child, { killTreeOnWindows: killTreeOnTimeout });
        }, timeoutMs)
        : null;

    child.stdout.on("data", (chunk) => {
      stdout += chunk.toString();
    });
    child.stderr.on("data", (chunk) => {
      stderr += chunk.toString();
    });
    child.on("error", (error) => {
      if (timer) {
        clearTimeout(timer);
      }
      reject(error);
    });
    child.on("close", (code, signal) => {
      if (timer) {
        clearTimeout(timer);
      }
      resolve({
        code: timedOut ? -1 : (code ?? -1),
        signal: signal ?? null,
        stdout,
        stderr,
        timedOut,
      });
    });
  });
}

function sleepSync(ms) {
  Atomics.wait(new Int32Array(new SharedArrayBuffer(4)), 0, 0, ms);
}

function isTransientFsError(error) {
  return isTransientFsErrorMessage(error?.code || error?.message || "");
}

function appendProcessMessage(stderr, message) {
  return stderr ? `${stderr}\n${message}` : message;
}

function tryKillProcessTree(pid) {
  if (!pid || process.platform !== "win32") {
    return;
  }

  try {
    execFileSync("taskkill", ["/PID", String(pid), "/T", "/F"], {
      stdio: "ignore",
      windowsHide: true,
      timeout: 15_000,
    });
  } catch {
    // 进程可能已经退出，这里不再覆盖原始失败语义。
  }
}

/**
 * 终止测试脚本拉起的子进程。
 *
 * 对 `wiki-runtime` 这类 leaf 进程，Windows 下需要回收整棵进程树，避免残留句柄继续锁住 `.wiki/.cache`；
 * 对项目级 node worker，只有在外层已经判定超时失控时才应回收整棵树，否则会留下孤儿 `wiki-runtime`。
 *
 * @param child Node `spawn()` 返回的子进程句柄。
 * @param options 终止选项；`killTreeOnWindows` 仅应在 leaf 进程上启用。
 */
export function terminateChildProcess(child, options = {}) {
  if (!child || child.killed) {
    return;
  }

  if (process.platform === "win32" && options.killTreeOnWindows) {
    tryKillProcessTree(child.pid);
    return;
  }

  try {
    child.kill("SIGKILL");
  } catch {
    // 进程可能已经退出，这里保留调用方原始错误。
  }
}

function writeFileWithRetry(filePath, content) {
  let lastError = null;

  for (let attempt = 0; attempt < FILE_RETRY_ATTEMPTS; attempt++) {
    try {
      writeFileSync(filePath, content);
      return;
    } catch (error) {
      lastError = error;
      if (!isTransientFsError(error)) {
        throw error;
      }
      sleepSync(FILE_RETRY_DELAY_MS);
    }
  }

  throw lastError;
}

function unlinkFileWithRetry(filePath) {
  let lastError = null;

  for (let attempt = 0; attempt < FILE_RETRY_ATTEMPTS; attempt++) {
    try {
      unlinkSync(filePath);
      return;
    } catch (error) {
      lastError = error;
      if (!isTransientFsError(error)) {
        throw error;
      }
      sleepSync(FILE_RETRY_DELAY_MS);
    }
  }

  throw lastError;
}

/**
 * 在 Windows 文件句柄释放有滞后时，带重试地删除目录或文件。
 *
 * @param targetPath 待删除路径。
 * @param options 删除选项；支持覆盖重试次数和延迟。
 * @param options.delayMs 每次重试前的等待时间。
 * @param options.maxAttempts 最大重试次数。
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
      if (!isTransientFsError(error)) {
        throw error;
      }
      sleepSync(delayMs);
    }
  }

  throw lastError;
}

// -------------------------------------------------------------------------
// SQLite
// -------------------------------------------------------------------------

/**
 * 读取 sqlite 查询结果，并在数据库短暂锁表时自动重试。
 *
 * @param dbPath SQLite 文件路径。
 * @param sql 要执行的 SQL 语句。
 * @param options 运行选项；支持覆盖超时和缓冲区。
 * @param options.timeout sqlite3 命令超时时间。
 * @param options.maxBuffer sqlite3 输出缓冲区大小。
 * @returns 返回去掉空行后的文本结果。
 */
export function querySqliteRows(
  dbPath,
  sql,
  { timeout = 35_000, maxBuffer = 64 * 1024 * 1024 } = {},
) {
  if (!existsSync(dbPath)) {
    return [];
  }

  const statement = `PRAGMA busy_timeout=30000; ${sql}`;
  let lastError = null;

  for (let attempt = 0; attempt < 5; attempt++) {
    try {
      const output = execFileSync("sqlite3", [dbPath, statement], {
        encoding: "utf-8",
        timeout,
        maxBuffer,
      })
        .split(/\r?\n/)
        .map((line) => line.trim())
        .filter(Boolean);
      if (output[0] === "30000") {
        output.shift();
      }
      return output;
    } catch (error) {
      lastError = error;
      if (!String(error.stderr || error.message || "").includes("database is locked")) {
        throw error;
      }
      sleepSync(250);
    }
  }

  throw lastError;
}

/**
 * 读取 sqlite 单值查询结果。
 *
 * @param dbPath SQLite 文件路径。
 * @param sql 要执行的 SQL 语句。
 * @returns 返回最后一行结果；若无结果则返回空字符串。
 */
export function querySqliteValue(dbPath, sql) {
  const rows = querySqliteRows(dbPath, sql);
  return rows.at(-1) ?? "";
}

/**
 * 把 sqlite 文本结果安全转成数字。
 *
 * @param value sqlite 返回值。
 * @returns 返回有限数字；无效值统一退回 0。
 */
export function parseSqliteNumber(value) {
  const parsed = Number(value ?? 0);
  return Number.isFinite(parsed) ? parsed : 0;
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
  const binaryPath = resolveBinaryPath();
  const timeout
    = options.timeoutMs
      ?? (["init", "update", "rebuild"].includes(command.action)
      ? HEAVY_ACTION_TIMEOUT_MS
      : DEFAULT_TIMEOUT_MS);
  const output = execFileSync(binaryPath, ["--json"], {
    cwd: ROOT_DIR,
    input,
    encoding: "utf-8",
    timeout,
    maxBuffer: 50 * 1024 * 1024,
  });
  return parseCoreOutput(output);
}

/**
 * 通过 release binary 执行带 progress 的 JSON IPC 调用。
 *
 * @param command 要发送给 core 的命令对象。
 * @param options 运行选项；支持 progress 回调与超时覆盖。
 * @returns 返回终态响应和捕获到的 progress 事件。
 */
export async function callCoreStreaming(command, options = {}) {
  const binaryPath = resolveBinaryPath();
  const timeout
    = options.timeoutMs
      ?? (["init", "update", "rebuild"].includes(command.action)
      ? HEAVY_ACTION_TIMEOUT_MS
      : DEFAULT_TIMEOUT_MS);

  return await new Promise((resolve, reject) => {
    const child = spawn(binaryPath, ["--json"], {
      cwd: ROOT_DIR,
      stdio: ["pipe", "pipe", "pipe"],
    });

    let stdoutBuffer = "";
    let stderr = "";
    let terminal = null;
    const progressEvents = [];
    let timedOut = false;
    const timer = setTimeout(() => {
      timedOut = true;
      stderr = appendProcessMessage(stderr, `wiki-runtime ${command.action} timed out after ${timeout}ms`);
      terminateChildProcess(child, { killTreeOnWindows: true });
    }, timeout);
    timer.unref?.();

    child.stdout.on("data", (chunk) => {
      stdoutBuffer += chunk.toString();
      drainOutput(false);
    });
    child.stderr.on("data", (chunk) => {
      stderr += chunk.toString();
    });
    child.on("error", (error) => {
      clearTimeout(timer);
      reject(error);
    });
    child.on("close", (code) => {
      clearTimeout(timer);
      drainOutput(true);

      if (timedOut) {
        reject(new Error(stderr || `wiki-runtime ${command.action} timed out after ${timeout}ms`));
        return;
      }
      if (code !== 0) {
        reject(new Error(stderr || `wiki-runtime exited with code ${code}`));
        return;
      }
      if (!terminal) {
        reject(new Error("wiki-runtime stream ended without terminal response"));
        return;
      }
      resolve({ progressEvents, response: terminal });
    });

    child.stdin.end(`${JSON.stringify({ ...command, streamProgress: true })}\n`);

    function drainOutput(flushRemainder) {
      while (true) {
        const newlineIndex = stdoutBuffer.indexOf("\n");
        if (newlineIndex < 0) {
          break;
        }
        const line = stdoutBuffer.slice(0, newlineIndex).trim();
        stdoutBuffer = stdoutBuffer.slice(newlineIndex + 1);
        if (!line) {
          continue;
        }
        consumeEventLine(line);
      }

      if (flushRemainder && stdoutBuffer.trim()) {
        consumeEventLine(stdoutBuffer.trim());
        stdoutBuffer = "";
      }
    }

    function consumeEventLine(line) {
      const event = JSON.parse(line);
      if (event.type === "progress") {
        progressEvents.push(event);
        options.onProgress?.(event);
        return;
      }
      if ((event.type === "result" || event.type === "error") && event.response) {
        if (terminal) {
          throw new Error("wiki-runtime emitted multiple terminal events");
        }
        terminal = event.response;
        return;
      }
      throw new Error(`unexpected wiki-runtime event: ${line}`);
    }
  });
}

function parseCoreOutput(output) {
  const trimmed = output.trim();
  if (!trimmed) {
    throw new Error("wiki-runtime returned empty stdout");
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
        throw new Error("wiki-runtime emitted multiple terminal events");
      }
      terminal = parsed.response;
      continue;
    }
    throw new Error(`unexpected wiki-runtime event: ${line}`);
  }

  if (!terminal) {
    throw new Error("wiki-runtime stream ended without terminal response");
  }

  return terminal;
}

export function formatUsageSnapshot(usage) {
  if (!usage) {
    return "requests=0 in=0 out=0 total=0";
  }
  return [
    `requests=${usage.request_count ?? 0}`,
    `in=${usage.input_tokens ?? 0}`,
    `out=${usage.output_tokens ?? 0}`,
    `total=${usage.total_tokens ?? 0}`,
  ].join(" ");
}

export function withDevelopmentMode(command, enabled = false) {
  return enabled ? { ...command, developmentMode: true } : command;
}

export function applyCacheModeOverride(content, cacheMode) {
  if (!cacheMode) {
    return content;
  }

  if (!/^\s*llm:\s*$/m.test(content)) {
    return `llm:\n  cache_mode: ${cacheMode}\n\n${content}`;
  }

  if (/^\s+cache_mode:\s*\S+/m.test(content)) {
    return content.replace(/(^\s+cache_mode:\s*)\S+/m, `$1${cacheMode}`);
  }

  return content.replace(/(^\s*llm:\s*$)/m, `$1\n  cache_mode: ${cacheMode}`);
}

/**
 * 在外层临时 dev config 已经存在时，把 repo 根 `wiki.dev.yaml` 的 cache_mode
 * 同步到当前 workflow 实际采用的值，避免 resume 后 status 立即把配置文件判脏。
 *
 * @param repoRoot 目标仓库根目录。
 * @param options 可选覆盖；当前支持 `cacheMode`。
 */
export function syncTemporaryDevConfig(repoRoot, options = {}) {
  if (!existsSync(ROOT_DEV_CONFIG_PATH)) {
    return;
  }

  const targetPath = path.join(repoRoot, "wiki.dev.yaml");
  const source = readFileSync(ROOT_DEV_CONFIG_PATH, "utf-8");
  const next = applyCacheModeOverride(source, options.cacheMode);
  writeFileWithRetry(targetPath, next);
}

/**
 * 临时把根目录 `wiki.dev.yaml` 覆盖到目标 repo，并可附加 cache_mode。
 *
 * @param repoRoot 目标仓库根目录。
 * @param callback 需要在 dev 配置存在时执行的逻辑。
 * @param options 可选覆盖；当前支持 `cacheMode`。
 * @returns 返回回调结果。
 */
export async function withTemporaryDevConfig(repoRoot, callback, options = {}) {
  const developmentMode = existsSync(ROOT_DEV_CONFIG_PATH);
  const devContext = {
    developmentMode,
    command(command) {
      return withDevelopmentMode(command, developmentMode);
    },
  };

  if (!existsSync(ROOT_DEV_CONFIG_PATH)) {
    return await callback(devContext);
  }

  const targetPath = path.join(repoRoot, "wiki.dev.yaml");
  const source = readFileSync(ROOT_DEV_CONFIG_PATH, "utf-8");
  const previous = existsSync(targetPath) ? readFileSync(targetPath, "utf-8") : null;
  const next = applyCacheModeOverride(source, options.cacheMode);

  writeFileWithRetry(targetPath, next);
  try {
    return await callback(devContext);
  } finally {
    if (previous === null) {
      unlinkFileWithRetry(targetPath);
    } else {
      writeFileWithRetry(targetPath, previous);
    }
  }
}

// -------------------------------------------------------------------------
// 文件工具
// -------------------------------------------------------------------------

export function countMdFiles(dir) {
  if (!existsSync(dir)) {
    return 0;
  }

  let count = 0;
  const walk = (currentDir) => {
    for (const entry of readdirSync(currentDir, { withFileTypes: true })) {
      if (entry.isDirectory()) {
        walk(path.join(currentDir, entry.name));
      } else if (entry.name.endsWith(".md")) {
        count += 1;
      }
    }
  };

  walk(dir);
  return count;
}

export function countFilesWithMarker(dir) {
  if (!existsSync(dir)) {
    return 0;
  }

  let count = 0;
  const walk = (currentDir) => {
    for (const entry of readdirSync(currentDir, { withFileTypes: true })) {
      const fullPath = path.join(currentDir, entry.name);
      if (entry.isDirectory()) {
        walk(fullPath);
      } else if (entry.name.endsWith(".md")) {
        const content = readFileSync(fullPath, "utf-8");
        if (content.includes("<!-- wiki:managed:start")) {
          count += 1;
        }
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
  constructor(options = {}) {
    this.total = 0;
    this.passed = 0;
    this.failed = 0;
    this.failures = [];
    this.failureFactory = options.failureFactory ?? null;
    this.assertions = [];
    this.assertionFactory = options.assertionFactory ?? null;
  }

  pass(label) {
    this.total++;
    this.passed++;
    if (this.assertionFactory) {
      this.assertions.push(this.assertionFactory(label, "pass"));
    }
    console.log(`    \x1B[32mPASS\x1B[0m ${label}`);
  }

  fail(label, detail) {
    this.total++;
    this.failed++;
    if (this.failureFactory) {
      this.failures.push(this.failureFactory(label, detail));
    }
    if (this.assertionFactory) {
      this.assertions.push(this.assertionFactory(label, "fail", detail));
    }
    console.log(`    \x1B[31mFAIL\x1B[0m ${label}`);
    if (detail) {
      console.log(`         ${detail}`);
    }
  }

  skip(label) {
    console.log(`    \x1B[33mSKIP\x1B[0m ${label}`);
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
      `Total: ${this.total}  \x1B[32mPassed: ${this.passed}\x1B[0m  \x1B[31mFailed: ${this.failed}\x1B[0m`,
    );
    return this.failed === 0;
  }
}
