use crate::repo::scanner::ScanReport;

pub fn overview_sections(report: &ScanReport) -> Vec<String> {
    let files = report
        .files
        .iter()
        .map(|file| format!("- {}", file.path))
        .collect::<Vec<_>>()
        .join("\n");

    vec![
        "## 简介\n\n由 codebuddy-wiki 自动生成的仓库概览。".to_string(),
        format!("## 项目文件\n\n{}", files),
    ]
}
