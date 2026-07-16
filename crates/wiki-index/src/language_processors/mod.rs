//! `language_processors` 负责把“不同语言的依赖提取规则”统一收口。
//! 它借鉴 `deepwiki-rs` 的做法：先建立按语言分发的 processor，再把提取结果交给上层模块树与页面规划使用。

use regex::Regex;

mod csharp;
mod java;
mod javascript;
mod kotlin;
mod php;
mod python;
mod react;
mod rust;
mod svelte;
mod swift;
mod typescript;
mod vue;

/// 每种语言处理器只负责两件事：
/// - 提取“这个文件依赖了谁”
/// - 提取“这个文件声明了哪些内部别名/命名空间”
pub trait LanguageProcessor: Send + Sync + std::fmt::Debug {
    /// 返回处理器支持的语言标签。
    fn supported_languages(&self) -> &'static [&'static str];

    /// 从源码文本里提取依赖目标。
    ///
    /// # 参数
    /// - `content`：待分析的源码文本。
    ///
    /// # 返回
    /// - 返回当前文件中出现的原始依赖目标列表。
    fn extract_dependency_targets(&self, content: &str) -> Vec<String>;

    /// 从源码文本里提取声明出来的内部别名，例如 package / namespace。
    ///
    /// # 参数
    /// - `content`：待分析的源码文本。
    ///
    /// # 返回
    /// - 返回当前文件声明出的内部别名列表；默认不声明任何别名。
    fn extract_declared_aliases(&self, _content: &str) -> Vec<String> {
        Vec::new()
    }
}

/// `LanguageProcessorManager` 是扫描层统一的语言入口。
/// 上层只管给它语言标签和源码文本，不再关心具体是哪种解析实现。
#[derive(Debug)]
pub struct LanguageProcessorManager {
    processors: Vec<Box<dyn LanguageProcessor>>,
}

impl Default for LanguageProcessorManager {
    fn default() -> Self {
        Self::new()
    }
}

impl LanguageProcessorManager {
    /// 构建默认语言处理器集合。
    ///
    /// # 返回
    /// - 返回当前仓库默认启用的语言处理器管理器。
    pub fn new() -> Self {
        Self {
            processors: vec![
                Box::new(rust::RustProcessor::new()),
                Box::new(javascript::JavaScriptProcessor::new()),
                Box::new(typescript::TypeScriptProcessor::new()),
                Box::new(python::PythonProcessor::new()),
                Box::new(java::JavaProcessor::new()),
                Box::new(csharp::CSharpProcessor::new()),
                Box::new(kotlin::KotlinProcessor::new()),
                Box::new(php::PhpProcessor::new()),
                Box::new(swift::SwiftProcessor::new()),
                Box::new(react::ReactProcessor::new()),
                Box::new(vue::VueProcessor::new()),
                Box::new(svelte::SvelteProcessor::new()),
            ],
        }
    }

    /// 提取某种语言的依赖目标。
    ///
    /// # 参数
    /// - `language`：当前文件语言标签。
    /// - `content`：当前文件源码文本。
    ///
    /// # 返回
    /// - 返回该语言处理器提取出的原始依赖目标列表；不支持时返回空数组。
    pub fn extract_dependency_targets(&self, language: &str, content: &str) -> Vec<String> {
        self.find_processor(language)
            .map(|processor| processor.extract_dependency_targets(content))
            .unwrap_or_default()
    }

    /// 提取某种语言在源码里声明的别名。
    ///
    /// # 参数
    /// - `language`：当前文件语言标签。
    /// - `content`：当前文件源码文本。
    ///
    /// # 返回
    /// - 返回该文件声明的内部别名列表；不支持时返回空数组。
    pub fn extract_declared_aliases(&self, language: &str, content: &str) -> Vec<String> {
        self.find_processor(language)
            .map(|processor| processor.extract_declared_aliases(content))
            .unwrap_or_default()
    }

    /// 按语言标签查找对应处理器。
    /// 这里统一封装匹配逻辑，避免上层代码关心“某种语言有没有专门处理器”。
    fn find_processor(&self, language: &str) -> Option<&dyn LanguageProcessor> {
        self.processors
            .iter()
            .find(|processor| processor.supported_languages().contains(&language))
            .map(|processor| processor.as_ref())
    }
}

/// 提取单行字符串里的第一个引号目标。
///
/// # 参数
/// - `line`：包含 import/require/include 的源码片段。
///
/// # 返回
/// - 如果行内存在引号目标，则返回目标字符串。
pub(crate) fn extract_first_quoted_target(line: &str) -> Option<String> {
    let quote_index = line.find(['"', '\''])?;
    let quote = line.as_bytes().get(quote_index).copied()? as char;
    let start = quote_index + 1;
    let end = line[start..].find(quote)? + start;
    Some(line[start..end].to_string())
}

/// 从文本中抽取所有匹配正则的目标值。
///
/// # 参数
/// - `content`：待匹配的源码文本。
/// - `regex`：用于提取目标值的正则。
///
/// # 返回
/// - 返回第一个捕获组对应的所有结果。
pub(crate) fn collect_regex_targets(content: &str, regex: &Regex) -> Vec<String> {
    regex
        .captures_iter(content)
        .filter_map(|captures| {
            captures
                .get(1)
                .map(|capture| capture.as_str().trim().to_string())
        })
        .filter(|target| !target.is_empty())
        .collect()
}

/// 从源码文本里提取 `<script>...</script>` 片段。
/// 这主要给 Vue / Svelte 这类单文件组件复用。
///
/// # 参数
/// - `content`：完整组件源码文本。
///
/// # 返回
/// - 返回全部脚本片段内容；没有脚本时返回空数组。
pub(crate) fn extract_script_blocks(content: &str) -> Vec<String> {
    let script_regex = Regex::new(r"(?s)<script[^>]*>(.*?)</script>").unwrap();
    script_regex
        .captures_iter(content)
        .filter_map(|captures| captures.get(1).map(|capture| capture.as_str().to_string()))
        .collect()
}
