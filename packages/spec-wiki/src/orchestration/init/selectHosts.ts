/**
 * 这个文件负责 `spec-wiki init` 的宿主选择编排。
 * 它借鉴 openspec 的“检测 + 交互多选 + 非交互兜底”流程，并复用宿主层提供的 prompt 组件。
 */
import {
  searchableMultiSelect,
  type SearchableMultiSelectConfig,
} from "../../agents/prompts/searchableMultiSelect.js";
import {
  detectHosts,
  HOSTS,
  parseRequestedHosts,
  type SupportedHost,
} from "../../agents/shared/hosts.js";

export type SelectHostsOptions = {
  /** 当前仓库根目录。 */
  repoRoot: string;
  /** CLI 传入的 `--tool` / `--tools` 原始值。 */
  rawTools?: string;
  /** 是否允许进入交互模式。 */
  interactive?: boolean;
  /** 可选的标准输入，用于检测 TTY。 */
  stdin?: NodeJS.ReadStream;
  /** 测试可注入的多选实现。 */
  selectMultiple?: (config: SearchableMultiSelectConfig) => Promise<string[]>;
};

function canPromptInteractively(options: SelectHostsOptions): boolean {
  if (options.rawTools) {
    return false;
  }

  if (options.interactive === false) {
    return false;
  }

  if (options.selectMultiple) {
    return true;
  }

  const input = options.stdin ?? process.stdin;
  return Boolean(input.isTTY && process.stdout.isTTY);
}

async function promptForHosts(
  detectedHosts: readonly SupportedHost[],
  options: SelectHostsOptions,
): Promise<SupportedHost[]> {
  const detectedSet = new Set(detectedHosts);
  const selectMultiple = options.selectMultiple ?? searchableMultiSelect;

  const selected = await selectMultiple({
    message: `Select hosts to bootstrap (${HOSTS.length} available)`,
    pageSize: 10,
    choices: HOSTS.map((host) => ({
      name: host.displayName,
      value: host.id,
      detected: detectedSet.has(host.id),
      preSelected: detectedSet.has(host.id),
    })),
    validate: (current) => current.length > 0 || "Select at least one host",
  });

  return selected.map((value) => value as SupportedHost);
}

/**
 * 为 `spec-wiki init` 解析最终应 bootstrap 的宿主。
 *
 * @param options 当前仓库、CLI 参数和交互控制项。
 * @returns 返回最终选中的宿主列表。
 */
export async function selectHostsForInit(
  options: SelectHostsOptions,
): Promise<SupportedHost[]> {
  const explicitHosts = parseRequestedHosts(options.rawTools);
  if (explicitHosts.length > 0) {
    return explicitHosts;
  }

  const detectedHosts = detectHosts(options.repoRoot);
  if (canPromptInteractively(options)) {
    return promptForHosts(detectedHosts, options);
  }

  if (detectedHosts.length === 1) {
    return detectedHosts;
  }

  if (detectedHosts.length > 1) {
    throw new Error(
      `multiple hosts detected: ${detectedHosts.join(", ")}. Use --tool or --tools to choose explicitly, or rerun without --no-interactive.`,
    );
  }

  throw new Error(
    `no supported host detected under ${options.repoRoot}. Use --tool codex, --tool claude, or --tool codebuddy, or rerun interactively to choose.`,
  );
}
