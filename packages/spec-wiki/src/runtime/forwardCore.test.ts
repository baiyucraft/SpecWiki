/**
 * 这个文件覆盖 CLI 侧 runtime passthrough 行为。
 * 它保护 spec-wiki wiki <action> 的 JSON / NDJSON 输出合同与 stdin bridge 转发。
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

function createChild(stdoutChunks: string[], stderrChunks: string[] = [], code = 0): MockChildProcess {
  const child = new EventEmitter() as MockChildProcess;
  child.stdout = new EventEmitter();
  child.stderr = new EventEmitter();
  child.stdin = {
    write: vi.fn(),
    end: vi.fn(() => {
      queueMicrotask(() => {
        for (const chunk of stdoutChunks) {
          child.stdout.emit("data", Buffer.from(chunk));
        }
        for (const chunk of stderrChunks) {
          child.stderr.emit("data", Buffer.from(chunk));
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

test("forwardCoreCommand 透传短流程 JSON 输出", async () => {
  const stdout: string[] = [];
  const stderr: string[] = [];
  const child = createChild(["{\"ok\":true}\n"]);
  spawnMock.mockReturnValue(child);

  const { forwardCoreCommand } = await import("./forwardCore.js");
  const exitCode = await forwardCoreCommand(
    { action: "status", repoRoot: "demo-repo" },
    {
      stdout: (text) => stdout.push(text),
      stderr: (text) => stderr.push(text),
    },
  );

  expect(exitCode).toBe(0);
  expect(child.stdin.write).toHaveBeenCalledWith(
    JSON.stringify({ action: "status", repoRoot: "demo-repo" }),
  );
  expect(spawnMock.mock.calls[0][2].env?.SPEC_WIKI_V0_1_INDEX_ONLY).toBeUndefined();
  expect(stdout.join("")).toBe("{\"ok\":true}\n");
  expect(stderr).toEqual([]);
});

test("forwardCoreCommand 在短流程 ok=false 时返回非零退出码", async () => {
  const child = createChild(["{\"ok\":false,\"error\":\"broken\"}\n"]);
  spawnMock.mockReturnValue(child);

  const { forwardCoreCommand } = await import("./forwardCore.js");
  const exitCode = await forwardCoreCommand({ action: "status", repoRoot: "demo-repo" });

  expect(exitCode).toBe(1);
});

test("forwardCoreCommand 为长流程透传 NDJSON 并根据 terminal event 决定退出码", async () => {
  const stdout: string[] = [];
  const child = createChild([
    "{\"type\":\"progress\",\"action\":\"init\",\"phase\":\"scan\",\"message\":\"扫描\",\"elapsed_ms\":1,\"processed\":0,\"total\":1}\n",
    "{\"type\":\"result\",\"response\":{\"ok\":true,\"data\":{\"state\":\"index_only\"}}}\n",
  ]);
  spawnMock.mockReturnValue(child);

  const { forwardCoreCommand } = await import("./forwardCore.js");
  const exitCode = await forwardCoreCommand(
    { action: "init", repoRoot: "demo-repo" },
    {
      stdout: (text) => stdout.push(text),
    },
  );

  expect(exitCode).toBe(0);
  expect(child.stdin.write).toHaveBeenCalledWith(
    `${JSON.stringify({
      action: "init",
      repoRoot: "demo-repo",
      streamProgress: true,
    })}\n`,
  );
  expect(spawnMock.mock.calls[0][2].env?.SPEC_WIKI_V0_1_INDEX_ONLY).toBe("1");
  expect(stdout.join("")).toContain("\"type\":\"progress\"");
  expect(stdout.join("")).toContain("\"type\":\"result\"");
});

test("forwardCoreCommand 在长流程 terminal error 时返回非零退出码", async () => {
  const child = createChild([
    "{\"type\":\"progress\",\"action\":\"update\",\"phase\":\"scan\",\"message\":\"扫描\",\"elapsed_ms\":1,\"processed\":0,\"total\":1}\n",
    "{\"type\":\"error\",\"response\":{\"ok\":false,\"error\":\"broken\"}}\n",
  ]);
  spawnMock.mockReturnValue(child);

  const { forwardCoreCommand } = await import("./forwardCore.js");
  const exitCode = await forwardCoreCommand({ action: "update", repoRoot: "demo-repo" });

  expect(exitCode).toBe(1);
});

test("forwardCoreCommand 在 bridge-stdio 模式下打开 llmBridge 并转发 stdin", async () => {
  const stdinEmitter = new EventEmitter();
  const stdin = {
    resume: vi.fn(),
    on: ((eventName, listener) => {
      stdinEmitter.on(eventName, listener);
      return stdin;
    }) as NodeJS.ReadStream["on"],
    off: ((eventName, listener) => {
      stdinEmitter.off(eventName, listener);
      return stdin;
    }) as NodeJS.ReadStream["off"],
  } as unknown as NodeJS.ReadStream;
  const child = createChild([
    "{\"type\":\"result\",\"response\":{\"ok\":true,\"data\":{\"state\":\"index_only\"}}}\n",
  ]);
  spawnMock.mockReturnValue(child);

  const { forwardCoreCommand } = await import("./forwardCore.js");
  const promise = forwardCoreCommand(
    { action: "update", repoRoot: "demo-repo" },
    {
      bridgeStdio: true,
      stdin,
    },
  );

  stdinEmitter.emit("data", Buffer.from("{\"type\":\"llm_response\"}\n"));
  stdinEmitter.emit("end");

  const exitCode = await promise;
  expect(exitCode).toBe(0);
  expect(child.stdin.write).toHaveBeenNthCalledWith(
    1,
    `${JSON.stringify({
      action: "update",
      repoRoot: "demo-repo",
      streamProgress: true,
      llmBridge: { protocol: "ndjson_session_v1" },
    })}\n`,
  );
  expect(child.stdin.write).toHaveBeenNthCalledWith(2, Buffer.from("{\"type\":\"llm_response\"}\n"));
  expect(spawnMock.mock.calls[0][2].env?.SPEC_WIKI_V0_1_INDEX_ONLY).toBe("1");
  expect(stdin.resume).toHaveBeenCalled();
});
