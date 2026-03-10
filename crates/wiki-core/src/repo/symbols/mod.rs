//! `symbols` 负责从源码里抽取可检索的符号事实。
//! 这一层独立于页面生成链路存在，供 SQLite / query workflow 复用。

pub mod models;
mod pipeline;
mod queries;
mod registry;

pub use models::{
    ParsedFileSymbols, ParsedSymbolsSnapshot, RawCallCapture, RawHeritageCapture,
    RawImportCapture, SymbolNode, SymbolParseDiagnostic, SymbolTable,
};
pub use pipeline::{
    parse_symbols, parse_symbols_for_paths, CHUNK_BYTE_BUDGET, MAX_FILE_BYTES,
};
pub(crate) use pipeline::{
    parse_symbols_for_paths_with_progress, parse_symbols_with_progress, symbol_parse_file_count,
};
pub use registry::{resolve_symbol_language, supported_symbol_languages};
