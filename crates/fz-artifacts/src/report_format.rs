pub fn format_target_section(report: &fz_core::CampaignReport, lines: &mut Vec<String>) {
    lines.push("## Target".to_string());
    lines.push(String::new());
    lines.push(format!("- Name: {}", report.target_name));
    lines.push(format!("- Kind: {}", report.target_kind));
    lines.push(format!("- Entry: {}", report.target_entry));
    lines.push(format!("- Timeout: {}ms", report.timeout_ms));
    lines.push(String::new());
}

pub fn format_summary_section(report: &fz_core::CampaignReport, lines: &mut Vec<String>) {
    lines.push("## Configuration".to_string());
    lines.push(String::new());
    lines.push(format!("- Total budget: {}", report.total_budget));
    lines.push(format!("- Total executions: {}", report.total_executions));
    lines.push(String::new());
    lines.push("## Summary".to_string());
    lines.push(String::new());
    lines.push(format!("- Crashes: {}", report.crash_count));
    lines.push(format!("- Panics: {}", report.panic_count));
    lines.push(format!("- Hangs: {}", report.hang_count));
    lines.push(format!("- Unique failures: {}", report.unique_failures));
    lines.push(format!("- Promoted to tests: {}", report.promoted_count));
    if !report.promoted_dir.is_empty() {
        lines.push(format!("- Test output: {}", report.promoted_dir));
    }
    lines.push(String::new());
}

pub fn format_layer_table(report: &fz_core::CampaignReport, lines: &mut Vec<String>) {
    lines.push("## Layer Breakdown".to_string());
    lines.push(String::new());
    lines.push("| Layer | Executions | New Findings |".to_string());
    lines.push("|-------|------------|-------------|".to_string());
    lines.push(format!(
        "| Baseline | {} | {} |",
        report.baseline_stats.executions, report.baseline_stats.new_findings
    ));
    lines.push(format!(
        "| LLM | {} | {} |",
        report.llm_stats.executions, report.llm_stats.new_findings
    ));
    lines.push(format!(
        "| Mutation | {} | {} |",
        report.mutation_stats.executions, report.mutation_stats.new_findings
    ));
    lines.push(format!(
        "| Feedback | {} | {} |",
        report.feedback_stats.executions, report.feedback_stats.new_findings
    ));
    lines.push(String::new());
}

pub fn format_findings_section(report: &fz_core::CampaignReport, lines: &mut Vec<String>) {
    if report.findings.is_empty() {
        return;
    }
    lines.push("## Findings".to_string());
    lines.push(String::new());
    for (i, finding) in report.findings.iter().enumerate() {
        let preview = String::from_utf8_lossy(&finding.input);
        let truncated = if preview.len() > 80 {
            format!("{}...", &preview[..80])
        } else {
            preview.to_string()
        };
        let timestamp = if finding.discovered_at.is_empty() {
            String::from("N/A")
        } else {
            finding.discovered_at.clone()
        };
        lines.push(format!(
            "{}. [{:?}] {} (via {:?}, {})",
            i + 1,
            finding.classification,
            truncated,
            finding.provenance,
            timestamp
        ));
    }
}

pub fn format_recommendations(report: &fz_core::CampaignReport, lines: &mut Vec<String>) {
    if report.unique_failures == 0 {
        return;
    }
    lines.push(String::new());
    lines.push("## Recommendations".to_string());
    lines.push(String::new());
    lines.push(format!(
        "- {} crash(es) found, {} promoted to regression tests",
        report.unique_failures, report.promoted_count
    ));
    if report.mutation_stats.new_findings == 0 && report.baseline_stats.executions > 0 {
        lines.push("- Consider increasing mutation budget for deeper exploration".to_string());
    }
    if report.llm_stats.executions == 0 {
        lines.push("- Enable Ollama for LLM-generated seed diversity".to_string());
    }
}

pub fn format_report(report: &fz_core::CampaignReport) -> String {
    let mut lines = vec![
        format!("# Fuzz Campaign Report: {}", report.target_name),
        String::new(),
    ];
    format_target_section(report, &mut lines);
    format_summary_section(report, &mut lines);
    format_layer_table(report, &mut lines);
    format_findings_section(report, &mut lines);
    format_recommendations(report, &mut lines);
    lines.join("\n")
}
