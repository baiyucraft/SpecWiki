import { spawn } from "node:child_process";
import { cpSync, existsSync, mkdirSync, readFileSync, readdirSync, rmSync, writeFileSync } from "node:fs";
import path from "node:path";
import { fileURLToPath, pathToFileURL } from "node:url";

const rootFromScript = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..");
const PACKAGE_RELATIVE = path.join("packages", "spec-wiki");
const STAGED_RELATIVE = path.join("dist", "spec-wiki");

function readJson(filePath) {
  return JSON.parse(readFileSync(filePath, "utf8"));
}

function run(command, args, cwd) {
  return new Promise((resolve, reject) => {
    const child = spawn(command, args, {
      cwd,
      stdio: "inherit",
      shell: process.platform === "win32",
    });
    child.on("error", reject);
    child.on("close", (code) => code === 0
      ? resolve()
      : reject(new Error(`${command} ${args.join(" ")} exited with code ${code}`)));
  });
}

function publishManifest(source) {
  return Object.fromEntries([
    "name",
    "version",
    "type",
    "description",
    "license",
    "engines",
    "exports",
    "main",
    "bin",
    "files",
    "dependencies",
  ].filter(key => source[key] !== undefined).map(key => [key, source[key]]));
}

export function stagePackage({ rootDir = rootFromScript, outputDir = path.join(rootDir, STAGED_RELATIVE) } = {}) {
  const packageRoot = path.join(rootDir, PACKAGE_RELATIVE);
  const source = readJson(path.join(packageRoot, "package.json"));
  rmSync(outputDir, { recursive: true, force: true });
  mkdirSync(outputDir, { recursive: true });
  for (const entry of ["bin", "dist", "assets"]) {
    const sourcePath = path.join(packageRoot, entry);
    if (!existsSync(sourcePath)) {
      throw new Error(`required package asset is missing: ${sourcePath}`);
    }
    cpSync(sourcePath, path.join(outputDir, entry), { recursive: true });
  }
  cpSync(path.join(rootDir, "README.md"), path.join(outputDir, "README.md"));
  cpSync(path.join(rootDir, "LICENSE"), path.join(outputDir, "LICENSE"));
  writeFileSync(path.join(outputDir, "package.json"), `${JSON.stringify(publishManifest(source), null, 2)}\n`, "utf8");
  return { manifest: publishManifest(source), packageDir: outputDir };
}

export function listPackageFiles(packageDir) {
  return readdirSync(packageDir, { recursive: true, withFileTypes: true })
    .filter(entry => entry.isFile())
    .map(entry => path.relative(packageDir, path.join(entry.parentPath, entry.name)).replaceAll("\\", "/"))
    .sort();
}

export async function buildDistribution({ rootDir = rootFromScript } = {}) {
  const packageRoot = path.join(rootDir, PACKAGE_RELATIVE);
  await run("pnpm", ["build"], packageRoot);
  return stagePackage({ rootDir });
}

if (import.meta.url === pathToFileURL(process.argv[1]).href) {
  const result = await buildDistribution();
  if (process.argv.includes("--pack")) {
    await run("npm", ["pack"], result.packageDir);
  }
  console.log(`Staged ${result.manifest.name}@${result.manifest.version} in ${result.packageDir}`);
}
