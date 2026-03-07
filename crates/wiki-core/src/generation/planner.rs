use crate::repo::scanner::ScanReport;

#[derive(Debug, Clone)]
pub struct PlannedPage {
    pub id: String,
    pub title: String,
    pub relative_path: String,
    pub page_type: String,
}

pub fn plan_pages(report: &ScanReport) -> Vec<PlannedPage> {
    let mut pages = vec![PlannedPage {
        id: "overview".to_string(),
        title: "项目概述".to_string(),
        relative_path: "项目概述.md".to_string(),
        page_type: "overview".to_string(),
    }];

    if report.detected_topics.iter().any(|topic| topic == "frontend") {
        pages.push(PlannedPage {
            id: "frontend-guide".to_string(),
            title: "前端开发指南".to_string(),
            relative_path: "前端开发指南/前端开发指南.md".to_string(),
            page_type: "guide".to_string(),
        });
    }

    pages
}
