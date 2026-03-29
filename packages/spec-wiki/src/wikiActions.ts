/**
 * 这个文件统一定义 `spec-wiki` 的内部 runtime action 与公开 CLI action。
 * 宿主资产模板、CLI 参数解析和测试都依赖这份单一真相来源。
 */

/** `wiki-runtime` 当前仍可识别的内部 action 集合。 */
export const WIKI_ACTIONS = [
  "init",
  "status",
  "update",
  "query",
  "sync",
  "rebuild",
] as const;

/** `v0.1.0` 当前对外正式支持的 CLI / 宿主显式动作集合。 */
export const PUBLIC_WIKI_ACTIONS = [
  "init",
  "status",
  "update",
  "query",
] as const;

/** `v0.1.0` 当前建议在宿主显式暴露的 action 集合。 */
export const HOST_EXPOSED_ACTIONS = PUBLIC_WIKI_ACTIONS;

/** 单个 action 的字面量类型。 */
export type WikiAction = (typeof WIKI_ACTIONS)[number];

/** 单个宿主显式暴露 action 的字面量类型。 */
export type HostExposedAction = (typeof HOST_EXPOSED_ACTIONS)[number];

/** 单个公开 CLI action 的字面量类型。 */
export type PublicWikiAction = (typeof PUBLIC_WIKI_ACTIONS)[number];

/**
 * 判断一个字符串是否是受支持的 runtime action。
 *
 * @param action 待校验的 action 名称。
 * @returns 当 action 受支持时返回 `true`。
 */
export function isWikiAction(action: string): action is WikiAction {
  return (WIKI_ACTIONS as readonly string[]).includes(action);
}

/**
 * 判断一个字符串是否是 `v0.1.0` 对外支持的 CLI action。
 *
 * @param action 待校验的 action 名称。
 * @returns 当 action 在当前公开命令面内时返回 `true`。
 */
export function isPublicWikiAction(action: string): action is PublicWikiAction {
  return (PUBLIC_WIKI_ACTIONS as readonly string[]).includes(action);
}
