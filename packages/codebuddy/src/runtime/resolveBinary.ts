import path from "node:path";
import { fileURLToPath } from "node:url";

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

export function resolveBinary(): string {
  if (process.env.CODEBUDDY_WIKI_CORE_BIN) {
    return process.env.CODEBUDDY_WIKI_CORE_BIN;
  }

  const executable = process.platform === "win32" ? "wiki-core.exe" : "wiki-core";
  return path.resolve(__dirname, "../../../../target/debug", executable);
}
