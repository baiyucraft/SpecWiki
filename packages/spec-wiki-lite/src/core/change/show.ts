import { readFileSync } from "node:fs";
import { ARTIFACTS, isArtifactId, type ArtifactId } from "./artifacts.js";
import { changeFilePath } from "./metadata.js";
import { validateChange, type ChangeValidationResult } from "./validate.js";

export type ShowChangeResult = {
  change: ChangeValidationResult;
  artifact?: {
    id: ArtifactId;
    path: string;
    content: string;
  };
};

export async function showChange(
  projectRoot: string,
  changeId: string,
  artifact?: string,
): Promise<ShowChangeResult> {
  const change = await validateChange(projectRoot, changeId);
  if (!artifact) {
    return { change };
  }
  if (!isArtifactId(artifact)) {
    throw new Error(`unknown artifact: ${artifact}`);
  }
  const artifactPath = changeFilePath(projectRoot, changeId, ARTIFACTS[artifact]);
  return {
    change,
    artifact: {
      id: artifact,
      path: artifactPath,
      content: readFileSync(artifactPath, "utf8"),
    },
  };
}
