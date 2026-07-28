import { existsSync, mkdirSync, readFileSync, renameSync, rmSync, writeFileSync } from "node:fs";
import path from "node:path";
import { randomUUID } from "node:crypto";
import { parseDocument, type Document } from "yaml";

import { resolveSafePath } from "./path.js";

export const WIKI_LANGUAGES = ["zh", "en"] as const;
export type WikiLanguage = typeof WIKI_LANGUAGES[number];
export const DEFAULT_WIKI_LANGUAGE: WikiLanguage = "zh";
export const PROJECT_CONFIG_PATH = ".wiki/config.yaml";

export type ProjectConfig = {
  exists: boolean;
  language: WikiLanguage;
  version: 1;
};

function atomicWrite(target: string, content: string): void {
  mkdirSync(path.dirname(target), { recursive: true });
  const temporary = path.join(path.dirname(target), `.${path.basename(target)}.${randomUUID()}.tmp`);
  try {
    writeFileSync(temporary, content, "utf8");
    renameSync(temporary, target);
  } finally {
    rmSync(temporary, { force: true });
  }
}

export function isWikiLanguage(value: unknown): value is WikiLanguage {
  return typeof value === "string" && WIKI_LANGUAGES.includes(value as WikiLanguage);
}

function parseConfigDocument(content: string): {
  document: Document;
  language: WikiLanguage;
} {
  const document = parseDocument(content);
  if (document.errors.length > 0) {
    throw new Error(`invalid .wiki/config.yaml: ${document.errors.map(error => error.message).join("; ")}`);
  }
  const value: unknown = document.toJS();
  if (typeof value !== "object" || value === null || Array.isArray(value)) {
    throw new Error("invalid .wiki/config.yaml: root must be a YAML object");
  }
  const config = value as Record<string, unknown>;
  if (config.version !== 1) {
    throw new Error("invalid .wiki/config.yaml: version must be 1");
  }
  const wiki = config.wiki;
  if (typeof wiki !== "object" || wiki === null || Array.isArray(wiki)) {
    throw new Error("invalid .wiki/config.yaml: wiki must be an object");
  }
  const language = (wiki as Record<string, unknown>).language;
  if (!isWikiLanguage(language)) {
    throw new Error(`invalid .wiki/config.yaml: wiki.language must be one of ${WIKI_LANGUAGES.join(", ")}`);
  }
  return { document, language };
}

export function readProjectConfig(projectRoot: string): ProjectConfig {
  const configPath = resolveSafePath(path.resolve(projectRoot), PROJECT_CONFIG_PATH);
  if (!existsSync(configPath)) {
    return {
      exists: false,
      language: DEFAULT_WIKI_LANGUAGE,
      version: 1,
    };
  }
  const { language } = parseConfigDocument(readFileSync(configPath, "utf8"));
  return { exists: true, language, version: 1 };
}

export function projectConfigContent(
  projectRoot: string,
  language: WikiLanguage,
): { content: string; outcome: "created" | "updated" | "unchanged" } {
  const configPath = resolveSafePath(path.resolve(projectRoot), PROJECT_CONFIG_PATH);
  if (!existsSync(configPath)) {
    return {
      content: `version: 1\nwiki:\n  language: ${language}\n`,
      outcome: "created",
    };
  }
  const previous = readFileSync(configPath, "utf8");
  const { document, language: current } = parseConfigDocument(previous);
  if (current === language) {
    return { content: previous, outcome: "unchanged" };
  }
  document.setIn(["wiki", "language"], language);
  return { content: document.toString(), outcome: "updated" };
}

export function writeProjectLanguage(
  projectRoot: string,
  language: WikiLanguage,
): "created" | "updated" | "unchanged" {
  const root = path.resolve(projectRoot);
  const configPath = resolveSafePath(root, PROJECT_CONFIG_PATH);
  const result = projectConfigContent(root, language);
  if (result.outcome !== "unchanged") {
    atomicWrite(configPath, result.content);
  }
  return result.outcome;
}
