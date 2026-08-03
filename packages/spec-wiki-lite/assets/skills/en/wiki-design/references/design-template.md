# <change-id> Design

## Overview

<Core design, boundaries, and why it satisfies the proposal.>

## Interfaces and Stable Contracts

- <CLI/API/file/schema/path/field>

## Ownership and Data/File Flow

```text
<source of truth> -> <operation> -> <managed/user-owned output>
```

## Normal Flow

1. <Step and observable result>

## Failure, Boundaries, and Rollback

- Invalid input: <fail-closed behavior>
- Repeat/concurrent execution: <idempotency or conflict>
- Partial failure: <atomicity and recovery>
- User content: <protection rule>
- Path safety: <containment/symlink rule>

## Verification Design

- <Test seam, output, command, and success-criteria mapping entry>

## Wiki and Durable Contract Landing

- `<page>`: <Update and SSOT responsibility>

## Reference Boundary

- Source: <source>
- Target landing area: <target landing area>
- Adoption mode: direct migration / rewrite / inspiration only

## Rollback

- <How to restore safely without damaging user content>
