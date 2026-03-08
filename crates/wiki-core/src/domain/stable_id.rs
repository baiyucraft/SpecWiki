use blake3::Hasher;

/// Repo Wiki 的核心对象需要稳定 ID，不能依赖运行时随机值。
/// 这里统一把“对象类型 + 关键路径/名字”映射成短哈希，方便落到 metadata 和 cache 中。
///
/// # 参数
/// - `kind`：对象类型前缀，例如 `module`、`page`、`source`。
/// - `seed`：用于生成稳定 ID 的关键种子值。
///
/// # 返回
/// - 返回带类型前缀的稳定短 ID。
pub fn stable_id(kind: &str, seed: impl AsRef<str>) -> String {
    let mut hasher = Hasher::new();
    hasher.update(kind.as_bytes());
    hasher.update(b":");
    hasher.update(seed.as_ref().as_bytes());

    let digest = hasher.finalize();
    let short = digest.to_hex().chars().take(12).collect::<String>();
    format!("{kind}-{short}")
}
