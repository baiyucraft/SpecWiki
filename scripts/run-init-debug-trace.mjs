/**
 * 运行单个仓库的 `init`，并把进度与 `wiki-core` debug trace 一起落盘。
 * 默认目标是 `tmp/test/storybook`，日志固定输出到当前 9.1 change 目录下。
 *
 * 用法：
 *   node scripts/run-init-debug-trace.mjs
 *   node scripts/run-init-debug-trace.mjs storybook
 *   node scripts/run-init-debug-trace.mjs --repo tmp/test/chi
 */

import { spawn } from "node:child_process";
import {
  appendFileSync,
  existsSync,
  mkdirSync,
  readFileSync,
  statSync,
  unlinkSync,
  writeFileSync,
} from "node:fs";
import path from "node:path";
import { fileURLToPath, pathToFileURL } from "node:url";

import { ROOT_DIR, TEST_DIR, removePathWithRetry } from "./testing/helpers.mjs";

const __filename = fileURLToPath(import.meta.url);
const BINARY_NAME = process.platform === "win32" ? "wiki-core.exe" : "wiki-core";
const DEBUG_BINARY = path.join(ROOT_DIR, "target", "debug", BINARY_NAME);
const RELEASE_BINARY = path.join(ROOT_DIR, "target", "release", BINARY_NAME);
const CHANGE_DIR = path.join(
  ROOT_DIR,
  "openspec",
  "changes",
  "iteration-9-1-topic-page-planner-and-evidence-layer",
);
const LOG_ROOT_DIR = path.join(CHANGE_DIR, "debug-init-traces");
const ROOT_DEV_CONFIG_PATH = path.join(ROOT_DIR, "wiki.dev.yaml");

function resolveBinaryPath() {
  if (!existsSync(DEBUG_BINARY) && !existsSync(RELEASE_BINARY)) {
    throw new Error(`缺少 wiki-core binary: ${DEBUG_BINARY} / ${RELEASE_BINARY}`);
  }
  if (!existsSync(DEBUG_BINARY)) {
    return RELEASE_BINARY;
  }
  if (!existsSync(RELEASE_BINARY)) {
    return DEBUG_BINARY;
  }

  return statSync(DEBUG_BINARY).mtimeMs >= statSync(RELEASE_BINARY).mtimeMs
    ? DEBUG_BINARY
    : RELEASE_BINARY;
}

function parseCliArgs(argv) {
  let project = "storybook";
  let repoRoot = null;

  for (let index = 0; index < argv.length; index++) {
    const arg = argv[index];
    if (arg === "--repo") {
      repoRoot = argv[index + 1];
      index++;
      continue;
    }
    project = arg;
  }

  return { project, repoRoot };
}

function resolveRepo(args) {
  if (args.repoRoot) {
    const absolute = path.isAbsolute(args.repoRoot)
      ? args.repoRoot
      : path.join(ROOT_DIR, args.repoRoot);
    return {
      label: path.basename(absolute),
      repoRoot: absolute,
      commandRepoRoot: absolute,
    };
  }

  const repoRoot = path.join(TEST_DIR, args.project);
  return {
    label: args.project,
    repoRoot,
    commandRepoRoot: `tmp/test/${args.project}`,
  };
}

function formatElapsed(elapsedMs) {
  if (elapsedMs < 1000) {
    return `${elapsedMs}ms`;
  }
  return `${(elapsedMs / 1000).toFixed(elapsedMs >= 10_000 ? 0 : 1)}s`;
}

function timestampSlug() {
  const now = new Date();
  const parts = [
    now.getFullYear(),
    String(now.getMonth() + 1).padStart(2, "0"),
    String(now.getDate()).padStart(2, "0"),
    String(now.getHours()).padStart(2, "0"),
    String(now.getMinutes()).padStart(2, "0"),
    String(now.getSeconds()).padStart(2, "0"),
  ];
  return `${parts[0]}${parts[1]}${parts[2]}-${parts[3]}${parts[4]}${parts[5]}`;
}

function ensureDir(dir) {
  mkdirSync(dir, { recursive: true });
}

function appendLine(filePath, line = "") {
  appendFileSync(filePath, `${line}\n`);
}

function formatProgressEvent(event) {
  const prefix = `[${formatElapsed(event.elapsed_ms)}] ${event.phase}`;
  if (event.processed != null && event.total != null) {
    return `${prefix} ${event.processed}/${event.total} ${event.message}`;
  }
  return `${prefix} ${event.message}`;
}

function createProgressFilter() {
  const countedPercents = new Map();
  const phaseMessages = new Map();

  return {
    shouldPrint(event) {
      if (event.processed != null && event.total != null && event.total > 0) {
        const percent = Math.floor((event.processed / event.total) * 100);
        const lastPercent = countedPercents.get(event.phase) ?? -1;
        const shouldPrint =
          event.processed === 0
          || event.processed === event.total
          || percent >= lastPercent + 10;
        if (shouldPrint) {
          countedPercents.set(event.phase, percent);
        }
        return shouldPrint;
      }

      if (phaseMessages.get(event.phase) === event.message) {
        return false;
      }
      phaseMessages.set(event.phase, event.message);
      return true;
    },
  };
}

function renderTraceEntry(entry) {
  const time = new Date(Number(entry.ts_ms || 0)).toISOString();
  return [
    `=== ${entry.kind} @ ${time} ===`,
    JSON.stringify(entry.payload, null, 2),
    "",
  ].join("\n");
}

async function withTemporaryDevConfig(repoRoot, callback) {
  if (!existsSync(ROOT_DEV_CONFIG_PATH)) {
    throw new Error(`缺少 root wiki.dev.yaml: ${ROOT_DEV_CONFIG_PATH}`);
  }

  const targetPath = path.join(repoRoot, "wiki.dev.yaml");
  const rootConfig = readFileSync(ROOT_DEV_CONFIG_PATH, "utf-8");
  const previous = existsSync(targetPath) ? readFileSync(targetPath, "utf-8") : null;

  writeFileSync(targetPath, rootConfig);
  try {
    return await callback();
  } finally {
    if (previous === null) {
      unlinkSync(targetPath);
    } else {
      writeFileSync(targetPath, previous);
    }
  }
}

async function runInitTrace(target) {
  const binary = resolveBinaryPath();
  const runDir = path.join(LOG_ROOT_DIR, `${target.label}-${timestampSlug()}`);
  const sessionLogPath = path.join(runDir, "session.log");
  const ipcLogPath = path.join(runDir, "ipc.ndjson");
  const tracePath = path.join(runDir, "trace.ndjson");
  const tracePrettyPath = path.join(runDir, "trace.pretty.log");
  const summaryPath = path.join(runDir, "summary.json");
  const wikiDir = path.join(target.repoRoot, ".wiki");

  ensureDir(runDir);
  removePathWithRetry(wikiDir);
  writeFileSync(
    path.join(runDir, "command.json"),
    `${JSON.stringify(
      {
        binary,
        args: ["--json", "--debug-trace-dir", runDir],
        command: {
          action: "init",
          repoRoot: target.commandRepoRoot,
          streamProgress: true,
        },
      },
      null,
      2,
    )}\n`,
  );
  appendLine(sessionLogPath, `repo=${target.repoRoot}`);
  appendLine(sessionLogPath, `binary=${binary}`);
  appendLine(sessionLogPath, `run_dir=${runDir}`);
  appendLine(sessionLogPath);
  const progressFilter = createProgressFilter();

  const terminal = await withTemporaryDevConfig(target.repoRoot, async () => {
    return await new Promise((resolve, reject) => {
      const child = spawn(
        binary,
        ["--json", "--debug-trace-dir", runDir],
        {
          cwd: ROOT_DIR,
          stdio: ["pipe", "pipe", "pipe"],
        },
      );

      let stdoutBuffer = "";
      let stderrBuffer = "";
      let terminalEvent = null;

      child.stdout.on("data", (chunk) => {
        stdoutBuffer += chunk.toString();
        drainStdout(false);
      });

      child.stderr.on("data", (chunk) => {
        stderrBuffer += chunk.toString();
      });

      child.on("error", reject);
      child.on("close", (code) => {
        drainStdout(true);
        if (stderrBuffer.trim()) {
          writeFileSync(path.join(runDir, "stderr.log"), stderrBuffer);
        }
        if (code !== 0) {
          reject(new Error(stderrBuffer || `wiki-core exited with code ${code}`));
          return;
        }
        if (!terminalEvent) {
          reject(new Error("wiki-core 输出中缺少终态事件"));
          return;
        }
        resolve(terminalEvent);
      });

      child.stdin.end(
        `${JSON.stringify({
          action: "init",
          repoRoot: target.commandRepoRoot,
          streamProgress: true,
        })}\n`,
      );

      function drainStdout(flushRemainder) {
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
          consumeStdoutLine(line);
        }

        if (flushRemainder && stdoutBuffer.trim()) {
          consumeStdoutLine(stdoutBuffer.trim());
          stdoutBuffer = "";
        }
      }

      function consumeStdoutLine(line) {
        appendLine(ipcLogPath, line);
        const event = JSON.parse(line);
        if (event.type === "progress") {
          if (!progressFilter.shouldPrint(event)) {
            return;
          }
          const formatted = formatProgressEvent(event);
          console.log(formatted);
          appendLine(sessionLogPath, formatted);
          return;
        }

        terminalEvent = event;
        const formatted = `terminal=${JSON.stringify(event.response)}`;
        console.log(formatted);
        appendLine(sessionLogPath, formatted);
      }
    });
  });

  if (existsSync(tracePath)) {
    const prettyTrace = readFileSync(tracePath, "utf-8")
      .split(/\r?\n/)
      .filter(Boolean)
      .map((line) => renderTraceEntry(JSON.parse(line)))
      .join("\n");
    writeFileSync(tracePrettyPath, prettyTrace);
  }

  writeFileSync(
    summaryPath,
    `${JSON.stringify(
      {
        ok: terminal.response?.ok ?? false,
        repoRoot: target.repoRoot,
        runDir,
        sessionLogPath,
        ipcLogPath,
        tracePath: existsSync(tracePath) ? tracePath : null,
        tracePrettyPath: existsSync(tracePrettyPath) ? tracePrettyPath : null,
      },
      null,
      2,
    )}\n`,
  );

  return {
    runDir,
    sessionLogPath,
    ipcLogPath,
    tracePath: existsSync(tracePath) ? tracePath : null,
    tracePrettyPath: existsSync(tracePrettyPath) ? tracePrettyPath : null,
    summaryPath,
    terminal,
  };
}

async function main(argv) {
  const args = parseCliArgs(argv);
  const target = resolveRepo(args);
  if (!existsSync(target.repoRoot)) {
    throw new Error(`目标仓库不存在: ${target.repoRoot}`);
  }

  const result = await runInitTrace(target);
  process.stdout.write(
    `${JSON.stringify(
      {
        ok: result.terminal.response?.ok ?? false,
        runDir: result.runDir,
        sessionLogPath: result.sessionLogPath,
        ipcLogPath: result.ipcLogPath,
        tracePath: result.tracePath,
        tracePrettyPath: result.tracePrettyPath,
        summaryPath: result.summaryPath,
      },
      null,
      2,
    )}\n`,
  );
}

if (import.meta.url === pathToFileURL(process.argv[1]).href) {
  await main(process.argv.slice(2));
}
