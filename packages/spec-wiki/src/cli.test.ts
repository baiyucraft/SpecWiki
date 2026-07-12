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

test("runCli dispatches unified init with explicit host and repo root", async () => {
  selectHostsForInitMock.mockResolvedValue(["claude"]);
  runBootstrapInitMock.mockResolvedValue({ outcome: "ready", hosts: [] });
  const stdout: string[] = [];
  const stderr: string[] = [];
  const { runCli } = await import("./cli.js");

  const exitCode = await runCli(
    ["init", "--host", "claude", "--repo-root", "/repo", "--json"],
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
    rawHosts: "claude",
    interactive: false,
    stdin: undefined,
  });
  expect(runBootstrapInitMock).toHaveBeenCalledWith({
    repoRoot: "/repo",
    hosts: "claude",
    env: process.env,
  });
  expect(stderr).toEqual([]);
  expect(stdout).toEqual([]);
});

test("runCli supports interactive init selection without --hosts", async () => {
  selectHostsForInitMock.mockResolvedValue(["codex", "codebuddy"]);
  runBootstrapInitMock.mockResolvedValue({ outcome: "ready", hosts: [] });
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
    rawHosts: undefined,
    interactive: true,
    stdin: undefined,
  });
  expect(runBootstrapInitMock).toHaveBeenCalledWith({
    repoRoot: "/repo",
    hosts: "codex,codebuddy",
    env: process.env,
  });
});

test("runCli passes --no-interactive to host selection", async () => {
  selectHostsForInitMock.mockResolvedValue(["codebuddy"]);
  runBootstrapInitMock.mockResolvedValue({ outcome: "ready", hosts: [] });
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
    rawHosts: undefined,
    interactive: false,
    stdin: undefined,
  });
});

test("runCli dispatches top-level query with positional terms", async () => {
  forwardCoreCommandMock.mockResolvedValue(0);
  const { runCli } = await import("./cli.js");

  const exitCode = await runCli(["query", "payment", "flow"], {
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

test("runCli dispatches top-level sync as a public short-running action", async () => {
  forwardCoreCommandMock.mockResolvedValue(0);
  const { runCli } = await import("./cli.js");

  const exitCode = await runCli(["sync", "--repo-root", "/repo"], {
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

test("runCli dispatches top-level rebuild as a public streaming action", async () => {
  forwardCoreCommandMock.mockResolvedValue(0);
  const { runCli } = await import("./cli.js");

  const exitCode = await runCli(["rebuild", "--bridge-stdio"], {
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

test("runCli rejects bridge-stdio for non-streaming actions", async () => {
  const stdout: string[] = [];
  const { runCli } = await import("./cli.js");

  const exitCode = await runCli(["query", "payment", "flow", "--bridge-stdio"], {
    cwd: "/repo",
    env: process.env,
    stdout: (text) => stdout.push(text),
    stderr: vi.fn(),
  });

  expect(exitCode).toBe(64);
  expect(stdout.join("")).toContain("invalid_argument");
  expect(forwardCoreCommandMock).not.toHaveBeenCalled();
});

test("runCli rejects bridge-stdio for status as well", async () => {
  const stdout: string[] = [];
  const { runCli } = await import("./cli.js");

  const exitCode = await runCli(["status", "--bridge-stdio"], {
    cwd: "/repo",
    env: process.env,
    stdout: (text) => stdout.push(text),
    stderr: vi.fn(),
  });

  expect(exitCode).toBe(64);
  expect(stdout.join("")).toContain("--bridge-stdio is only supported");
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
  expect(stdout.join("")).toContain("spec-wiki init");
  expect(stdout.join("")).toContain("spec-wiki status");
  expect(stdout.join("")).not.toContain("spec-wiki wiki");
  expect(stdout.join("")).not.toContain("archive");
});

test("runCli exposes implemented advanced commands only through --help-all", async () => {
  const stdout: string[] = [];
  const { runCli } = await import("./cli.js");

  const exitCode = await runCli(["--help-all"], {
    cwd: "/repo",
    env: process.env,
    stdout: (text) => stdout.push(text),
    stderr: vi.fn(),
  });

  expect(exitCode).toBe(0);
  expect(stdout.join("")).toContain("spec-wiki changes");
  expect(stdout.join("")).toContain("spec-wiki validate <change-id>");
  expect(stdout.join("")).not.toContain("archive");
});

test("runCli rejects help mixed with machine flags", async () => {
  const stdout: string[] = [];
  const { runCli } = await import("./cli.js");
  expect(await runCli(["--help", "--json"], {
    cwd: "/repo",
    env: process.env,
    stdout: (text) => stdout.push(text),
    stderr: vi.fn(),
  })).toBe(64);
  expect(stdout.join("")).toContain("invalid_argument");
});

test("runCli rejects the legacy wiki namespace with usage exit code", async () => {
  const { runCli } = await import("./cli.js");
  expect(await runCli(["wiki", "status"], {
    cwd: "/repo",
    env: process.env,
    stdout: vi.fn(),
    stderr: vi.fn(),
  })).toBe(64);
});

test("runCli rejects query without a term", async () => {
  const stderr: string[] = [];
  const { runCli } = await import("./cli.js");

  const exitCode = await runCli(["query"], {
    cwd: "/repo",
    env: process.env,
    stdout: vi.fn(),
    stderr: (text) => stderr.push(text),
  });

  expect(exitCode).toBe(64);
  expect(stderr.join("")).toContain("query requires at least one term");
});
