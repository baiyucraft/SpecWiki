# CodeBuddy Repo Wiki Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** 构建一个由 Rust core 驱动、通过 `packages/codebuddy` 注入 CodeBuddy、并向目标仓库生成 `.wiki/` 的 Repo Wiki 插件。

**Architecture:** 仓库只保留 `crates/wiki-core` 和 `packages/codebuddy` 两层源码。Rust core 负责扫描、索引、生成、增量更新和查询；CodeBuddy adapter 负责工具注入与调用 Rust 二进制；平台 npm 子包在发布时由脚本临时生成。

**Tech Stack:** Rust, Cargo workspace, Node.js, TypeScript, pnpm workspace, Vitest, child_process JSON IPC

---

### Task 1: Scaffold Workspace

**Files:**
- Create: `Cargo.toml`
- Create: `package.json`
- Create: `pnpm-workspace.yaml`
- Create: `README.md`
- Create: `crates/wiki-core/Cargo.toml`
- Create: `crates/wiki-core/src/lib.rs`
- Create: `crates/wiki-core/src/main.rs`
- Create: `packages/codebuddy/package.json`
- Create: `packages/codebuddy/tsconfig.json`
- Create: `packages/codebuddy/src/index.ts`
- Create: `packages/codebuddy/bin/codebuddy.js`

**Step 1: Write the failing workspace smoke test**

```rust
// crates/wiki-core/tests/workspace_smoke.rs
#[test]
fn binary_name_is_exposed() {
    assert!(option_env!("CARGO_PKG_NAME").is_some());
}
```

```ts
// packages/codebuddy/src/index.test.ts
import { describe, expect, it } from "vitest";

describe("workspace smoke", () => {
  it("loads the adapter entry", async () => {
    const mod = await import("./index");
    expect(mod).toBeTruthy();
  });
});
```

**Step 2: Run test to verify it fails**

Run: `cargo test -p wiki-core workspace_smoke -- --nocapture`
Expected: FAIL because workspace files and tests do not exist yet

Run: `pnpm --dir packages/codebuddy test`
Expected: FAIL because package files and test runner are not configured

**Step 3: Write minimal implementation**

Create a Cargo workspace rooted at `Cargo.toml`:

```toml
[workspace]
members = ["crates/wiki-core"]
resolver = "2"
```

Create a pnpm workspace rooted at `pnpm-workspace.yaml`:

```yaml
packages:
  - "packages/*"
```

Expose a minimal Rust binary and a minimal TypeScript adapter entry.

**Step 4: Run test to verify it passes**

Run: `cargo test -p wiki-core --test workspace_smoke -v`
Expected: PASS

Run: `pnpm install`
Expected: workspace dependencies installed

Run: `pnpm --dir packages/codebuddy test`
Expected: PASS

**Step 5: Commit**

```bash
git add Cargo.toml package.json pnpm-workspace.yaml README.md crates/wiki-core packages/codebuddy
git commit -m "chore: scaffold repo wiki workspace"
```

### Task 2: Define Core Command Contract

**Files:**
- Modify: `crates/wiki-core/src/lib.rs`
- Modify: `crates/wiki-core/src/main.rs`
- Create: `crates/wiki-core/src/transport/dto.rs`
- Create: `crates/wiki-core/src/transport/json_rpc.rs`
- Create: `crates/wiki-core/src/transport/cli.rs`
- Create: `crates/wiki-core/tests/command_contract.rs`

**Step 1: Write the failing test**

```rust
use wiki_core::transport::dto::{CoreCommand, CoreResponse};

#[test]
fn parses_init_command() {
    let cmd: CoreCommand = serde_json::from_str(r#"{"action":"init","repoRoot":"."}"#).unwrap();
    assert_eq!(cmd.action.as_str(), "init");
}

#[test]
fn serializes_error_response() {
    let response = CoreResponse::error("not_git_repo");
    let json = serde_json::to_string(&response).unwrap();
    assert!(json.contains("not_git_repo"));
}
```

**Step 2: Run test to verify it fails**

Run: `cargo test -p wiki-core --test command_contract -v`
Expected: FAIL with missing transport module and DTO types

**Step 3: Write minimal implementation**

Add DTOs for:

- `CoreCommand`
- `CoreResponse`
- `StatusPayload`
- `QueryPayload`

Add a CLI entry that:

- reads JSON from `stdin` when `--json` is present
- dispatches by `action`
- writes JSON response to `stdout`

**Step 4: Run test to verify it passes**

Run: `cargo test -p wiki-core --test command_contract -v`
Expected: PASS

**Step 5: Commit**

```bash
git add crates/wiki-core/src crates/wiki-core/tests/command_contract.rs
git commit -m "feat: add core command transport contract"
```

### Task 3: Model Metadata And Wiki Domain

**Files:**
- Create: `crates/wiki-core/src/domain/wiki_item.rs`
- Create: `crates/wiki-core/src/domain/relation.rs`
- Create: `crates/wiki-core/src/domain/metadata.rs`
- Create: `crates/wiki-core/src/domain/change_set.rs`
- Create: `crates/wiki-core/tests/metadata_roundtrip.rs`

**Step 1: Write the failing test**

```rust
use wiki_core::domain::metadata::WikiMetadata;

#[test]
fn metadata_roundtrip_keeps_dirty_state() {
    let metadata = WikiMetadata::sample();
    let json = serde_json::to_string_pretty(&metadata).unwrap();
    let decoded: WikiMetadata = serde_json::from_str(&json).unwrap();
    assert_eq!(decoded.dirty_state.status, metadata.dirty_state.status);
}
```

**Step 2: Run test to verify it fails**

Run: `cargo test -p wiki-core --test metadata_roundtrip -v`
Expected: FAIL with missing domain types

**Step 3: Write minimal implementation**

Define serializable structs for:

- `WikiItem`
- `WikiRelation`
- `SourceFileRecord`
- `DirtyState`
- `WikiMetadata`
- `ChangeSet`

Include helper constructors for empty metadata and sample metadata used by tests.

**Step 4: Run test to verify it passes**

Run: `cargo test -p wiki-core --test metadata_roundtrip -v`
Expected: PASS

**Step 5: Commit**

```bash
git add crates/wiki-core/src/domain crates/wiki-core/tests/metadata_roundtrip.rs
git commit -m "feat: add wiki metadata domain models"
```

### Task 4: Implement Repository Inspection And Fingerprints

**Files:**
- Create: `crates/wiki-core/src/repo/scanner.rs`
- Create: `crates/wiki-core/src/repo/git.rs`
- Create: `crates/wiki-core/src/repo/fingerprint.rs`
- Create: `crates/wiki-core/src/repo/detectors/mod.rs`
- Create: `crates/wiki-core/tests/repo_scan.rs`

**Step 1: Write the failing test**

```rust
use wiki_core::repo::scanner::scan_repo;

#[test]
fn scan_repo_discovers_files_and_detected_stack() {
    let fixture = std::path::PathBuf::from("tests/fixtures/minimal-node-repo");
    let report = scan_repo(&fixture).unwrap();
    assert!(report.files.iter().any(|f| f.path.ends_with("package.json")));
    assert!(report.detected_topics.iter().any(|x| x == "frontend"));
}
```

**Step 2: Run test to verify it fails**

Run: `cargo test -p wiki-core --test repo_scan -v`
Expected: FAIL with missing scanner implementation and fixtures

**Step 3: Write minimal implementation**

Implement:

- Git repository validation
- Recursive file listing with ignores for `.git`, `.wiki/.cache`, `node_modules`, `target`
- SHA-256 or blake3 file fingerprinting
- Simple tech-stack detectors based on filenames such as `package.json`, `pyproject.toml`, `Cargo.toml`, `Dockerfile`

Add a minimal fixture repository under `crates/wiki-core/tests/fixtures/minimal-node-repo`.

**Step 4: Run test to verify it passes**

Run: `cargo test -p wiki-core --test repo_scan -v`
Expected: PASS

**Step 5: Commit**

```bash
git add crates/wiki-core/src/repo crates/wiki-core/tests
git commit -m "feat: add repository scanning and fingerprinting"
```

### Task 5: Generate Initial Wiki And Metadata

**Files:**
- Create: `crates/wiki-core/src/app/init.rs`
- Create: `crates/wiki-core/src/generation/planner.rs`
- Create: `crates/wiki-core/src/generation/renderer.rs`
- Create: `crates/wiki-core/src/generation/sections.rs`
- Create: `crates/wiki-core/src/storage/wiki_fs.rs`
- Create: `crates/wiki-core/src/storage/metadata_store.rs`
- Create: `crates/wiki-core/src/storage/cache_store.rs`
- Create: `crates/wiki-core/tests/init_generates_wiki.rs`

**Step 1: Write the failing test**

```rust
use std::fs;
use wiki_core::app::init::run_init;

#[test]
fn init_writes_wiki_layout() {
    let fixture = tempfile::tempdir().unwrap();
    let repo_root = fixture.path();
    fs::create_dir(repo_root.join(".git")).unwrap();
    fs::write(repo_root.join("package.json"), r#"{"name":"demo"}"#).unwrap();

    run_init(repo_root).unwrap();

    assert!(repo_root.join(".wiki/项目概述.md").exists());
    assert!(repo_root.join(".wiki/wiki.metadata.json").exists());
    assert!(repo_root.join(".wiki/.cache").exists());
}
```

**Step 2: Run test to verify it fails**

Run: `cargo test -p wiki-core --test init_generates_wiki -v`
Expected: FAIL with missing init/generation/storage implementation

**Step 3: Write minimal implementation**

Implement:

- page planner that always creates `项目概述.md`
- optional page groups based on detected stack
- markdown renderer with managed section markers
- filesystem writer for `.wiki/`
- metadata writer for `wiki.metadata.json`
- cache directory initialization for `.wiki/.cache/`

**Step 4: Run test to verify it passes**

Run: `cargo test -p wiki-core --test init_generates_wiki -v`
Expected: PASS

**Step 5: Commit**

```bash
git add crates/wiki-core/src/app/init.rs crates/wiki-core/src/generation crates/wiki-core/src/storage crates/wiki-core/tests/init_generates_wiki.rs
git commit -m "feat: generate initial repo wiki output"
```

### Task 6: Add Status And Incremental Update

**Files:**
- Create: `crates/wiki-core/src/app/status.rs`
- Create: `crates/wiki-core/src/app/update.rs`
- Create: `crates/wiki-core/tests/status_and_update.rs`

**Step 1: Write the failing test**

```rust
use std::fs;
use wiki_core::app::{init::run_init, status::run_status, update::run_update};

#[test]
fn update_only_marks_changed_pages_dirty() {
    let fixture = tempfile::tempdir().unwrap();
    let repo_root = fixture.path();
    fs::create_dir(repo_root.join(".git")).unwrap();
    fs::write(repo_root.join("package.json"), r#"{"name":"demo"}"#).unwrap();
    fs::write(repo_root.join("src.ts"), "export const a = 1;").unwrap();

    run_init(repo_root).unwrap();
    fs::write(repo_root.join("src.ts"), "export const a = 2;").unwrap();

    let status = run_status(repo_root).unwrap();
    assert_eq!(status.state, "stale");

    let update = run_update(repo_root, None).unwrap();
    assert!(!update.updated_pages.is_empty());
}
```

**Step 2: Run test to verify it fails**

Run: `cargo test -p wiki-core --test status_and_update -v`
Expected: FAIL with missing status/update implementation

**Step 3: Write minimal implementation**

Implement:

- `status` by comparing current file fingerprints to metadata snapshots
- dirty source calculation
- dirty page backtracking from `source_files -> wiki_items`
- selective page rewrite and metadata refresh

**Step 4: Run test to verify it passes**

Run: `cargo test -p wiki-core --test status_and_update -v`
Expected: PASS

**Step 5: Commit**

```bash
git add crates/wiki-core/src/app/status.rs crates/wiki-core/src/app/update.rs crates/wiki-core/tests/status_and_update.rs
git commit -m "feat: add wiki status and incremental update"
```

### Task 7: Add Query, Sync, And Rebuild

**Files:**
- Create: `crates/wiki-core/src/app/query.rs`
- Create: `crates/wiki-core/src/app/sync.rs`
- Create: `crates/wiki-core/src/app/rebuild.rs`
- Create: `crates/wiki-core/src/sync/markdown_patch.rs`
- Create: `crates/wiki-core/tests/query_sync_rebuild.rs`

**Step 1: Write the failing test**

```rust
use std::fs;
use wiki_core::app::{init::run_init, query::run_query, rebuild::run_rebuild, sync::run_sync};

#[test]
fn sync_detects_manual_markdown_changes() {
    let fixture = tempfile::tempdir().unwrap();
    let repo_root = fixture.path();
    fs::create_dir(repo_root.join(".git")).unwrap();
    fs::write(repo_root.join("package.json"), r#"{"name":"demo"}"#).unwrap();
    run_init(repo_root).unwrap();

    let overview = repo_root.join(".wiki/项目概述.md");
    fs::write(&overview, "# 项目概述\n\n自定义说明\n").unwrap();

    let result = run_sync(repo_root).unwrap();
    assert!(result.synced_pages.iter().any(|x| x.ends_with("项目概述.md")));

    let query = run_query(repo_root, "项目概述").unwrap();
    assert!(!query.matched_pages.is_empty());

    let rebuild = run_rebuild(repo_root).unwrap();
    assert!(!rebuild.updated_pages.is_empty());
}
```

**Step 2: Run test to verify it fails**

Run: `cargo test -p wiki-core --test query_sync_rebuild -v`
Expected: FAIL with missing query/sync/rebuild implementation

**Step 3: Write minimal implementation**

Implement:

- topic matching over page titles, summaries, and source references
- markdown sync that re-hashes user-edited files
- rebuild that clears cache and reruns planning/rendering

**Step 4: Run test to verify it passes**

Run: `cargo test -p wiki-core --test query_sync_rebuild -v`
Expected: PASS

**Step 5: Commit**

```bash
git add crates/wiki-core/src/app/query.rs crates/wiki-core/src/app/sync.rs crates/wiki-core/src/app/rebuild.rs crates/wiki-core/src/sync crates/wiki-core/tests/query_sync_rebuild.rs
git commit -m "feat: add wiki query sync and rebuild"
```

### Task 8: Build CodeBuddy Adapter

**Files:**
- Modify: `packages/codebuddy/src/index.ts`
- Create: `packages/codebuddy/src/runtime/resolveBinary.ts`
- Create: `packages/codebuddy/src/runtime/invokeCore.ts`
- Create: `packages/codebuddy/src/runtime/parseResult.ts`
- Create: `packages/codebuddy/src/tools/wikiInit.ts`
- Create: `packages/codebuddy/src/tools/wikiStatus.ts`
- Create: `packages/codebuddy/src/tools/wikiUpdate.ts`
- Create: `packages/codebuddy/src/tools/wikiQuery.ts`
- Create: `packages/codebuddy/src/tools/wikiSync.ts`
- Create: `packages/codebuddy/src/tools/wikiRebuild.ts`
- Create: `packages/codebuddy/src/index.test.ts`

**Step 1: Write the failing test**

```ts
import { describe, expect, it, vi } from "vitest";
import { invokeCore } from "./runtime/invokeCore";

vi.mock("./runtime/invokeCore", () => ({
  invokeCore: vi.fn().mockResolvedValue({ ok: true, action: "status" }),
}));

describe("adapter", () => {
  it("exposes wiki status tool", async () => {
    const mod = await import("./index");
    expect(mod.tools.wikiStatus).toBeTypeOf("function");
    await mod.tools.wikiStatus({ repoRoot: "." });
    expect(invokeCore).toHaveBeenCalled();
  });
});
```

**Step 2: Run test to verify it fails**

Run: `pnpm --dir packages/codebuddy test`
Expected: FAIL because adapter runtime and tools are missing

**Step 3: Write minimal implementation**

Implement:

- binary resolution for current platform
- JSON request/response wrapper around spawned Rust process
- six CodeBuddy-facing tool functions
- a single adapter export that groups these tools

**Step 4: Run test to verify it passes**

Run: `pnpm --dir packages/codebuddy test`
Expected: PASS

**Step 5: Commit**

```bash
git add packages/codebuddy/src packages/codebuddy/bin/codebuddy.js
git commit -m "feat: add codebuddy adapter for wiki core"
```

### Task 9: Add Packaging And Publish Scripts

**Files:**
- Create: `scripts/build-core.mjs`
- Create: `scripts/stage-binaries.mjs`
- Create: `scripts/publish-npm.mjs`
- Modify: `packages/codebuddy/package.json`
- Modify: `package.json`
- Create: `packages/codebuddy/templates/platform-package.json`
- Create: `tests/integration/package_staging.test.mjs`

**Step 1: Write the failing test**

```js
import { test } from "node:test";
import assert from "node:assert/strict";
import { existsSync } from "node:fs";

test("staging script creates platform package manifest", async () => {
  assert.equal(existsSync("dist/npm"), false);
});
```

**Step 2: Run test to verify it fails**

Run: `node --test tests/integration/package_staging.test.mjs`
Expected: FAIL after script work is wired because staging output is not produced yet

**Step 3: Write minimal implementation**

Implement scripts to:

- build release binaries into `target/release`
- copy selected binary into temporary `dist/npm/<platform-package>`
- generate `package.json` manifests from templates
- wire `optionalDependencies` from main package to platform packages

**Step 4: Run test to verify it passes**

Run: `node --test tests/integration/package_staging.test.mjs`
Expected: PASS

**Step 5: Commit**

```bash
git add scripts packages/codebuddy/templates tests/integration package.json
git commit -m "chore: add npm packaging and staging scripts"
```

### Task 10: Add End-To-End Verification And Docs

**Files:**
- Create: `tests/e2e/init-update-query.test.mjs`
- Modify: `README.md`
- Modify: `packages/codebuddy/package.json`
- Modify: `crates/wiki-core/Cargo.toml`

**Step 1: Write the failing test**

```js
import test from "node:test";
import assert from "node:assert/strict";

test("e2e flow initializes and queries wiki", async () => {
  assert.ok(false, "implement e2e harness");
});
```

**Step 2: Run test to verify it fails**

Run: `node --test tests/e2e/init-update-query.test.mjs`
Expected: FAIL with placeholder assertion

**Step 3: Write minimal implementation**

Implement an end-to-end harness that:

- creates a temp Git-like fixture repo
- runs the packaged adapter against Rust core
- verifies `.wiki/` output
- edits one source file
- runs update
- runs query and checks returned page matches

Update `README.md` with:

- install steps
- local dev steps
- how `.wiki/` works
- how to publish

**Step 4: Run test to verify it passes**

Run: `cargo test -p wiki-core -v`
Expected: PASS

Run: `pnpm --dir packages/codebuddy test`
Expected: PASS

Run: `node --test tests/e2e/init-update-query.test.mjs`
Expected: PASS

**Step 5: Commit**

```bash
git add README.md tests/e2e packages/codebuddy/package.json crates/wiki-core/Cargo.toml
git commit -m "test: add end-to-end verification for repo wiki"
```
