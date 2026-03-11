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

function createSessionChild(
  onAgentMessage: (payload: unknown, child: MockChildProcess) => void,
): MockChildProcess {
  const child = new EventEmitter() as MockChildProcess;
  child.stdout = new EventEmitter();
  child.stderr = new EventEmitter();
  let wroteCommand = false;

  child.stdin = {
    write: vi.fn((payload: string) => {
      const trimmed = payload.trim();
      if (!wroteCommand) {
        wroteCommand = true;
        queueMicrotask(() => {
          child.stdout.emit(
            "data",
            Buffer.from(
              `${JSON.stringify({
                type: "llm_request",
                request: {
                  request_id: "req-1",
                  prompt_type: "page_enrichment",
                  prompt_version: "page-enrichment/v1",
                  input_hash: "hash-1",
                  system: "system",
                  instruction: "instruction",
                  input: { page_id: "page-1" },
                  response_schema: { type: "object" },
                },
              })}\n`,
            ),
          );
        });
        return;
      }

      onAgentMessage(JSON.parse(trimmed), child);
    }),
    end: vi.fn(),
  };

  return child;
}

function createStreamingChild(lines: string[], code = 0): MockChildProcess {
  const child = new EventEmitter() as MockChildProcess;
  child.stdout = new EventEmitter();
  child.stderr = new EventEmitter();
  let wroteCommand = false;

  child.stdin = {
    write: vi.fn(() => {
      if (wroteCommand) {
        return;
      }
      wroteCommand = true;
      queueMicrotask(() => {
        for (const line of lines) {
          child.stdout.emit("data", Buffer.from(`${line}\n`));
        }
        child.emit("close", code);
      });
    }),
    end: vi.fn(),
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
    `${JSON.stringify({
      action: "init",
      repoRoot: "demo-repo",
      streamProgress: true,
    })}\n`,
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

test("invokeCore 在收到 llm_request 时桥接宿主响应", async () => {
  const child = createSessionChild((payload, currentChild) => {
    expect(payload).toEqual({
      type: "llm_response",
      requestId: "req-1",
      response: {
        output: { summary: "enhanced" },
        model: "mock-model",
      },
    });

    queueMicrotask(() => {
      currentChild.stdout.emit(
        "data",
        Buffer.from(
          `${JSON.stringify({
            type: "result",
            response: { ok: true, data: { bridged: true } },
          })}\n`,
        ),
      );
      currentChild.emit("close", 0);
    });
  });
  spawnMock.mockReturnValue(child);

  const { invokeCore } = await import("./invokeCore.js");
  const result = await invokeCore(
    { action: "init", repoRoot: "demo-repo" },
    {
      llmBridge: {
        request: async () => ({
          output: { summary: "enhanced" },
          model: "mock-model",
        }),
      },
    },
  );

  expect(child.stdin.write).toHaveBeenNthCalledWith(
    1,
    `${JSON.stringify({
      action: "init",
      repoRoot: "demo-repo",
      streamProgress: true,
      llmBridge: { protocol: "ndjson_session_v1" },
    })}\n`,
  );
  expect(result).toEqual({
    ok: true,
    data: { bridged: true },
  });
});

test("invokeCore 在 core 直接走 provider 时不追加 llm 会话写入", async () => {
  const child = createStreamingChild([
    JSON.stringify({
      type: "progress",
      action: "init",
      phase: "llm_enrichment",
      message: "生成页面增强内容",
      elapsed_ms: 2,
      processed: null,
      total: null,
    }),
    JSON.stringify({
      type: "result",
      response: {
        ok: true,
        data: { providerDirect: true },
      },
    }),
  ]);
  spawnMock.mockReturnValue(child);

  const { invokeCore } = await import("./invokeCore.js");
  const result = await invokeCore(
    { action: "init", repoRoot: "demo-repo" },
    {
      llmBridge: {
        request: async () => ({
          output: { summary: "should-not-run" },
          model: "unused-model",
        }),
      },
    },
  );

  expect(child.stdin.write).toHaveBeenCalledTimes(1);
  expect(result).toEqual({
    ok: true,
    data: { providerDirect: true },
  });
});

test("invokeCore 在宿主未提供结果时回写 llm_unavailable", async () => {
  const child = createSessionChild((payload, currentChild) => {
    expect(payload).toEqual({
      type: "llm_unavailable",
      requestId: "req-1",
      reason: "agent_llm_bridge_returned_empty",
    });

    queueMicrotask(() => {
      currentChild.stdout.emit(
        "data",
        Buffer.from(
          `${JSON.stringify({
            type: "result",
            response: { ok: true, data: { fallback: true } },
          })}\n`,
        ),
      );
      currentChild.emit("close", 0);
    });
  });
  spawnMock.mockReturnValue(child);

  const { invokeCore } = await import("./invokeCore.js");
  const result = await invokeCore(
    { action: "init", repoRoot: "demo-repo" },
    {
      llmBridge: {
        request: async () => null,
      },
    },
  );

  expect(result).toEqual({
    ok: true,
    data: { fallback: true },
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
