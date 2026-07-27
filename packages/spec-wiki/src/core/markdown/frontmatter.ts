import { parseDocument } from "yaml";

export function parseYamlFrontmatter(content: string): Record<string, unknown> | undefined {
  const normalized = content.replace(/^\uFEFF/u, "").replaceAll("\r\n", "\n");
  if (!normalized.startsWith("---\n")) {
    return undefined;
  }
  const closingWithBody = normalized.indexOf("\n---\n", 4);
  const closingAtEnd = normalized.endsWith("\n---") ? normalized.length - 4 : -1;
  const closing = closingWithBody >= 0 ? closingWithBody : closingAtEnd;
  if (closing < 0) {
    return undefined;
  }

  const document = parseDocument(normalized.slice(4, closing));
  if (document.errors.length > 0) {
    return undefined;
  }
  const value: unknown = document.toJS();
  if (typeof value !== "object" || value === null || Array.isArray(value)) {
    return undefined;
  }
  return value as Record<string, unknown>;
}
