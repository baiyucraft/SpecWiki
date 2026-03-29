const INDEX_ONLY_ACTIONS = new Set(["init", "update"]);
const INDEX_ONLY_ENV_NAME = "SPEC_WIKI_V0_1_INDEX_ONLY";

/**
 * 构造传给 `wiki-runtime` 的环境变量。
 * `v0.1.0` 的临时发布收敛只对 index-only 正式承诺的 action 默认生效。
 */
export function buildCoreEnv(
  baseEnv: NodeJS.ProcessEnv | undefined,
  action: string,
): NodeJS.ProcessEnv | undefined {
  if (!baseEnv) {
    return INDEX_ONLY_ACTIONS.has(action)
      ? { [INDEX_ONLY_ENV_NAME]: "1" }
      : undefined;
  }

  if (!INDEX_ONLY_ACTIONS.has(action) || baseEnv[INDEX_ONLY_ENV_NAME] !== undefined) {
    return baseEnv;
  }

  return {
    ...baseEnv,
    [INDEX_ONLY_ENV_NAME]: "1",
  };
}
