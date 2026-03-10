use std::path::Path;

use tree_sitter::Language;

use super::queries::{
    CSHARP_QUERY, CPP_QUERY, C_QUERY, GO_QUERY, JAVASCRIPT_QUERY, JAVA_QUERY, KOTLIN_QUERY,
    PHP_QUERY, PYTHON_QUERY, RUST_QUERY, SWIFT_QUERY, TYPESCRIPT_QUERY,
};

/// 符号解析当前支持的语言标签。
pub fn supported_symbol_languages() -> &'static [&'static str] {
    &[
        "c",
        "cpp",
        "csharp",
        "go",
        "java",
        "javascript",
        "kotlin",
        "php",
        "python",
        "react",
        "rust",
        "svelte",
        "swift",
        "typescript",
        "vue",
    ]
}

/// 解析 registry 后得到的实际 grammar 和 query 绑定。
#[derive(Clone, Copy)]
pub struct ResolvedSymbolLanguage {
    /// 当前文件最终采用的语言标签；React 之类包装语言会在这里降到 JS/TS。
    pub effective_language: &'static str,
    /// 与 grammar 配套的 definition query 源码。
    pub query_source: &'static str,
    language_fn: fn() -> Language,
}

impl ResolvedSymbolLanguage {
    /// 延迟构造 tree-sitter `Language`，避免 registry 常量在模块初始化时做重工作。
    pub fn language(&self) -> Language {
        (self.language_fn)()
    }
}

/// 按扫描语言和文件扩展解析真正要用的 tree-sitter grammar。
pub fn resolve_symbol_language(
    file_path: &str,
    scan_language: &str,
) -> Option<ResolvedSymbolLanguage> {
    match scan_language {
        "c" => Some(spec("c", c_language, C_QUERY)),
        "cpp" => Some(spec("cpp", cpp_language, CPP_QUERY)),
        "csharp" => Some(spec("csharp", csharp_language, CSHARP_QUERY)),
        "go" => Some(spec("go", go_language, GO_QUERY)),
        "java" => Some(spec("java", java_language, JAVA_QUERY)),
        "javascript" => Some(spec("javascript", javascript_language, JAVASCRIPT_QUERY)),
        "kotlin" => Some(spec("kotlin", kotlin_language, KOTLIN_QUERY)),
        "php" => Some(spec("php", php_language, PHP_QUERY)),
        "python" => Some(spec("python", python_language, PYTHON_QUERY)),
        "rust" => Some(spec("rust", rust_language, RUST_QUERY)),
        "svelte" => resolve_embedded_language("javascript"),
        "swift" => Some(spec("swift", swift_language, SWIFT_QUERY)),
        "typescript" => Some(spec("typescript", typescript_language, TYPESCRIPT_QUERY)),
        "react" => resolve_react_language(file_path),
        "vue" => resolve_embedded_language("javascript"),
        _ => None,
    }
}

fn resolve_react_language(file_path: &str) -> Option<ResolvedSymbolLanguage> {
    match Path::new(file_path)
        .extension()
        .and_then(|extension| extension.to_str())
    {
        Some("tsx") => Some(spec("typescript", tsx_language, TYPESCRIPT_QUERY)),
        Some("jsx") => Some(spec("javascript", javascript_language, JAVASCRIPT_QUERY)),
        _ => None,
    }
}

/// 包装语言默认先落到最保守的底层 JS parser；
/// Vue/Svelte 的 `lang` 细分会在 pipeline 的虚拟脚本切片阶段再二次收口。
pub fn resolve_embedded_language(effective_language: &str) -> Option<ResolvedSymbolLanguage> {
    match effective_language {
        "javascript" => Some(spec("javascript", javascript_language, JAVASCRIPT_QUERY)),
        "typescript" => Some(spec("typescript", typescript_language, TYPESCRIPT_QUERY)),
        "tsx" => Some(spec("typescript", tsx_language, TYPESCRIPT_QUERY)),
        _ => None,
    }
}

fn spec(
    effective_language: &'static str,
    language_fn: fn() -> Language,
    query_source: &'static str,
) -> ResolvedSymbolLanguage {
    ResolvedSymbolLanguage {
        effective_language,
        query_source,
        language_fn,
    }
}

fn c_language() -> Language {
    tree_sitter_c::LANGUAGE.into()
}

fn cpp_language() -> Language {
    tree_sitter_cpp::LANGUAGE.into()
}

fn csharp_language() -> Language {
    tree_sitter_c_sharp::LANGUAGE.into()
}

fn go_language() -> Language {
    tree_sitter_go::LANGUAGE.into()
}

fn java_language() -> Language {
    tree_sitter_java::LANGUAGE.into()
}

fn javascript_language() -> Language {
    tree_sitter_javascript::LANGUAGE.into()
}

fn kotlin_language() -> Language {
    tree_sitter_kotlin_ng::LANGUAGE.into()
}

fn php_language() -> Language {
    tree_sitter_php::LANGUAGE_PHP.into()
}

fn python_language() -> Language {
    tree_sitter_python::LANGUAGE.into()
}

fn rust_language() -> Language {
    tree_sitter_rust::LANGUAGE.into()
}

fn swift_language() -> Language {
    tree_sitter_swift::LANGUAGE.into()
}

fn typescript_language() -> Language {
    tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into()
}

fn tsx_language() -> Language {
    tree_sitter_typescript::LANGUAGE_TSX.into()
}

#[cfg(test)]
mod tests {
    use super::{resolve_embedded_language, resolve_symbol_language, supported_symbol_languages};

    #[test]
    fn registry_resolves_every_supported_language() {
        let samples = [
            ("src/a.c", "c"),
            ("src/a.cpp", "cpp"),
            ("src/A.cs", "csharp"),
            ("src/main.go", "go"),
            ("src/Main.java", "java"),
            ("src/index.js", "javascript"),
            ("src/App.kt", "kotlin"),
            ("src/index.php", "php"),
            ("src/main.py", "python"),
            ("src/App.tsx", "react"),
            ("src/App.svelte", "svelte"),
            ("src/lib.rs", "rust"),
            ("src/App.swift", "swift"),
            ("src/index.ts", "typescript"),
            ("src/App.vue", "vue"),
        ];

        for (file_path, language) in samples {
            let resolved = resolve_symbol_language(file_path, language);
            assert!(
                resolved.is_some(),
                "expected registry support for {language} ({file_path})"
            );
        }

        assert!(supported_symbol_languages().contains(&"react"));
        assert!(supported_symbol_languages().contains(&"vue"));
        assert!(supported_symbol_languages().contains(&"svelte"));
        assert!(resolve_embedded_language("typescript").is_some());
    }
}
