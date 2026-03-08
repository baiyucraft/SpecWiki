/// 统一计算字节内容的稳定哈希。
/// 指纹同时用于源码变化检测和页面内容哈希，避免各处自己选不同算法。
///
/// # 参数
/// - `bytes`：要计算哈希的原始字节内容。
///
/// # 返回
/// - 返回十六进制字符串形式的内容指纹。
pub fn fingerprint_bytes(bytes: &[u8]) -> String {
    blake3::hash(bytes).to_hex().to_string()
}
