/**
 * 这个文件收口 `wiki-core` 的最小响应协议校验。
 * Agent 只依赖稳定的外壳字段，不把 core 的内部数据结构写死在这里。
 */
export type CoreResponse = {
  /** 是否执行成功。 */
  ok: boolean;
  /** 失败时的错误消息。 */
  error?: string | null;
  /** 成功或失败时附带的数据载荷。 */
  data?: unknown;
};

/**
 * 校验 `wiki-core` 的最小响应协议。
 * 当前只强校验 `ok` 字段，其他字段允许随着 core 能力继续扩展。
 *
 * @param stdout `wiki-core` 写到标准输出的 JSON 文本。
 * @returns 返回通过最小协议校验的响应对象。
 */
export function parseResult(stdout: string): CoreResponse {
  const parsed = JSON.parse(stdout) as Partial<CoreResponse>;

  if (typeof parsed !== "object" || parsed === null || typeof parsed.ok !== "boolean") {
    throw new Error("invalid wiki-core response");
  }

  return parsed as CoreResponse;
}
