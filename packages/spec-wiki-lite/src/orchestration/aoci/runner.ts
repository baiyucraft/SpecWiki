import { existsSync } from "node:fs";
import path from "node:path";

import type { WikiLanguage } from "../../core/config.js";
import { installAociRelease, defaultAociInstallRoot, type AociInstallerOptions } from "./installer.js";
import { invokeTool, runToolCommand, type ToolCommandResult, type ToolCommandRunner } from "../tools/command.js";
import { AOCI_RELEASE, selectAociAsset } from "../tools/manifest.js";

export type AociWarning = { stage: string; message: string; recovery: string };

export type AociDatabaseStatus = {
  required: boolean;
  sourceCount: number;
  ready: boolean;
};

export type AociResult = {
  requested: boolean;
  available: boolean;
  installed: boolean;
  expectedVersion: string;
  version?: string;
  executablePath?: string;
  initialized: boolean;
  governanceAligned: boolean;
  restartRequired: boolean;
  database: AociDatabaseStatus;
  warnings: AociWarning[];
  nextActions: string[];
};

export type AociRunOptions = {
  executablePath: string;
  projectRoot: string;
  language?: WikiLanguage;
  env: NodeJS.ProcessEnv;
  runner?: ToolCommandRunner;
};

export type AociIntegrationOptions = {
  projectRoot: string;
  language: WikiLanguage;
  env: NodeJS.ProcessEnv;
  mode?: "required" | "skip";
  runner?: ToolCommandRunner;
  installRoot?: string;
  installer?: (options: AociInstallerOptions) => Promise<{ executablePath: string; installed: boolean; version: string }>;
  installerOptions?: Partial<Pick<AociInstallerOptions, "download" | "extract" | "verifyExecutable">>;
};

function versionFrom(output: string): string | undefined {
  return output.match(/(?:version\s+|v)(\d+\.\d+\.\d+(?:-[0-9A-Za-z.-]+)?)/iu)?.[1]
    ?? output.match(/(\d+\.\d+\.\d+(?:-[0-9A-Za-z.-]+)?)/u)?.[1];
}

function safeJson(result: ToolCommandResult): unknown | undefined {
  if (result.code !== 0) return undefined;
  try { return JSON.parse(result.stdout) as unknown; } catch { return undefined; }
}

function findBoolean(value: unknown, keys: string[]): boolean | undefined {
  if (!value || typeof value !== "object") return undefined;
  for (const [key, child] of Object.entries(value as Record<string, unknown>)) {
    if (keys.includes(key.toLowerCase()) && typeof child === "boolean") return child;
  }
  for (const child of Object.values(value as Record<string, unknown>)) {
    const found = findBoolean(child, keys);
    if (found !== undefined) return found;
  }
  return undefined;
}

function findArray(value: unknown, keys: string[]): unknown[] | undefined {
  if (!value || typeof value !== "object") return undefined;
  for (const [key, child] of Object.entries(value as Record<string, unknown>)) {
    if (keys.includes(key.toLowerCase()) && Array.isArray(child)) return child;
  }
  return undefined;
}

function resultMessage(result: ToolCommandResult): string {
  return (result.stderr || result.stdout || `exit code ${result.code}`).trim().slice(0, 1000);
}

async function aociCommand(options: AociRunOptions, args: string[]): Promise<ToolCommandResult> {
  return invokeAoci(options, ["--repo", options.projectRoot, ...args], 120_000);
}

async function invokeAoci(options: AociRunOptions, args: string[], timeoutMs: number): Promise<ToolCommandResult> {
  const command = options.env.SPEC_WIKI_LITE_AOCI_NODE_SCRIPT ? process.execPath : options.executablePath;
  const commandArgs = options.env.SPEC_WIKI_LITE_AOCI_NODE_SCRIPT
    ? [options.env.SPEC_WIKI_LITE_AOCI_NODE_SCRIPT, ...args]
    : args;
  return invokeTool(options.runner ?? runToolCommand, command, commandArgs, {
    cwd: options.projectRoot,
    env: options.env,
    timeoutMs,
  });
}

export async function runAociBootstrap(options: AociRunOptions): Promise<AociResult> {
  const base = emptyAociResult(true);
  base.executablePath = options.executablePath;
  const version = await invokeAoci(options, ["--version"], 15_000);
  base.version = versionFrom(version.stdout);
  base.available = version.code === 0;
  if (!base.available || base.version !== AOCI_RELEASE.version) {
    base.warnings.push({ stage: "version", message: resultMessage(version), recovery: "run spec-wiki-lite update --tools" });
    base.nextActions.push("run spec-wiki-lite update --tools");
    return base;
  }
  const locale = options.language === "en" ? "en-US" : "zh-CN";
  for (const [stage, args] of [
    ["init", ["init", "--locale", locale, "--agent", "codex"]],
    ["scan", ["scan"]],
    ["doctor", ["doctor", "--json"]],
  ] as const) {
    const result = await aociCommand(options, [...args]);
    if (result.code !== 0) {
      base.warnings.push({ stage, message: resultMessage(result), recovery: `run aoci --repo <project> ${args.join(" ")}` });
      base.nextActions.push(`repair AOCI ${stage} and rerun spec-wiki-lite init`);
      return base;
    }
  }
  base.initialized = true;
  const inspected = await inspectAoci(options);
  return { ...inspected, initialized: true, installed: base.installed, warnings: [...base.warnings, ...inspected.warnings] };
}

function emptyAociResult(requested: boolean): AociResult {
  return {
    requested,
    available: false,
    installed: false,
    expectedVersion: AOCI_RELEASE.version,
    initialized: false,
    governanceAligned: false,
    restartRequired: false,
    database: { required: false, sourceCount: 0, ready: true },
    warnings: [],
    nextActions: [],
  };
}

export async function inspectAoci(options: AociRunOptions): Promise<AociResult> {
  const state = emptyAociResult(true);
  state.executablePath = options.executablePath;
  const versionResult = await invokeAoci(options, ["--version"], 15_000);
  state.version = versionFrom(versionResult.stdout);
  state.available = versionResult.code === 0;
  if (!state.available || state.version !== AOCI_RELEASE.version) {
    state.nextActions.push("run spec-wiki-lite update --tools");
    return state;
  }

  const verify = await aociCommand(options, ["verify", "--json"]);
  const check = await aociCommand(options, ["check", "--json"]);
  const guide = await aociCommand(options, ["index", "agent", "guide", "--agent", "codex", "--json"]);
  const sources = await aociCommand(options, ["database", "source", "list", "--json"]);
  const verifyJson = safeJson(verify);
  const checkJson = safeJson(check);
  const guideJson = safeJson(guide);
  const sourceJson = safeJson(sources);
  const malformed = [verifyJson, checkJson, guideJson, sourceJson].some(value => value === undefined);
  if (malformed) {
    state.warnings.push({ stage: "inspect", message: "AOCI returned malformed or unavailable JSON", recovery: "run AOCI doctor, verify, check, and Guide manually" });
    state.nextActions.push("run AOCI doctor and complete the official Guide workflow");
    return state;
  }

  const sourceList = Array.isArray(sourceJson)
    ? sourceJson
    : findArray(sourceJson, ["sources", "items", "data"]) ?? [];
  state.database.sourceCount = sourceList.length;
  state.database.required = sourceList.length > 0;
  if (state.database.required) {
    let accessReady = true;
    for (const source of sourceList) {
      const sourceId = source && typeof source === "object"
        ? String((source as Record<string, unknown>).id ?? (source as Record<string, unknown>).name ?? "")
        : "";
      if (!sourceId) { accessReady = false; continue; }
      const access = safeJson(await aociCommand(options, ["database", "source", "access", "--source", sourceId, "--json"]));
      accessReady &&= access !== undefined && findBoolean(access, ["ready", "available", "accessible", "ok"]) !== false;
    }
    const cognition = safeJson(await aociCommand(options, ["database", "cognition", "status", "--json"]));
    const cognitionReady = cognition !== undefined && findBoolean(cognition, ["aligned", "ready", "complete", "valid", "ok"]) === true;
    state.database.ready = accessReady && cognitionReady;
  }

  const verifyReady = findBoolean(verifyJson, ["valid", "verified", "aligned", "ready", "ok"]) === true;
  const checkReady = findBoolean(checkJson, ["valid", "clean", "aligned", "ready", "ok"]) === true;
  const guideReady = findBoolean(guideJson, ["aligned", "complete", "ready", "ok"]) === true;
  state.initialized = existsSync(path.join(options.projectRoot, "aoci.txt")) || verifyReady || checkReady;
  state.governanceAligned = verifyReady && checkReady && guideReady && state.database.ready;
  state.restartRequired = !state.governanceAligned && findBoolean(guideJson, ["restartrequired", "restart_required"]) !== false;
  if (!state.governanceAligned) {
    state.nextActions.push("restart Codex if the AOCI MCP was newly configured, then follow the official AOCI Guide");
  }
  return state;
}

export function resolveInstalledAoci(env: NodeJS.ProcessEnv = process.env): string {
  if (env.SPEC_WIKI_LITE_AOCI_NODE_SCRIPT) return process.execPath;
  if (env.SPEC_WIKI_LITE_AOCI_BIN) return path.resolve(env.SPEC_WIKI_LITE_AOCI_BIN);
  const asset = selectAociAsset(process.platform, process.arch);
  return path.join(defaultAociInstallRoot(env), AOCI_RELEASE.version, asset.executable);
}

export async function runAociIntegration(options: AociIntegrationOptions): Promise<AociResult> {
  if (options.mode === "skip") {
    const skipped = emptyAociResult(false);
    skipped.nextActions.push("rerun spec-wiki-lite init without --no-aoci");
    return skipped;
  }
  const asset = selectAociAsset(process.platform, process.arch);
  const installRoot = options.installRoot ?? defaultAociInstallRoot(options.env);
  try {
    const injected = options.env.SPEC_WIKI_LITE_AOCI_BIN
      || (options.env.SPEC_WIKI_LITE_AOCI_NODE_SCRIPT ? process.execPath : undefined);
    const install = injected
      ? { executablePath: path.resolve(injected), installed: false, version: AOCI_RELEASE.version }
      : await (options.installer ?? installAociRelease)({
        installRoot,
        asset,
        version: AOCI_RELEASE.version,
        env: options.env,
        runner: options.runner,
        ...options.installerOptions,
      });
    const result = await runAociBootstrap({
      executablePath: install.executablePath,
      projectRoot: options.projectRoot,
      language: options.language,
      env: options.env,
      runner: options.runner,
    });
    result.installed = install.installed;
    return result;
  } catch (error) {
    const failed = emptyAociResult(true);
    failed.warnings.push({
      stage: "install",
      message: error instanceof Error ? error.message.slice(0, 1000) : String(error).slice(0, 1000),
      recovery: "check network/platform permissions and rerun spec-wiki-lite update --tools",
    });
    failed.nextActions.push("run spec-wiki-lite update --tools");
    return failed;
  }
}
