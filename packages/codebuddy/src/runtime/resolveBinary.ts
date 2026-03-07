import path from "node:path";
import { fileURLToPath } from "node:url";

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

export function resolveBinary(platform = process.platform): string {
  if (process.env.CODEBUDDY_WIKI_CORE_BIN) {
    return process.env.CODEBUDDY_WIKI_CORE_BIN;
  }

  if (platform !== "win32") {
    throw new Error(`CodeBuddy Agent currently supports Windows only (received ${platform})`);
  }

  const executable = "wiki-core.exe";
  return path.resolve(__dirname, "../../../../target/debug", executable);
}
