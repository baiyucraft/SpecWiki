import { tokenLabel } from "@repo/shared";

export function login(): string {
  return `auth:${tokenLabel}`;
}
