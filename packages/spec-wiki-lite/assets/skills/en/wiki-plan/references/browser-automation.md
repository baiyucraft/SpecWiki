# General Browser Automation Testing Guide

Browser automation is optional evidence. It is not a runner built into SpecWiki Lite and does not require a particular framework.

## Adoption Conditions

- The project starts reliably in an isolated environment with a known base URL.
- Test accounts, fixtures, and permissions are controlled; no production data or irreversible external state is touched.
- Existing project tooling can be reused, or the user explicitly authorized a new dependency.
- Actions are repeatable and cleanable, with no destructive residue on failure.

## Evidence Requirements

- Record environment, data, preconditions, steps, and machine-checkable assertions.
- Prefer visible text, URL, network result, DOM state, file/API side effect, or other deterministic assertions over timed waits.
- Cover loading, empty, error, success, disabled, duplicate-submit, and recovery states when applicable.
- Screenshots, traces, and video are supporting evidence only; none can be the sole pass evidence for an ST or success criterion.

## Tool Selection

- Prefer the repository's existing browser or E2E tooling.
- Depending on the project, Playwright, WebDriver, Cypress, a browser connector, or another reliable option may be used.
- Never claim that Lite provides a browser environment, test data, or server launcher.

## Fallback

When tooling is unavailable, the project cannot start safely, or permissions are insufficient, record the `fallback reason` and cover the same ST/success criterion with component tests, API/CLI assertions, static output checks, or structured manual verification.
