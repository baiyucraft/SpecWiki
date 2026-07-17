/**
 * 这个文件定义首批宿主的 registry、检测与显式选择规则。
 * 它只关心宿主目录与落点，不承载 Wiki 业务逻辑。
 */
import { existsSync, statSync } from "node:fs";
import os from "node:os";
import path from "node:path";

export type SupportedHost = "codex" | "claude" | "codebuddy";

/** 宿主相对 Codex-first 合同的兼容角色。 */
export type HostCompatibilityRole = "reference" | "compatible";

/** 单宿主真实可观测的 trigger/delivery 能力声明。 */
export type HostTriggerCapabilities = {
  /** 是否通过 repo-local skill discovery 暴露公开 action。 */
  nativeSkillDiscovery: "supported";
  /** 是否存在项目可控的用户 prompt 检查机制。 */
  promptInspection: "none" | "user_prompt_submit";
  /** 会话启动时是否注入固定 orientation context。 */
  sessionStartContext: "none" | "orientation";
  /** 是否需要写入受管宿主 settings。 */
  settingsIntegration: "none" | "managed_settings";
  /** Trigger decision 的宿主投递机制。 */
  deterministicDelivery: "skill_guidance" | "generated_hook";
};

export type HostDefinition = {
  /** 宿主稳定 ID。 */
  id: SupportedHost;
  /** 面向用户的显示名称。 */
  displayName: string;
  /** 项目根目录下用于检测该宿主的目录。 */
  detectDir: string;
  /** 当前宿主是 reference 还是 compatible projection。 */
  compatibilityRole: HostCompatibilityRole;
  /** 必须与实际生成 assets 一致的 trigger 能力。 */
  triggerCapabilities: HostTriggerCapabilities;
};

/** 当前 iteration-11-6 明确支持的宿主注册表。 */
export const HOSTS: readonly HostDefinition[] = [
  {
    id: "codex",
    displayName: "Codex",
    detectDir: ".codex",
    compatibilityRole: "reference",
    triggerCapabilities: {
      nativeSkillDiscovery: "supported",
      promptInspection: "none",
      sessionStartContext: "none",
      settingsIntegration: "none",
      deterministicDelivery: "skill_guidance",
    },
  },
  {
    id: "claude",
    displayName: "Claude Code",
    detectDir: ".claude",
    compatibilityRole: "compatible",
    triggerCapabilities: {
      nativeSkillDiscovery: "supported",
      promptInspection: "none",
      sessionStartContext: "none",
      settingsIntegration: "none",
      deterministicDelivery: "skill_guidance",
    },
  },
  {
    id: "codebuddy",
    displayName: "CodeBuddy",
    detectDir: ".codebuddy",
    compatibilityRole: "compatible",
    triggerCapabilities: {
      nativeSkillDiscovery: "supported",
      promptInspection: "user_prompt_submit",
      sessionStartContext: "orientation",
      settingsIntegration: "managed_settings",
      deterministicDelivery: "generated_hook",
    },
  },
] as const;

/**
 * 解析 Codex 的全局 home 目录。
 *
 * @param env 当前进程环境变量。
 * @returns 返回 `<CODEX_HOME>` 的绝对路径。
 */
export function resolveCodexHome(env: NodeJS.ProcessEnv): string {
  const configuredHome = env.CODEX_HOME?.trim();
  const rawHome = configuredHome && configuredHome.length > 0
    ? configuredHome
    : path.join(os.homedir(), ".codex");

  return path.resolve(rawHome);
}

/**
 * 检测项目根目录内已经存在哪些受支持宿主目录。
 *
 * @param repoRoot 当前项目根目录。
 * @returns 返回按 registry 顺序排序的宿主 ID 列表。
 */
export function detectHosts(repoRoot: string): SupportedHost[] {
  return HOSTS
    .filter((host) => {
      const hostPath = path.join(repoRoot, host.detectDir);

      if (!existsSync(hostPath)) {
        return false;
      }

      try {
        return statSync(hostPath).isDirectory();
      } catch {
        return false;
      }
    })
    .map((host) => host.id);
}

/**
 * 把 `--tool` / `--tools` 参数解析为宿主列表。
 *
 * @param rawTools 原始参数文本。
 * @returns 返回去重后的宿主列表；若未传参则返回空数组。
 */
export function parseRequestedHosts(rawTools?: string): SupportedHost[] {
  if (!rawTools) {
    return [];
  }

  const resolvedHosts = rawTools
    .split(",")
    .map((token) => token.trim().toLowerCase())
    .filter((token) => token.length > 0);

  if (resolvedHosts.length === 0) {
    return [];
  }

  const validHosts = new Set(HOSTS.map((host) => host.id));
  const dedupedHosts: SupportedHost[] = [];

  for (const host of resolvedHosts) {
    if (!validHosts.has(host as SupportedHost)) {
      throw new Error(
        `unsupported host "${host}". Supported hosts: ${HOSTS.map((item) => item.id).join(", ")}`,
      );
    }

    if (!dedupedHosts.includes(host as SupportedHost)) {
      dedupedHosts.push(host as SupportedHost);
    }
  }

  return dedupedHosts;
}

/**
 * 根据检测结果和显式参数决定本次 bootstrap 的宿主。
 *
 * @param repoRoot 当前项目根目录。
 * @param rawTools `--tool` / `--tools` 的原始参数。
 * @returns 返回最终应执行 bootstrap 的宿主列表。
 */
export function resolveSelectedHosts(repoRoot: string, rawTools?: string): SupportedHost[] {
  const requestedHosts = parseRequestedHosts(rawTools);
  if (requestedHosts.length > 0) {
    return requestedHosts;
  }

  const detectedHosts = detectHosts(repoRoot);
  if (detectedHosts.length === 1) {
    return detectedHosts;
  }

  if (detectedHosts.length > 1) {
    throw new Error(
      `multiple hosts detected: ${detectedHosts.join(", ")}. Use --host or --hosts to choose explicitly.`,
    );
  }

  throw new Error(
    `no supported host detected under ${repoRoot}. Use --host codex, --host claude, or --host codebuddy.`,
  );
}

/**
 * 获取单个宿主的 registry 定义。
 *
 * @param id 宿主 ID。
 * @returns 返回对应的宿主定义。
 */
export function getHostDefinition(id: SupportedHost): HostDefinition {
  const host = HOSTS.find((item) => item.id === id);
  if (!host) {
    throw new Error(`unknown host ${id}`);
  }
  return host;
}
