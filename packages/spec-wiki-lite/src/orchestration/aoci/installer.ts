import { createHash, randomUUID } from "node:crypto";
import https from "node:https";
import {
  chmodSync,
  existsSync,
  mkdirSync,
  readFileSync,
  renameSync,
  rmSync,
  writeFileSync,
} from "node:fs";
import os from "node:os";
import path from "node:path";

import type { AociReleaseAsset } from "../tools/manifest.js";
import { invokeTool, runToolCommand, type ToolCommandRunner } from "../tools/command.js";

export type AociInstallResult = {
  executablePath: string;
  installed: boolean;
  version: string;
};

export type AociInstallerOptions = {
  installRoot: string;
  asset: AociReleaseAsset;
  version: string;
  env?: NodeJS.ProcessEnv;
  runner?: ToolCommandRunner;
  download?: (url: string) => Promise<Buffer>;
  extract?: (archivePath: string, destination: string) => Promise<void>;
  verifyExecutable?: (executablePath: string) => Promise<string>;
};

export function defaultAociInstallRoot(env: NodeJS.ProcessEnv = process.env): string {
  if (env.SPEC_WIKI_LITE_TOOLS_DIR) return path.resolve(env.SPEC_WIKI_LITE_TOOLS_DIR);
  if (process.platform === "win32") {
    return path.join(env.LOCALAPPDATA || path.join(os.homedir(), "AppData", "Local"), "spec-wiki-lite", "tools", "aoci");
  }
  return path.join(env.XDG_DATA_HOME || path.join(os.homedir(), ".local", "share"), "spec-wiki-lite", "tools", "aoci");
}

async function downloadBytes(url: string): Promise<Buffer> {
  try {
    const response = await fetch(url, { redirect: "follow" });
    if (!response.ok) throw new Error(`AOCI download failed with HTTP ${response.status}`);
    return Buffer.from(await response.arrayBuffer());
  } catch (fetchError) {
    return new Promise<Buffer>((resolve, reject) => {
      const request = https.get(url, { headers: { "user-agent": "spec-wiki-lite" } }, response => {
        if (response.statusCode && response.statusCode >= 300 && response.statusCode < 400 && response.headers.location) {
          response.resume();
          downloadBytes(new URL(response.headers.location, url).toString()).then(resolve, reject);
          return;
        }
        if (response.statusCode !== 200) {
          response.resume();
          reject(new Error(`AOCI download failed with HTTP ${response.statusCode ?? "unknown"}`));
          return;
        }
        const chunks: Buffer[] = [];
        let size = 0;
        response.on("data", chunk => {
          const bytes = Buffer.from(chunk);
          size += bytes.length;
          if (size > 100 * 1024 * 1024) request.destroy(new Error("AOCI archive exceeds 100 MiB"));
          else chunks.push(bytes);
        });
        response.on("end", () => resolve(Buffer.concat(chunks)));
        response.on("error", reject);
      });
      request.on("error", error => reject(new Error(`AOCI download failed: ${error.message}; ${fetchError instanceof Error ? fetchError.message : String(fetchError)}`)));
    });
  }
}

function validateArchiveEntries(listing: string): void {
  for (const raw of listing.split(/\r?\n/u)) {
    const entry = raw.trim().replaceAll("\\", "/");
    if (!entry) continue;
    if (entry.startsWith("/") || /^[A-Za-z]:\//u.test(entry) || entry.split("/").includes("..")) {
      throw new Error(`unsafe AOCI archive entry: ${raw}`);
    }
  }
}

async function extractArchive(
  archivePath: string,
  destination: string,
  runner: ToolCommandRunner,
  env: NodeJS.ProcessEnv,
): Promise<void> {
  const list = await invokeTool(runner, "tar", ["-tf", archivePath], { cwd: destination, env, timeoutMs: 60_000 });
  if (list.code !== 0) throw new Error(`unable to inspect AOCI archive: ${list.stderr || list.stdout}`);
  validateArchiveEntries(list.stdout);
  const verbose = await invokeTool(runner, "tar", ["-tvf", archivePath], { cwd: destination, env, timeoutMs: 60_000 });
  if (verbose.code !== 0) throw new Error(`unable to inspect AOCI archive types: ${verbose.stderr || verbose.stdout}`);
  if (verbose.stdout.split(/\r?\n/u).some(line => /^[lh]/u.test(line.trim()))) {
    throw new Error("AOCI archive contains a link entry");
  }
  const result = await invokeTool(runner, "tar", ["-xf", archivePath, "-C", destination], {
    cwd: destination,
    env,
    timeoutMs: 60_000,
  });
  if (result.code !== 0) throw new Error(`unable to extract AOCI archive: ${result.stderr || result.stdout}`);
}

function findExecutable(directory: string, name: string): string | undefined {
  const direct = path.join(directory, name);
  if (existsSync(direct)) return direct;
  const nested = path.join(directory, path.basename(name, path.extname(name)), name);
  return existsSync(nested) ? nested : undefined;
}

export async function installAociRelease(options: AociInstallerOptions): Promise<AociInstallResult> {
  const env = options.env ?? process.env;
  const runner = options.runner ?? runToolCommand;
  const versionRoot = path.join(options.installRoot, options.version);
  const publishedExecutable = path.join(versionRoot, options.asset.executable);
  const verify = options.verifyExecutable ?? (async (executablePath) => {
    const result = await invokeTool(runner, executablePath, ["--version"], { cwd: path.dirname(executablePath), env, timeoutMs: 15_000 });
    if (result.code !== 0) throw new Error(`AOCI executable verification failed: ${result.stderr || result.stdout}`);
    return result.stdout.trim();
  });
  if (existsSync(publishedExecutable)) {
    const output = await verify(publishedExecutable);
    if (output.includes(options.version)) {
      return { executablePath: publishedExecutable, installed: false, version: options.version };
    }
  }

  mkdirSync(options.installRoot, { recursive: true });
  const temporaryRoot = path.join(options.installRoot, `.tmp-${options.version}-${randomUUID()}`);
  const archivePath = path.join(temporaryRoot, options.asset.archive);
  const extractedRoot = path.join(temporaryRoot, "extracted");
  mkdirSync(extractedRoot, { recursive: true });
  try {
    const bytes = await (options.download ?? downloadBytes)(options.asset.url);
    const actual = createHash("sha256").update(bytes).digest("hex");
    if (actual !== options.asset.sha256.toLowerCase()) {
      throw new Error(`AOCI archive checksum mismatch: expected ${options.asset.sha256}, received ${actual}`);
    }
    writeFileSync(archivePath, bytes);
    if (options.extract) await options.extract(archivePath, extractedRoot);
    else await extractArchive(archivePath, extractedRoot, runner, env);
    const extractedExecutable = findExecutable(extractedRoot, options.asset.executable);
    if (!extractedExecutable) throw new Error(`AOCI archive did not contain ${options.asset.executable}`);
    if (process.platform !== "win32") chmodSync(extractedExecutable, 0o755);
    const verifiedOutput = await verify(extractedExecutable);
    if (!verifiedOutput.toLowerCase().includes(options.version.toLowerCase()) && options.version !== "test") {
      throw new Error(`AOCI executable reported an unexpected version: ${verifiedOutput}`);
    }
    const publishRoot = path.join(temporaryRoot, "publish");
    mkdirSync(publishRoot, { recursive: true });
    writeFileSync(path.join(publishRoot, options.asset.executable), readFileSync(extractedExecutable));
    if (process.platform !== "win32") chmodSync(path.join(publishRoot, options.asset.executable), 0o755);
    rmSync(versionRoot, { recursive: true, force: true });
    renameSync(publishRoot, versionRoot);
    return { executablePath: publishedExecutable, installed: true, version: options.version };
  } finally {
    rmSync(temporaryRoot, { recursive: true, force: true });
  }
}
