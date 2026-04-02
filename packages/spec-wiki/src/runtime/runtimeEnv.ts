/**
 * 构造传给 `wiki-runtime` 的环境变量。
 * `v0.2.0` 不再注入 `index_only` 发布短路环境变量。
 */
export function buildCoreEnv(
  baseEnv: NodeJS.ProcessEnv | undefined,
  _action: string,
): NodeJS.ProcessEnv | undefined {
  return baseEnv;
}
