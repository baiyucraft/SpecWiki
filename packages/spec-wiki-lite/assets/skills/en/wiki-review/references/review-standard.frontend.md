# Frontend Review Standard

Use together with `references/review-standard.md`, only for an actual frontend/UI diff.

## Blocking / P0

- External JSON, URL, form, storage, upload, and environment input need runtime boundaries beyond TypeScript.
- Promise rejection, request cancellation, unmount, timers, subscriptions, and listeners require cleanup.
- Hook lifecycle, stale closures, controlled state, list keys, and optimistic rollback must be correct.
- Form constraints, duplicate-submit prevention, and loading/empty/error/disabled/success/recovery states must be visible and verifiable.
- Unsanitized HTML, unencoded URLs, or sensitive tokens in storage/logs are blocking.
- Critical user behavior and failure states need component, E2E, or structured fallback evidence.

## Non-blocking / P1/P2

- Performance risk from large lists/images, high-frequency events, repeated render, or pointless memoization.
- Accessibility gaps in focus, keyboard, semantic labels, or error association.
- Oversized components, misplaced state, style leakage, or stacking complexity.

## Tooling and False Positives

- Existing component tests, browser automation, or other evidence are acceptable; never fail mechanically because one particular runner is absent.
- Do not require universal memoization or block on aesthetic preference.
