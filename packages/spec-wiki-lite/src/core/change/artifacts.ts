export const ARTIFACTS = {
  "split": "split.md",
  "proposal": "proposal.md",
  "design": "design.md",
  "cases": "system-tests.md",
  "tasks": "tasks.md",
  "unit-tests": "unit-tests.md",
  "review-report": "review-report.md",
  "test-report": "test-report.md",
  "metadata": "meta.yaml",
} as const;

export type ArtifactId = keyof typeof ARTIFACTS;

export function isArtifactId(value: string): value is ArtifactId {
  return Object.hasOwn(ARTIFACTS, value);
}
