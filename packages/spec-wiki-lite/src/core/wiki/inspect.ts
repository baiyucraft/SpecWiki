import { existsSync, readFileSync, readdirSync, statSync } from "node:fs";
import path from "node:path";

import { parseYamlFrontmatter } from "../markdown/frontmatter.js";
import { readProjectConfig, type WikiLanguage } from "../config.js";
import { resolveSafePath } from "../path.js";

const EXCLUDED_DIRECTORIES = new Set([".cache", ".knowledge", "pages"]);
const REQUIRED_FRONTMATTER = ["title", "description", "updated", "owner"] as const;
const BOOTSTRAP_MARKER = "<!-- spec-wiki-lite:bootstrap-pending -->";

export type WikiIssueKind
  = | "missing_index"
    | "invalid_frontmatter"
    | "broken_link"
    | "orphan_page"
    | "duplicate_ssot";

export type WikiIssue = {
  kind: WikiIssueKind;
  path: string;
  message: string;
};

export type WikiInspectionReport = {
  ready: boolean;
  language: WikiLanguage;
  bootstrapPending: boolean;
  pages: string[];
  issues: WikiIssue[];
};

type ParsedPage = {
  absolutePath: string;
  path: string;
  content: string;
  frontmatter?: Record<string, unknown>;
};

function toProjectPath(value: string): string {
  return value.replaceAll("\\", "/");
}

function listMarkdownFiles(wikiRoot: string): string[] {
  if (!existsSync(wikiRoot)) {
    return [];
  }
  return readdirSync(wikiRoot, { recursive: true, withFileTypes: true })
    .filter(entry => entry.isFile() && entry.name.toLowerCase().endsWith(".md"))
    .map(entry => path.join(entry.parentPath, entry.name))
    .filter((file) => {
      const segments = path.relative(wikiRoot, file).split(path.sep);
      return !segments.some(segment => EXCLUDED_DIRECTORIES.has(segment));
    })
    .sort();
}

function hasRequiredFrontmatter(frontmatter: Record<string, unknown> | undefined): boolean {
  return frontmatter !== undefined && REQUIRED_FRONTMATTER.every((field) => {
    const value = frontmatter[field];
    return typeof value === "string" && value.trim().length > 0;
  });
}

function extractMarkdownLinks(content: string): string[] {
  const links: string[] = [];
  const pattern = /(?<!!)\[[^\]]*\]\(([^)]+)\)/g;
  for (const match of content.matchAll(pattern)) {
    const raw = match[1].trim();
    const target = raw.startsWith("<") && raw.includes(">")
      ? raw.slice(1, raw.indexOf(">"))
      : raw.split(/\s+["']/u, 1)[0];
    links.push(target);
  }
  return links;
}

function resolveWikiLink(
  wikiRoot: string,
  pagePath: string,
  rawTarget: string,
): string | undefined {
  if (rawTarget.startsWith("#") || /^[a-z][a-z\d+.-]*:/iu.test(rawTarget)) {
    return undefined;
  }
  const withoutFragment = rawTarget.split(/[?#]/u, 1)[0];
  let decoded: string;
  try {
    decoded = decodeURIComponent(withoutFragment);
  } catch {
    decoded = withoutFragment;
  }
  const candidate = path.resolve(path.dirname(pagePath), decoded);
  const relative = path.relative(wikiRoot, candidate);
  if (relative.startsWith("..") || path.isAbsolute(relative)) {
    return "";
  }
  const safeCandidate = resolveSafePath(wikiRoot, relative || ".");
  if (existsSync(safeCandidate) && statSync(safeCandidate).isDirectory()) {
    return resolveSafePath(wikiRoot, path.join(relative, "INDEX.md"));
  }
  return safeCandidate;
}

function addIssue(issues: WikiIssue[], issue: WikiIssue): void {
  if (!issues.some(existing => existing.kind === issue.kind && existing.path === issue.path && existing.message === issue.message)) {
    issues.push(issue);
  }
}

export async function inspectWiki(projectRoot: string): Promise<WikiInspectionReport> {
  const root = path.resolve(projectRoot);
  const wikiRoot = path.basename(root) === ".wiki" ? root : resolveSafePath(root, ".wiki");
  const repositoryRoot = path.basename(root) === ".wiki" ? path.dirname(root) : root;
  const config = readProjectConfig(repositoryRoot);
  const files = listMarkdownFiles(wikiRoot);
  const issues: WikiIssue[] = [];
  const pages: ParsedPage[] = files.map((listedPath) => {
    const absolutePath = resolveSafePath(wikiRoot, path.relative(wikiRoot, listedPath));
    const content = readFileSync(absolutePath, "utf8");
    return {
      absolutePath,
      path: toProjectPath(path.relative(root, absolutePath)),
      content,
      frontmatter: parseYamlFrontmatter(content),
    };
  });
  const fileSet = new Set(files.map(file => path.resolve(file)));

  const directories = new Set(files.map(file => path.dirname(file)));
  if (!existsSync(path.join(wikiRoot, "INDEX.md"))) {
    directories.add(wikiRoot);
  }
  for (const directory of directories) {
    const indexPath = path.join(directory, "INDEX.md");
    if (!fileSet.has(path.resolve(indexPath))) {
      addIssue(issues, {
        kind: "missing_index",
        path: toProjectPath(path.relative(root, indexPath)),
        message: "Markdown directory is missing INDEX.md",
      });
    }
  }

  const graph = new Map<string, Set<string>>();
  for (const page of pages) {
    if (!hasRequiredFrontmatter(page.frontmatter)) {
      addIssue(issues, {
        kind: "invalid_frontmatter",
        path: page.path,
        message: `YAML frontmatter requires ${REQUIRED_FRONTMATTER.join(", ")}`,
      });
    }
    const targets = new Set<string>();
    for (const rawTarget of extractMarkdownLinks(page.content)) {
      const target = resolveWikiLink(wikiRoot, page.absolutePath, rawTarget);
      if (target === undefined) {
        continue;
      }
      if (target === "" || !fileSet.has(path.resolve(target))) {
        addIssue(issues, {
          kind: "broken_link",
          path: page.path,
          message: `Relative Wiki link does not resolve: ${rawTarget}`,
        });
        continue;
      }
      targets.add(path.resolve(target));
    }
    graph.set(path.resolve(page.absolutePath), targets);
  }

  const reachable = new Set<string>();
  const queue = [path.resolve(wikiRoot, "INDEX.md")];
  while (queue.length > 0) {
    const current = queue.shift()!;
    if (reachable.has(current) || !fileSet.has(current)) {
      continue;
    }
    reachable.add(current);
    queue.push(...(graph.get(current) ?? []));
  }
  for (const page of pages) {
    if (path.basename(page.absolutePath) !== "INDEX.md" && !reachable.has(path.resolve(page.absolutePath))) {
      addIssue(issues, {
        kind: "orphan_page",
        path: page.path,
        message: "Page is not reachable from .wiki/INDEX.md",
      });
    }
  }

  const ssotOwners = new Map<string, ParsedPage[]>();
  for (const page of pages) {
    const value = page.frontmatter?.source_of_truth;
    if (typeof value === "string" && value.trim().length > 0) {
      const key = value.trim();
      ssotOwners.set(key, [...(ssotOwners.get(key) ?? []), page]);
    }
  }
  for (const [source, owners] of ssotOwners) {
    if (owners.length < 2) {
      continue;
    }
    for (const owner of owners) {
      addIssue(issues, {
        kind: "duplicate_ssot",
        path: owner.path,
        message: `source_of_truth is declared by multiple pages: ${source}`,
      });
    }
  }

  issues.sort((left, right) => left.path.localeCompare(right.path) || left.kind.localeCompare(right.kind));
  return {
    ready: issues.length === 0,
    language: config.language,
    bootstrapPending: pages.some(page => page.absolutePath === path.join(wikiRoot, "INDEX.md") && page.content.includes(BOOTSTRAP_MARKER)),
    pages: pages.map(page => page.path),
    issues,
  };
}
