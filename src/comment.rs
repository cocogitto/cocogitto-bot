use crate::model::report::CommitReport;
use indoc::formatdoc;

pub fn build_comment_failure(reports: Vec<CommitReport>) -> String {
    // should be ok to unwrap since we build this only we at least one commit is errored
    let start = reports.first().unwrap().get_sha();
    let end = reports.last().unwrap().get_sha();

    let range = format!("{}...{}", start, end);

    let success_commit_count = reports
        .iter()
        .filter(|report| matches!(report, CommitReport::Success(_)))
        .count();

    let error_reports = reports
        .iter()
        .filter_map(|report| {
            if let CommitReport::Error(error_report) = report {
                Some(error_report.to_string())
            } else {
                None
            }
        })
        .collect::<Vec<String>>();
    let error_count = error_reports.len();
    let error_reports = error_reports.join("\n");

    formatdoc!(
        "Found {} compliant commit and {} non-compliant commits in {}.

        {}",
        success_commit_count,
        error_count,
        range,
        error_reports
    )
}

pub fn build_comment_success(reports: Vec<CommitReport>) -> String {
    let report_list = reports
        .iter()
        .map(|report| format!("- {} :heavy_check_mark:", &report.get_sha()[0..7]))
        .collect::<Vec<String>>()
        .join("\n");

    formatdoc!(
        "Conventional commits check succeeded :
                {}",
        report_list
    )
}
