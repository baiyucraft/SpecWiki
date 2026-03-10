import { EventEmitter } from "node:events";

import { beforeEach, expect, test, vi } from "vitest";

const spawnMock = vi.fn();

vi.mock("node:child_process", () => ({
  spawn: spawnMock,
}));

vi.mock("./resolveBinary.js", () => ({
  resolveBinary: () => "wiki-core-test-bin",
}));

type MockChildProcess = EventEmitter & {
  stdout: EventEmitter;
  stderr: EventEmitter;
  stdin: {
    write: ReturnType<typeof vi.fn>;
    end: ReturnType<typeof vi.fn>;
  };
};

function createMockChild(lines: string[], code = 0, stderr = ""): MockChildProcess {
  const child = new EventEmitter() as MockChildProcess;
  child.stdout = new EventEmitter();
  child.stderr = new EventEmitter();
  child.stdin = {
    write: vi.fn(),
    end: vi.fn(() => {
      queueMicrotask(() => {
        for (const line of lines) {
          child.stdout.emit("data", Buffer.from(`${line}\n`));
        }
        if (stderr) {
          child.stderr.emit("data", Buffer.from(stderr));
        }
        child.emit("close", code);
      });
    }),
  };

  return child;
}

beforeEach(() => {
  spawnMock.mockReset();
});

test("invokeCore 逐行消费长流程进度并返回最终结果", async () => {
  const progressEvents: Array<{ phase: string; processed: number | null; total: number | null }> =
    [];
  const child = createMockChild([
    JSON.stringify({
      type: "progress",
      action: "init",
      phase: "parse_symbols",
      message: "解析源码符号 1/2",
      elapsed_ms: 3,
      processed: 1,
      total: 2,
    }),
    JSON.stringify({
      type: "progress",
      action: "init",
      phase: "render_pages",
      message: "渲染页面 1/1",
      elapsed_ms: 7,
      processed: 1,
      total: 1,
    }),
    JSON.stringify({
      type: "result",
      response: {
        ok: true,
        data: { initialized: true },
      },
    }),
  ]);
  spawnMock.mockReturnValue(child);

  const { invokeCore } = await import("./invokeCore.js");
  const result = await invokeCore(
    { action: "init", repoRoot: "demo-repo" },
    {
      onProgress: (event) =>
        progressEvents.push({
          phase: event.phase,
          processed: event.processed,
          total: event.total,
        }),
    },
  );

  expect(spawnMock).toHaveBeenCalledWith("wiki-core-test-bin", ["--json"], {
    stdio: ["pipe", "pipe", "pipe"],
  });
  expect(child.stdin.write).toHaveBeenCalledWith(
    JSON.stringify({
      action: "init",
      repoRoot: "demo-repo",
      streamProgress: true,
    }),
  );
  expect(progressEvents).toEqual([
    { phase: "parse_symbols", processed: 1, total: 2 },
    { phase: "render_pages", processed: 1, total: 1 },
  ]);
  expect(result).toEqual({
    ok: true,
    data: { initialized: true },
  });
});

test("invokeCore 在收到 error 终态事件时返回失败响应", async () => {
  const child = createMockChild([
    JSON.stringify({
      type: "progress",
      action: "update",
      phase: "plan_changes",
      message: "应用增量变更",
      elapsed_ms: 1,
      processed: null,
      total: null,
    }),
    JSON.stringify({
      type: "error",
      response: {
        ok: false,
        error: "write_state failed",
      },
    }),
  ]);
  spawnMock.mockReturnValue(child);

  const { invokeCore } = await import("./invokeCore.js");
  const result = await invokeCore({ action: "update", repoRoot: "demo-repo" });

  expect(result).toEqual({
    ok: false,
    error: "write_state failed",
  });
});

test("invokeCore 对短流程仍按单个最终 JSON 解析", async () => {
  const child = createMockChild([
    JSON.stringify({
      ok: true,
      data: { status: "fresh" },
    }),
  ]);
  spawnMock.mockReturnValue(child);

  const { invokeCore } = await import("./invokeCore.js");
  const result = await invokeCore({ action: "status", repoRoot: "demo-repo" });

  expect(child.stdin.write).toHaveBeenCalledWith(
    JSON.stringify({
      action: "status",
      repoRoot: "demo-repo",
    }),
  );
  expect(result).toEqual({
    ok: true,
    data: { status: "fresh" },
  });
});
