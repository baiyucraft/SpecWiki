/**
 * 这个文件覆盖 spec-wiki CLI 的参数解析和命令分发行为。
 * 它保护顶层 init 与 wiki <action> 两层命令面的边界。
 */
import { beforeEach, expect, test, vi } from "vitest";

const runBootstrapInitMock = vi.fn();
const selectHostsForInitMock = vi.fn();
const forwardCoreCommandMock = vi.fn();

vi.mock("./orchestration/init/runInit.js", () => ({
  runBootstrapInit: runBootstrapInitMock,
}));

vi.mock("./orchestration/init/selectHosts.js", () => ({
  selectHostsForInit: selectHostsForInitMock,
}));

vi.mock("./runtime/forwardCore.js", () => ({
  forwardCoreCommand: forwardCoreCommandMock,
}));

beforeEach(() => {
  runBootstrapInitMock.mockReset();
  selectHostsForInitMock.mockReset();
  forwardCoreCommandMock.mockReset();
});

test("runCli dispatches init with explicit tool and repo root", async () => {
  selectHostsForInitMock.mockResolvedValue(["claude"]);
  runBootstrapInitMock.mockResolvedValue({ hosts: [] });
  const stdout: string[] = [];
  const stderr: string[] = [];
  const { runCli } = await import("./cli.js");

  const exitCode = await runCli(
    ["init", "--tool", "claude", "--repo-root", "/repo"],
    {
      cwd: "/cwd",
      env: process.env,
      stdout: (text) => stdout.push(text),
      stderr: (text) => stderr.push(text),
    },
  );

  expect(exitCode).toBe(0);
  expect(selectHostsForInitMock).toHaveBeenCalledWith({
    repoRoot: "/repo",
    rawTools: "claude",
    interactive: undefined,
    stdin: undefined,
  });
  expect(runBootstrapInitMock).toHaveBeenCalledWith({
    repoRoot: "/repo",
    tools: "claude",
    env: process.env,
  });
  expect(stderr).toEqual([]);
  expect(stdout).toEqual([]);
});

test("runCli supports interactive init selection without --tools", async () => {
  selectHostsForInitMock.mockResolvedValue(["codex", "codebuddy"]);
  runBootstrapInitMock.mockResolvedValue({ hosts: [] });
  const { runCli } = await import("./cli.js");

  const exitCode = await runCli(["init"], {
    cwd: "/repo",
    env: process.env,
    interactive: true,
    stdout: vi.fn(),
    stderr: vi.fn(),
  });

  expect(exitCode).toBe(0);
  expect(selectHostsForInitMock).toHaveBeenCalledWith({
    repoRoot: "/repo",
    rawTools: undefined,
    interactive: true,
    stdin: undefined,
  });
  expect(runBootstrapInitMock).toHaveBeenCalledWith({
    repoRoot: "/repo",
    tools: "codex,codebuddy",
    env: process.env,
  });
});

test("runCli passes --no-interactive to host selection", async () => {
  selectHostsForInitMock.mockResolvedValue(["codebuddy"]);
  runBootstrapInitMock.mockResolvedValue({ hosts: [] });
  const { runCli } = await import("./cli.js");

  const exitCode = await runCli(["init", "--no-interactive"], {
    cwd: "/repo",
    env: process.env,
    interactive: true,
    stdout: vi.fn(),
    stderr: vi.fn(),
  });

  expect(exitCode).toBe(0);
  expect(selectHostsForInitMock).toHaveBeenCalledWith({
    repoRoot: "/repo",
    rawTools: undefined,
    interactive: false,
    stdin: undefined,
  });
});

test("runCli dispatches wiki query with positional terms", async () => {
  forwardCoreCommandMock.mockResolvedValue(0);
  const { runCli } = await import("./cli.js");

  const exitCode = await runCli(["wiki", "query", "payment", "flow"], {
    cwd: "/repo",
    env: process.env,
    stdout: vi.fn(),
    stderr: vi.fn(),
  });

  expect(exitCode).toBe(0);
  expect(forwardCoreCommandMock).toHaveBeenCalledWith(
    {
      action: "query",
      repoRoot: "/repo",
      term: "payment flow",
    },
    expect.objectContaining({
      bridgeStdio: false,
      cwd: "/repo",
      env: process.env,
    }),
  );
});

test("runCli dispatches wiki sync as a public short-running action", async () => {
  forwardCoreCommandMock.mockResolvedValue(0);
  const { runCli } = await import("./cli.js");

  const exitCode = await runCli(["wiki", "sync", "--repo-root", "/repo"], {
    cwd: "/cwd",
    env: process.env,
    stdout: vi.fn(),
    stderr: vi.fn(),
  });

  expect(exitCode).toBe(0);
  expect(forwardCoreCommandMock).toHaveBeenCalledWith(
    {
      action: "sync",
      repoRoot: "/repo",
    },
    expect.objectContaining({
      bridgeStdio: false,
      cwd: "/cwd",
      env: process.env,
    }),
  );
});

test("runCli dispatches wiki rebuild as a public streaming action", async () => {
  forwardCoreCommandMock.mockResolvedValue(0);
  const { runCli } = await import("./cli.js");

  const exitCode = await runCli(["wiki", "rebuild", "--bridge-stdio"], {
    cwd: "/repo",
    env: process.env,
    stdout: vi.fn(),
    stderr: vi.fn(),
  });

  expect(exitCode).toBe(0);
  expect(forwardCoreCommandMock).toHaveBeenCalledWith(
    {
      action: "rebuild",
      repoRoot: "/repo",
    },
    expect.objectContaining({
      bridgeStdio: true,
      cwd: "/repo",
      env: process.env,
    }),
  );
});

test("runCli rejects bridge-stdio for non-streaming wiki actions", async () => {
  const stderr: string[] = [];
  const { runCli } = await import("./cli.js");

  const exitCode = await runCli(["wiki", "query", "payment", "flow", "--bridge-stdio"], {
    cwd: "/repo",
    env: process.env,
    stdout: vi.fn(),
    stderr: (text) => stderr.push(text),
  });

  expect(exitCode).toBe(1);
  expect(stderr.join("")).toContain("--bridge-stdio is only supported for long-running wiki actions");
  expect(forwardCoreCommandMock).not.toHaveBeenCalled();
});

test("runCli rejects bridge-stdio for wiki status as well", async () => {
  const stderr: string[] = [];
  const { runCli } = await import("./cli.js");

  const exitCode = await runCli(["wiki", "status", "--bridge-stdio"], {
    cwd: "/repo",
    env: process.env,
    stdout: vi.fn(),
    stderr: (text) => stderr.push(text),
  });

  expect(exitCode).toBe(1);
  expect(stderr.join("")).toContain("--bridge-stdio is only supported for long-running wiki actions");
  expect(stderr.join("")).toContain("status does not stream");
  expect(forwardCoreCommandMock).not.toHaveBeenCalled();
});

test("runCli prints usage for --help", async () => {
  const stdout: string[] = [];
  const { runCli } = await import("./cli.js");

  const exitCode = await runCli(["--help"], {
    cwd: "/repo",
    env: process.env,
    stdout: (text) => stdout.push(text),
    stderr: vi.fn(),
  });

  expect(exitCode).toBe(0);
  expect(stdout.join("")).toContain("Usage:");
  expect(stdout.join("")).toContain("--no-interactive");
  expect(stdout.join("")).toContain("Supported actions: init, status, update, query, sync, rebuild");
  expect(stdout.join("")).toContain("--bridge-stdio only applies to long-running wiki actions such as init, update, and rebuild");
});

test("runCli rejects query without a term", async () => {
  const stderr: string[] = [];
  const { runCli } = await import("./cli.js");

  const exitCode = await runCli(["wiki", "query"], {
    cwd: "/repo",
    env: process.env,
    stdout: vi.fn(),
    stderr: (text) => stderr.push(text),
  });

  expect(exitCode).toBe(1);
  expect(stderr.join("")).toContain("wiki query requires --term");
});
