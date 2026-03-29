/**
 * 这个文件覆盖已发布 CLI 入口对 runCli 的绑定行为。
 * 它防止 bridge-stdio 因为 stdin 丢失而在发布包里失效。
 */
import { beforeEach, expect, test, vi } from "vitest";

const runCliMock = vi.fn();

vi.mock("./cli.js", () => ({
  runCli: runCliMock,
}));

beforeEach(() => {
  runCliMock.mockReset();
  process.exitCode = undefined;
});

test("runBin forwards stdin to runCli for published CLI bridge mode", async () => {
  runCliMock.mockResolvedValue(7);
  const stdin = {} as NodeJS.ReadStream;
  const stdout = vi.fn();
  const stderr = vi.fn();
  const { runBin } = await import("./bin.js");

  const exitCode = await runBin({
    argv: ["node", "spec-wiki", "wiki", "update", "--bridge-stdio"],
    cwd: "/repo",
    env: process.env,
    stdin,
    stdout,
    stderr,
  });

  expect(exitCode).toBe(7);
  expect(process.exitCode).toBe(7);
  expect(runCliMock).toHaveBeenCalledWith(
    ["wiki", "update", "--bridge-stdio"],
    {
      cwd: "/repo",
      env: process.env,
      stdin,
      stdout,
      stderr,
    },
  );
});
