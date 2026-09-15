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

## AOCI-derived semantic constraints

- durable responsibilities: <long-term ownership>
- strong relations and invariants: <constraints that must remain true>
- planned semantic delta: <durable semantic change introduced here>
- Maintain timing: <when official Maintain/Guide runs after implementation>
- database condition: <source and Database Cognition gate>
- cognition vs source/test differences: <conflicts and handling>
- unresolved governance items: <unknowns>

## CodeGraph-derived design constraints

- entry points and call paths: <entry points and call paths>
- ownership and dependency boundaries: <ownership and dependency boundaries>
- impact radius: <impact radius>
- affected tests: <affected tests>
- rollback boundary: <rollback boundary>
- graph evidence vs source verification: <difference between graph evidence and source verification>
- unresolved items: <unconfirmed items and handling>
