use crate::generation::planner::PlannedPage;
use crate::generation::sections::overview_sections;
use crate::repo::scanner::ScanReport;

pub fn render_page(page: &PlannedPage, report: &ScanReport) -> String {
    let mut lines = vec![format!("# {}", page.title)];

    if page.id == "overview" {
        lines.extend(overview_sections(report));
    } else {
        lines.push("## 简介\n\n由 codebuddy-wiki 自动生成。".to_string());
    }

    lines.join("\n\n")
}
