import { login } from "@repo/auth";
import { tokenLabel } from "@repo/shared";

export function bootstrap(): string {
  return `${login()} ${tokenLabel}`;
}
