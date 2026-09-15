import { spawn } from "node:child_process";

export type ToolCommandResult = {
  code: number;
  stdout: string;
  stderr: string;
  timedOut?: boolean;
};

export type ToolCommandRunner = (
  command: string,
  args: string[],
  options: { cwd: string; env: NodeJS.ProcessEnv; timeoutMs?: number },
) => Promise<ToolCommandResult>;

const MAX_OUTPUT = 2 * 1024 * 1024;

export const runToolCommand: ToolCommandRunner = (command, args, options) => new Promise((resolve) => {
  const child = spawn(command, args, {
    cwd: options.cwd,
    env: options.env,
    shell: false,
    windowsHide: true,
  });
  let stdout = "";
  let stderr = "";
  let settled = false;
  const append = (current: string, chunk: unknown) => (current + String(chunk)).slice(-MAX_OUTPUT);
  child.stdout?.on("data", chunk => { stdout = append(stdout, chunk); });
  child.stderr?.on("data", chunk => { stderr = append(stderr, chunk); });
  const finish = (result: ToolCommandResult) => {
    if (settled) return;
    settled = true;
    if (timer) clearTimeout(timer);
    resolve(result);
  };
  child.on("error", error => finish({ code: 1, stdout, stderr: append(stderr, error.message) }));
  child.on("close", code => finish({ code: code ?? 1, stdout, stderr }));
  const timer = options.timeoutMs
    ? setTimeout(() => {
        child.kill();
        finish({ code: 1, stdout, stderr: append(stderr, "command timed out"), timedOut: true });
      }, options.timeoutMs)
    : undefined;
});

export async function invokeTool(
  runner: ToolCommandRunner,
  command: string,
  args: string[],
  options: { cwd: string; env: NodeJS.ProcessEnv; timeoutMs?: number },
): Promise<ToolCommandResult> {
  try {
    return await runner(command, args, options);
  } catch (error) {
    return { code: 1, stdout: "", stderr: error instanceof Error ? error.message : String(error) };
  }
}
