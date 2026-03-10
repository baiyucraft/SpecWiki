# wiki-core tests

当前 `tests/` 目录按主题分成 5 个 integration suites：

- `acceptance.rs`
  - CLI 合约、基础 acceptance、`init` 产物检查
- `hierarchy.rs`
  - module tree、hierarchy planning、page topology、merge strategy
- `repo.rs`
  - repo scan、language parsing、noise filter、steering
- `runtime.rs`
  - runtime/SQLite、query/sync/update/rebuild、markdown merge、metadata
- `symbols.rs`
  - symbol parsing、symbol resolution、symbol graph analysis

真实测试文件已经移到对应子目录下，顶层只保留 suite 入口，方便继续扩展而不让根目录失控增长。
