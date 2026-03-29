/**
 * 这个文件覆盖 Node 侧 invokeCore 的 JSON IPC、长流程事件和 LLM bridge。
 * 它保护共享 runtime 调用层不会吞掉 progress、终态或会话协商。
 */
import { Buffer } from "node:buffer";
import { EventEmitter } from "node:events";

import { beforeEach, expect, test, vi } from "vitest";

const spawnMock = vi.fn();

vi.mock("node:child_process", () => ({
  spawn: spawnMock,
}));

vi.mock("./resolveBinary.js", () => ({
  resolveBinary: () => "wiki-runtime-test-bin",
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

function _createStreamingChild(lines: string[], code = 0): MockChildProcess {
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
  const progressEvents: Array<{ phase: string; processed: number | null; total: number | null }>
    = [];
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

  expect(spawnMock).toHaveBeenCalledWith("wiki-runtime-test-bin", ["--json"], expect.any(Object));
  expect(spawnMock.mock.calls[0][2]).toMatchObject({
    stdio: ["pipe", "pipe", "pipe"],
    env: expect.objectContaining({ SPEC_WIKI_V0_1_INDEX_ONLY: "1" }),
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
  expect(spawnMock.mock.calls[0][2].env?.SPEC_WIKI_V0_1_INDEX_ONLY).toBe("1");
  expect(result).toEqual({
    ok: true,
    data: { bridged: true },
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
        data: {
          blocker_hint: "compose_leaf:payment-flow:provider timeout",
          runtime_summary: {
            workflow_action: "update",
            runtime_state: "interrupted",
            researched_units: 2,
            compose_ready_units: 1,
            composed_units: 0,
            assembled_pages: 0,
            blocked_units: ["payment-flow"],
          },
        },
      },
    }),
  ]);
  spawnMock.mockReturnValue(child);

  const { invokeCore } = await import("./invokeCore.js");
  const result = await invokeCore({ action: "update", repoRoot: "demo-repo" });

  expect(result).toEqual({
    ok: false,
    error: "write_state failed",
    data: {
      blocker_hint: "compose_leaf:payment-flow:provider timeout",
      runtime_summary: {
        workflow_action: "update",
        runtime_state: "interrupted",
        researched_units: 2,
        compose_ready_units: 1,
        composed_units: 0,
        assembled_pages: 0,
        blocked_units: ["payment-flow"],
        last_ready_stage: undefined,
        summary_reason: undefined,
      },
    },
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
  expect(spawnMock.mock.calls[0][2].env?.SPEC_WIKI_V0_1_INDEX_ONLY).toBeUndefined();
  expect(result).toEqual({
    ok: true,
    data: { status: "fresh" },
  });
});
