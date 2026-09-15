/**
 * 这个文件负责已发布 CLI 可执行入口的薄封装。
 * 它只把 Node 进程的 argv/stdin/stdout/stderr 绑定到 runCli。
 */
import { runCli } from "./cli.js";

export type RunBinOptions = {
  /** 完整 argv；默认读取当前进程。 */
  argv?: string[];
  /** 当前工作目录；默认读取当前进程。 */
  cwd?: string;
  /** 当前环境变量；默认读取当前进程。 */
  env?: NodeJS.ProcessEnv;
  /** 传给 runCli 的标准输入。 */
  stdin?: NodeJS.ReadStream;
  /** 传给 runCli 的标准输出写入函数。 */
  stdout?: (text: string) => void;
  /** 传给 runCli 的标准错误写入函数。 */
  stderr?: (text: string) => void;
  /** 是否允许交互式终端；当前保留给后续提示扩展。 */
  interactive?: boolean;
};

/**
 * 运行已发布的 `spec-wiki-lite` 可执行入口。
 *
 * @param options 可选覆盖项，主要给测试使用。
 * @returns 返回 CLI 最终退出码。
 */
export async function runBin(options: RunBinOptions = {}): Promise<number> {
  const exitCode = await runCli((options.argv ?? process.argv).slice(2), {
    cwd: options.cwd ?? process.cwd(),
    env: options.env ?? process.env,
    stdin: options.stdin ?? process.stdin,
    interactive: options.interactive ?? Boolean((options.stdin ?? process.stdin).isTTY),
    stdout: options.stdout ?? ((text) => process.stdout.write(text)),
    stderr: options.stderr ?? ((text) => process.stderr.write(text)),
  });

  if (typeof exitCode === "number" && exitCode !== 0) {
    process.exitCode = exitCode;
  }

  return exitCode;
}
