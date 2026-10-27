use crate::reporting::metrics::TestSuite;
use anyhow::Result;
use html_escape::encode_text;
use std::fs::File;
use std::io::Write;

pub struct HtmlReporter;

impl HtmlReporter {
    pub fn write(suite: &TestSuite, path: &str) -> Result<()> {
        let mut html = String::from(r#"<!DOCTYPE html>
<html lang=\"en\">
<head>
<meta charset=\"UTF-8\">
<title>Restful Booker Test Report</title>
<style>
body { font-family: system-ui, sans-serif; background:#0f172a; color:#e2e8f0; margin:2rem; }
h1 { color:#38bdf8; }
table { width:100%; border-collapse:collapse; }
th, td { border:1px solid #334155; padding:0.5rem 1rem; text-align:left; }
th { background:#1e293b; }
.pass { color:#4ade80; }
.fail { color:#f87171; }
.summary { margin-bottom:1.5rem; }
</style>
</head>
<body>
"#);
        html.push_str(&format!("<h1>{}</h1>", encode_text(&suite.name)));
        html.push_str(&format!(
            "<div class=\"summary\">Started: {} | Passed: {} | Failed: {}</div>",
            suite.started_at,
            suite.passed_count(),
            suite.failed_count()
        ));
        html.push_str("<table><thead><tr><th>Name</th><th>Status</th><th>Message</th></tr></thead><tbody>");
        for r in &suite.results {
            let status = if r.passed { "pass" } else { "fail" };
            let label = if r.passed { "PASS" } else { "FAIL" };
            html.push_str(&format!(
                "<tr><td>{}</td><td class=\"{}\">{}</td><td>{}</td></tr>",
                encode_text(&r.name),
                status,
                label,
                encode_text(&r.message)
            ));
        }
        html.push_str("</tbody></table></body></html>");
        let mut file = File::create(path)?;
        file.write_all(html.as_bytes())?;
        Ok(())
    }
}
