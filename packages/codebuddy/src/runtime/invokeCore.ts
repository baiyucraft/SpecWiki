import { spawn } from "node:child_process";

import { parseResult, type CoreResponse } from "./parseResult.js";
import { resolveBinary } from "./resolveBinary.js";

export type CoreCommand = {
  action: string;
  repoRoot?: string;
  term?: string;
};

export async function invokeCore(command: CoreCommand): Promise<CoreResponse> {
  const binary = resolveBinary();

  return new Promise((resolve, reject) => {
    const child = spawn(binary, ["--json"], {
      stdio: ["pipe", "pipe", "pipe"],
    });

    let stdout = "";
    let stderr = "";

    child.stdout.on("data", (chunk) => {
      stdout += chunk.toString();
    });

    child.stderr.on("data", (chunk) => {
      stderr += chunk.toString();
    });

    child.on("error", (error) => {
      reject(new Error(`failed to launch wiki-core at ${binary}: ${error.message}`));
    });
    child.on("close", (code) => {
      if (code !== 0) {
        reject(new Error(stderr || `wiki-core exited with code ${code}`));
        return;
      }

      try {
        resolve(parseResult(stdout.trim()));
      } catch (error) {
        reject(error);
      }
    });

    child.stdin.write(JSON.stringify(command));
    child.stdin.end();
  });
}
